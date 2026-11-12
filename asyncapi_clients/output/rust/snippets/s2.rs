/*
 * File:        output/rust/snippets/s2.rs
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
//! Scenario S2 — Wildcard subscription across a domain.
//!
//! Demonstrates: subscribing to *.fault (every
//! fault from every device domain) using AMQP 1.0 source filters.
//! Teaches A8 (channel naming convention).
//!
//! Broker-specific syntax may differ (Artemis: `*.fault`, Qpid
//! Dispatch: `mc/*.fault`); adjust the source address — or pass a
//! richer filter set via `ReceiverBuilder::source(...)` — to match
//! your deployment.

use std::env;
use std::time::Duration;

use fe2o3_amqp::types::messaging::{Body, Source};
use fe2o3_amqp::types::primitives::Symbol;
use fe2o3_amqp::{Connection, Receiver, Session};
use serde_json::Value;
use tokio::time::timeout;

const PATTERN: &str = "*.fault";
const RECV_TIMEOUT: Duration = Duration::from_secs(5);

fn server_url() -> String {
    env::var("TOPAS_AMQP_URL")
        .unwrap_or_else(|_| "amqps://{host}:{port}".replace("{host}", "localhost").replace("{port}", "5671"))
}

fn max_messages() -> usize {
    env::var("TOPAS_MAX_MESSAGES")
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(5)
}

fn body_as_string(body: &Body<Value>) -> String {
    match body {
        Body::Data(batch) => {
            let mut out = String::new();
            for data in batch.iter() {
                out.push_str(&String::from_utf8_lossy(data.0.as_ref()));
            }
            out
        }
        Body::Value(amqp_value) => match &amqp_value.0 {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        },
        Body::Sequence(_) | Body::Empty => String::new(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = server_url();
    let max = max_messages();

    let mut connection = Connection::open("topas-s2", url.as_str()).await?;
    let mut session = Session::begin(&mut connection).await?;
    // SourceCapabilities=["topic"] — see s1.rs for rationale.
    let source = Source::builder()
        .address(PATTERN)
        .capabilities(vec![Symbol::from("topic")])
        .build();
    let mut receiver = Receiver::builder()
        .name("s2-receiver")
        .source(source)
        .attach(&mut session)
        .await?;
    // Readiness handshake — the link is attached, so a burst
    // published from here on will be delivered rather than discarded.
    eprintln!("[ready]");  // readiness handshake

    for _ in 0..max {
        let delivery = match timeout(RECV_TIMEOUT, receiver.recv::<Body<Value>>()).await {
            Ok(Ok(d)) => d,
            Ok(Err(e)) => {
                eprintln!("[S2] receive failed: {e}");
                break;
            }
            Err(_) => {
                eprintln!("[S2] receive timed out — broker idle? aborting.");
                break;
            }
        };

        // The address (the channel the broker delivered on) tells us
        // the originating domain — `vms.fault`, `detector.fault`, etc.
        let address: String = delivery
            .message()
            .properties
            .as_ref()
            .and_then(|p| p.to.as_ref())
            .map(|to| to.to_string())
            .unwrap_or_else(|| PATTERN.to_string());
        let domain = address.split('.').next().unwrap_or(&address).to_string();

        let raw = body_as_string(delivery.body());
        match serde_json::from_str::<Value>(&raw) {
            Ok(payload) => println!("[fault from {domain}] {payload}"),
            Err(e) => eprintln!("[S2] JSON parse failed: {e}"),
        }

        if let Err(e) = receiver.accept(&delivery).await {
            eprintln!("[S2] accept failed: {e}");
        }
    }

    receiver.close().await?;
    session.end().await?;
    connection.close().await?;
    Ok(())
}
