// ──────────────────────────────────────────────────────────────────────
// File:        smoketest/rust/conformance/responder.rs
// Origin:      Hand-written for asyncapi-clients conformance testing
//
// Author:      Christopher Kettell, TRL Software
// Date:        2026-08-28
// Version:     1.0
// License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
// ──────────────────────────────────────────────────────────────────────
//
//! A11 command responder — Rust half of the cross-language matrix.
//!
//! The mirror image of S3: consume `vms.command`, read the requester's
//! `reply-to`, and answer *that node* with `correlation-id` mirroring the
//! request's `message-id` and `commandCorrelationId` carried through
//! unchanged. Every requester asserts exactly those three things.
//!
//! The sender uses an anonymous terminus — a target with no address — so
//! one link answers every requester's reply node, with each message's
//! `to` doing the routing (A11.4). Rust is the client where the
//! identifier is echoed most directly: `MessageId` is carried from
//! request to response as a value, with no rendering step in between.
//!
//! Configuration mirrors vms_responder.py so the harness can drive any
//! language's responder without knowing which it is.

use std::env;
use std::time::Duration;

use fe2o3_amqp::types::messaging::{Body, Message, Properties, Target};
use fe2o3_amqp::{Connection, Receiver, Sender, Session};
use serde_json::{json, Value};

const COMMAND_CHANNEL: &str = "vms.command";

fn body_as_string(body: &Body<Value>) -> Option<String> {
    match body {
        Body::Data(batch) => {
            let mut out = String::new();
            for data in batch.iter() {
                out.push_str(&String::from_utf8_lossy(data.0.as_ref()));
            }
            Some(out)
        }
        Body::Value(v) => match &v.0 {
            Value::String(s) => Some(s.clone()),
            _ => None,
        },
        Body::Sequence(_) | Body::Empty => None,
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = env::var("TOPAS_AMQP_URL").unwrap_or_else(|_| "amqp://localhost:5672".into());
    let progression: Vec<String> = env::var("TOPAS_RESPONDER_PROGRESSION")
        .unwrap_or_else(|_| "pending,pending,success".into())
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    let delay: f64 = env::var("TOPAS_RESPONDER_DELAY")
        .ok().and_then(|v| v.parse().ok()).unwrap_or(0.15);
    let max_replies: Option<usize> = env::var("TOPAS_RESPONDER_MAX_REPLIES")
        .ok().and_then(|v| v.parse().ok());

    let mut connection = Connection::open("topas-responder", url.as_str()).await?;
    let mut session = Session::begin(&mut connection).await?;

    let source = fe2o3_amqp::types::messaging::Source::builder()
        .address(COMMAND_CHANNEL)
        .capabilities(vec![fe2o3_amqp::types::primitives::Symbol::from("topic")])
        .build();
    let mut receiver = Receiver::builder()
        .name("responder-commands")
        .source(source)
        .attach(&mut session)
        .await?;

    // Anonymous terminus: a target with no address. Each message's `to`
    // routes it, so one link serves every reply node (A11.4). Brokers
    // advertise this as ANONYMOUS-RELAY; A11 does not require it, and a
    // responder may instead cache an addressed sender per reply node.
    let mut sender = Sender::builder()
        .name("responder-replies")
        .target(Target::builder().build())
        .attach(&mut session)
        .await?;

    // Readiness handshake, identical across languages.
    println!("[responder-ready] rust (anonymous terminus)");

    let mut replied = 0usize;
    loop {
        if let Some(max) = max_replies {
            if replied >= max {
                break;
            }
        }
        let delivery = match tokio::time::timeout(
            Duration::from_secs(30), receiver.recv::<Body<Value>>()).await
        {
            Ok(Ok(d)) => d,
            Ok(Err(e)) => {
                eprintln!("[responder] receive failed: {e}");
                break;
            }
            Err(_) => {
                eprintln!("[responder] no command within 30s — exiting");
                break;
            }
        };

        let props = delivery.message().properties.as_ref();
        let reply_to = props.and_then(|p| p.reply_to.clone());
        // A11: the response is echoed back against the request's
        // message-id, as a MessageId value rather than a rendering of one.
        let request_message_id = props.and_then(|p| p.message_id.clone());

        let reply_to = match reply_to {
            Some(a) if !a.is_empty() => a,
            _ => {
                // Under A11 there is nowhere to address the response.
                // Loud, and does not answer.
                eprintln!("[responder] FATAL: command carried no reply-to; under A11 \
                           there is nowhere to address the response");
                let _ = receiver.accept(&delivery).await;
                break;
            }
        };

        // Decoding does not degrade: an unrecognised body is a visible
        // failure rather than a placeholder.
        let raw = match body_as_string(delivery.body()) {
            Some(s) => s,
            None => {
                eprintln!("[responder] FATAL: unhandled body representation");
                let _ = receiver.accept(&delivery).await;
                break;
            }
        };
        let command: Value = serde_json::from_str(&raw)?;
        let app_id = command.get("commandCorrelationId").cloned();
        let _ = receiver.accept(&delivery).await;

        for (idx, status) in progression.iter().enumerate() {
            let frame = json!({
                "vmsId": command.get("vmsId").cloned().unwrap_or(Value::Null),
                // Carried through unchanged, at the top level rather than
                // buried in an echo.
                "commandCorrelationId": app_id.clone().unwrap_or(Value::Null),
                "status": status,
                "responseTime": "1970-01-01T00:00:00Z",
                "echo": command.clone(),
            });
            let mut p = Properties::default();
            p.correlation_id = request_message_id.clone();
            p.to = Some(reply_to.clone());
            p.content_type = Some("application/json".into());
            sender
                .send(
                    Message::builder()
                        .properties(p)
                        .data(serde_json::to_vec(&frame)?)
                        .build(),
                )
                .await?;
            println!("[responder] {status} -> {reply_to} corr={request_message_id:?}");
            if idx < progression.len() - 1 && delay > 0.0 {
                tokio::time::sleep(Duration::from_secs_f64(delay)).await;
            }
        }
        replied += 1;
    }

    sender.close().await?;
    receiver.close().await?;
    session.end().await?;
    connection.close().await?;
    Ok(())
}
