/*
 * File:        output/rust/snippets/s1.rs
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
//! Scenario S1 — Subscribe to detector.measurement and print each message.
//!
//! Demonstrates: connect over AMQP 1.0 + mTLS, subscribe to a single
//! channel, parse the JSON payload, print, exit after the first N
//! messages.
//!
//! Tip: a typed SDK is not yet generated for Rust; derive
//! `Deserialize` on a struct of your choosing for static typing once
//! the SDK ships (the Rust SDK was deferred to a follow-up
//! sub-project).

use std::env;
use std::time::Duration;

use fe2o3_amqp::types::messaging::{Body, Source};
use fe2o3_amqp::types::primitives::Symbol;
use fe2o3_amqp::{Connection, Receiver, Session};
use serde_json::Value;
use tokio::time::timeout;

const CHANNEL: &str = "detector.measurement";
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

/// Read the AMQP body as a UTF-8 string. TOPAS 3000A uses JSON in
/// the AMQP value/data section by default (A1); tolerate both forms
/// so the snippet works against brokers that prefer either one.
///
/// fe2o3-amqp 0.14.0 note: `Body::Data` carries `Batch<Data>` (=
/// `TransparentVec<Data>`); the inner `Vec` field is private, so we
/// iterate via the `Deref<Target = Vec<Data>>` impl rather than
/// touching `data.0` directly. Each `Data(pub Binary)` exposes its
/// `ByteBuf` payload, on which `.as_ref()` yields `&[u8]`.
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

    let mut connection = Connection::open("topas-s1", url.as_str()).await?;
    let mut session = Session::begin(&mut connection).await?;
    // SourceCapabilities=["topic"] tells Artemis the address is
    // MULTICAST (pub/sub fan-out), not the default ANYCAST queue
    // behaviour. Without it the receiver binds to a queue named after
    // the address and never receives the publisher's MULTICAST
    // messages. Same root cause as the C# and TypeScript cases.
    // fe2o3-amqp 0.14.0 only exposes capabilities via
    // the builder API — the 3-arg `Receiver::attach` shortcut
    // produces a default Source with no capabilities.
    let source = Source::builder()
        .address(CHANNEL)
        .capabilities(vec![Symbol::from("topic")])
        .build();
    let mut receiver = Receiver::builder()
        .name("s1-receiver")
        .source(source)
        .attach(&mut session)
        .await?;
    // Readiness handshake — the link is attached, so a burst
    // published from here on will be delivered rather than discarded.
    eprintln!("[ready]");  // readiness handshake

    for received in 0..max {
        // 5s receive timeout per message; broker idle => abort.
        // `recv::<Body<Value>>()`: in 0.14.0 `serde_json::Value` does
        // not impl `FromBody`, but `Body<Value>` does — we then
        // pattern-match on the body section.
        let delivery = match timeout(RECV_TIMEOUT, receiver.recv::<Body<Value>>()).await {
            Ok(Ok(d)) => d,
            Ok(Err(e)) => {
                eprintln!("[S1] receive failed: {e}");
                break;
            }
            Err(_) => {
                eprintln!("[S1] receive timed out — broker idle? aborting.");
                break;
            }
        };

        let raw = body_as_string(delivery.body());
        // Parse to a generic Value for printing; in production decode
        // into a typed struct once the Rust SDK ships.
        match serde_json::from_str::<Value>(&raw) {
            Ok(payload) => println!("[{}] {}", received + 1, payload),
            Err(e) => eprintln!("[S1] JSON parse failed: {e}"),
        }

        if let Err(e) = receiver.accept(&delivery).await {
            eprintln!("[S1] accept failed: {e}");
        }
    }

    receiver.close().await?;
    session.end().await?;
    connection.close().await?;
    Ok(())
}
