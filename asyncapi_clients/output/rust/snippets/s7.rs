/*
 * File:        output/rust/snippets/s7.rs
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
//! Scenario S7 — Static reference data: consume the network topology.
//!
//! Demonstrates: the snapshot-then-disconnect idiom for static
//! reference data. Network topology is published across five
//! channels (one per object type: node, link, zone, path, turn). A
//! consumer that needs the topology opens five receivers in one
//! session, accumulates each object into a per-channel `HashMap`
//! keyed by the per-type id field, and disconnects once all five streams have
//! produced their first message — different from S1/S5 which are
//! continuous-telemetry consumers.
//!
//! Concurrency: this snippet uses `tokio::spawn` to run one task per
//! receiver — `Receiver::recv` is async-but-blocking-per-link, and
//! spawning is the simplest way to drain five channels in one
//! session. A `tokio::sync::Notify` signals task shutdown once every
//! channel has produced its first message.
//!
//! The resulting in-memory maps are what a route-planning or
//! incident-location component would build before serving its first
//! request.

use std::collections::{HashMap, HashSet};
use std::env;
use std::pin::pin;
use std::sync::Arc;
use std::time::Duration;

use fe2o3_amqp::types::messaging::{Body, Source};
use fe2o3_amqp::types::primitives::Symbol;
use fe2o3_amqp::{Connection, Receiver, Session};
use serde_json::Value;
use tokio::sync::{Mutex, Notify};
use tokio::time::timeout;

const CHANNELS: [&str; 5] = [
    "network.node",
    "network.link",
    "network.zone",
    "network.path",
    "network.turn",
];

/// Each network object has a type-specific id field (no shared objectId).
fn id_field_for(channel: &str) -> &'static str {
    match channel {
        "network.node" => "nodeId",
        "network.link" => "linkId",
        "network.zone" => "zoneId",
        "network.path" => "pathId",
        "network.turn" => "turnId",
        _ => "",
    }
}
const RECV_TIMEOUT: Duration = Duration::from_secs(1);
const TOTAL_DEADLINE: Duration = Duration::from_secs(30);

fn server_url() -> String {
    env::var("TOPAS_AMQP_URL")
        .unwrap_or_else(|_| "amqps://{host}:{port}".replace("{host}", "localhost").replace("{port}", "5671"))
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
        Body::Value(v) => match &v.0 {
            Value::String(s) => s.clone(),
            other => other.to_string(),
        },
        Body::Sequence(_) | Body::Empty => String::new(),
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = server_url();

    // Progress markers on stderr. S7 is the only snippet that attaches five
    // receivers before it can make progress, and a CI failure here previously
    // produced no output at all — the orchestrator killed the process before
    // TOTAL_DEADLINE could fire, leaving nothing to diagnose. These say which
    // phase we reached.
    eprintln!("[S7] connecting to {url}");
    let mut connection = Connection::open("topas-s7", url.as_str()).await?;
    let mut session = Session::begin(&mut connection).await?;
    eprintln!("[S7] session begun; attaching {} receivers", CHANNELS.len());

    // Per-channel object store keyed by the per-type id field, behind a single
    // mutex (one writer per channel — contention is negligible).
    let topology: Arc<Mutex<HashMap<&'static str, HashMap<String, Value>>>> =
        Arc::new(Mutex::new(
            CHANNELS.iter().map(|c| (*c, HashMap::new())).collect(),
        ));
    let seen: Arc<Mutex<HashSet<&'static str>>> = Arc::new(Mutex::new(HashSet::new()));
    let done = Arc::new(Notify::new());

    // Pre-register the "all-five-seen" waiter BEFORE spawning the receiver
    // tasks. `tokio::sync::Notify::notified()` returns a future that only
    // registers as a waiter on its first poll; without pre-registration, a
    // spawned task can call `done.notify_waiters()` before the main task's
    // `.await` polls the future, and the wake is silently missed.
    // `Notified::enable()` (tokio ≥1.13) registers the waiter without yet
    // yielding, so subsequent `notify_waiters()` calls are seen even if they
    // race the main task's progression to the await point.
    let mut waiter = pin!(done.notified());
    waiter.as_mut().enable();

    let mut handles = Vec::with_capacity(CHANNELS.len());
    for channel in CHANNELS.iter() {
        let receiver_name = format!("s7-{}", channel.replace('.', "-"));
        let source = Source::builder()
            .address(*channel)
            .capabilities(vec![Symbol::from("topic")])
            .build();
        let mut receiver = Receiver::builder()
            .name(receiver_name)
            .source(source)
            .attach(&mut session)
            .await?;
        eprintln!("[S7] attached {channel}");

        let topology = Arc::clone(&topology);
        let seen = Arc::clone(&seen);
        let done = Arc::clone(&done);
        let channel: &'static str = channel;

        handles.push(tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = done.notified() => break,
                    res = timeout(RECV_TIMEOUT, receiver.recv::<Body<Value>>()) => {
                        let delivery = match res {
                            Ok(Ok(d)) => d,
                            Ok(Err(_)) => break,
                            Err(_) => continue,  // idle tick — re-check `done`.
                        };

                        let raw = body_as_string(delivery.body());
                        let obj: Value = match serde_json::from_str(&raw) {
                            Ok(v) => v,
                            Err(_) => {
                                let _ = receiver.accept(&delivery).await;
                                continue;
                            }
                        };
                        // Every network object carries its type-specific
                        // id field (nodeId, linkId, zoneId, pathId, turnId).
                        let id = obj
                            .get(id_field_for(channel))
                            .and_then(|v| v.as_str())
                            .map(str::to_string);
                        if let Some(id) = id {
                            topology.lock().await.entry(channel).or_default().insert(id, obj);
                            let mut seen_guard = seen.lock().await;
                            seen_guard.insert(channel);
                            if seen_guard.len() == CHANNELS.len() {
                                done.notify_waiters();
                            }
                        }
                        let _ = receiver.accept(&delivery).await;
                    }
                }
            }
            let _ = receiver.close().await;
        }));
    }

    // Wait for all-five-seen (via the pre-registered waiter above), or the
    // overall deadline.
    // Readiness handshake — every receiver is attached, so it is
    // now safe for the harness to start the publisher.
    eprintln!("[ready]");
    eprintln!("[S7] all receivers attached; waiting for first message on each");
    let timed_out = timeout(TOTAL_DEADLINE, waiter).await.is_err();

    // Report the timeout BEFORE shutting the receiver tasks down. Each task
    // ends with `receiver.close().await`, an AMQP detach round-trip; if the
    // peer is slow to answer, the join below can outlast the harness's own
    // deadline. Printing after the join meant a timeout produced no
    // diagnostic at all — CI showed five successful attaches, the "waiting"
    // line, and then nothing, which is what made this so hard to place.
    if timed_out {
        let seen_now: Vec<&str> = seen.lock().await.iter().copied().collect();
        eprintln!("[S7] timed out waiting for all five channels; saw: {seen_now:?}");
    }

    // Tell every task to stop, regardless of which arm fired. The spawned
    // tasks register their own `done.notified()` futures inside `select!`,
    // which polls them on entry — so they aren't subject to the same
    // pre-registration race the main task was.
    done.notify_waiters();
    // Bound the join for the same reason: a hung link close must not stop
    // the process exiting and surrendering its output.
    for h in handles {
        let _ = timeout(Duration::from_secs(2), h).await;
    }

    let topology_snapshot = topology.lock().await;
    if !timed_out {
        let parts: Vec<String> = CHANNELS
            .iter()
            .map(|ch| {
                let short = ch.rsplit('.').next().unwrap_or(ch);
                let count = topology_snapshot.get(*ch).map(|m| m.len()).unwrap_or(0);
                format!("{short}={count}")
            })
            .collect();
        println!("[topology] {}", parts.join(", "));
    }

    session.end().await?;
    connection.close().await?;
    Ok(())
}
