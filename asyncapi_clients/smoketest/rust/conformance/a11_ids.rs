// ──────────────────────────────────────────────────────────────────────
// File:        smoketest/rust/conformance/a11_ids.rs
// Origin:      Hand-written for asyncapi-clients conformance testing
//
// Author:      Christopher Kettell, TRL Software
// Date:        2026-08-28
// Version:     1.0
// License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
// ──────────────────────────────────────────────────────────────────────
//
//! Round-trip all four AMQP message-id representations through the broker
//! and require each to come back as the same variant *and* value.
//!
//! A11 says a response's `correlation-id` must equal the request's
//! `message-id`. AMQP 1.0 defines that identifier as a union of ulong,
//! uuid, binary and string — so "equal" is a question about type as well
//! as value, and a client that renders both sides as text before
//! comparing is not testing the requirement. That is not hypothetical:
//! rhea surfaces a uuid as a 16-byte Buffer whose `String()` rendering is
//! the empty string, so a textual comparison there reports a uuid as
//! absent.
//!
//! Rust is the only client in this suite where the union is explicit in
//! the type system — `MessageId` is an enum deriving `Eq` — so it is the
//! only place the property can be asserted directly rather than
//! approximated. It is checked here on behalf of the whole suite: if the
//! broker preserved only some representations, every language's
//! transport assertion would rest on an assumption this proves.
//!
//! This is harness code, not a reference example. The generated snippets
//! under output/ show how to use the protocol; this shows the protocol
//! behaves as the snippets assume.

use fe2o3_amqp::types::messaging::{Body, Message, MessageId, Properties, Source};
use fe2o3_amqp::types::primitives::{Binary, Uuid as AmqpUuid};
use fe2o3_amqp::{Connection, Receiver, Sender, Session};
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = std::env::var("TOPAS_AMQP_URL")
        .unwrap_or_else(|_| "amqp://localhost:5672".to_string());

    let mut connection = Connection::open("topas-a11-ids", url.as_str()).await?;
    let mut session = Session::begin(&mut connection).await?;

    // A dynamic reply node, exactly as S3 uses — the point is to exercise
    // the identifiers over the same path a real response travels.
    let source = Source::builder().dynamic(true).build();
    let mut receiver = Receiver::builder()
        .name("a11-ids-reply")
        .source(source)
        .attach(&mut session)
        .await?;
    let address = receiver
        .source()
        .as_ref()
        .and_then(|s| s.address.clone())
        .ok_or("broker did not assign a dynamic reply address")?;
    eprintln!("[ids] reply node = {address}");

    let mut sender = Sender::builder()
        .name("a11-ids-send")
        .target(address.as_str())
        .attach(&mut session)
        .await?;

    // One of each variant the AMQP 1.0 message-id union admits.
    let cases: Vec<(&str, MessageId)> = vec![
        ("String", MessageId::String("str-id-123".to_string())),
        ("Ulong", MessageId::Ulong(42)),
        ("Uuid", MessageId::Uuid(AmqpUuid::from([7u8; 16]))),
        ("Binary", MessageId::Binary(Binary::from(vec![1u8, 2, 3, 4]))),
    ];

    for (label, id) in &cases {
        let mut props = Properties::default();
        props.message_id = Some(id.clone());
        props.correlation_id = Some(id.clone());
        props.to = Some(address.clone());
        sender
            .send(
                Message::builder()
                    .properties(props)
                    .data(serde_json::to_vec(&serde_json::json!({ "variant": label }))?)
                    .build(),
            )
            .await?;
    }

    let mut failures = 0;
    for (label, sent) in &cases {
        let delivery = receiver.recv::<Body<Value>>().await?;
        let got = delivery
            .message()
            .properties
            .as_ref()
            .and_then(|p| p.correlation_id.clone());
        // Typed equality: same variant and same value. `MessageId` derives
        // Eq, so this compares the identifier rather than a rendering of
        // it — a uuid that came back as a string is not equal, which is
        // the answer we want.
        let ok = got.as_ref() == Some(sent);
        if !ok {
            failures += 1;
        }
        println!(
            "[ids] {:<7} sent={:?} received={:?} typedEqual={}",
            label, sent, got, ok
        );
        let _ = receiver.accept(&delivery).await;
    }

    println!("[ids] variants={} failures={}", cases.len(), failures);
    println!(
        "A11 IDENTIFIER TYPES: {}",
        if failures == 0 { "PASS" } else { "FAIL" }
    );

    receiver.close().await?;
    sender.close().await?;
    session.end().await?;
    connection.close().await?;
    if failures == 0 {
        Ok(())
    } else {
        Err("one or more identifier representations were not preserved".into())
    }
}
