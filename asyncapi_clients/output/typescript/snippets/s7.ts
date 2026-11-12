/*
 * File:        output/typescript/snippets/s7.ts
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
import rhea from "rhea";
import type { EventContext } from "rhea";

/**
 * Scenario S7 — Static reference data: consume the network topology.
 *
 * Demonstrates: the snapshot-then-disconnect idiom for static
 * reference data. Network topology is published across five
 * channels (one per object type: node, link, zone, path, turn). A
 * consumer that needs the topology opens five receivers in one
 * connection, accumulates each object into a per-channel
 * `Map<string, unknown>` keyed by the per-type id field, and disconnects once
 * all five streams have produced their first message — different
 * from S1/S5 which are continuous-telemetry consumers.
 *
 * The resulting in-memory `Map`s are what a route-planning or
 * incident-location component would build before serving its first
 * request.
 */

const CHANNELS = [
  "network.node",
  "network.link",
  "network.zone",
  "network.path",
  "network.turn",
] as const;

// Each network object has a type-specific id field (no shared objectId).
const ID_FIELDS: Record<string, string> = {
  "network.node": "nodeId",
  "network.link": "linkId",
  "network.zone": "zoneId",
  "network.path": "pathId",
  "network.turn": "turnId",
};

const SERVER_URL =
  process.env.TOPAS_AMQP_URL ??
  "amqps://{host}:{port}".replace("{host}", "localhost").replace("{port}", "5671");

// Channel name -> per-channel object store, keyed by the per-type id field.
const topology = new Map<string, Map<string, unknown>>();
for (const ch of CHANNELS) {
  topology.set(ch, new Map<string, unknown>());
}
const seenFirstOn = new Set<string>();
const channelSet = new Set<string>(CHANNELS);

const container = rhea.create_container();

container.on("connection_open", (ctx) => {
  for (const channel of CHANNELS) {
    // Name each receiver after its channel so we can recover the
    // channel from the receiver in the message handler below.
    ctx.connection.open_receiver({
      source: { address: channel, capabilities: ["topic"] },
      name: `s7-receiver-${channel}`,
    });
  }
});

container.on("message", (ctx) => {
  const channel = resolveChannel(ctx);
  if (!channel || !channelSet.has(channel)) {
    return; // unknown receiver, ignore
  }

  const obj = parseJsonBody(ctx.message?.body) as Record<string, unknown>;
  const idField = ID_FIELDS[channel];
  const idVal = obj?.[idField];
  const id = typeof idVal === "string" ? idVal : String(idVal ?? "");
  if (!id) {
    // Safety net — every network object should carry its type-specific
    // id (nodeId, linkId, zoneId, pathId, turnId).
    return;
  }

  topology.get(channel)!.set(id, obj);
  seenFirstOn.add(channel);

  // Disconnect once we've heard from all five streams; in production
  // a consumer would tune this differently (drain each stream, or
  // wait for a sync-batch completion per A5).
  if (seenFirstOn.size === CHANNELS.length) {
    const summary = CHANNELS.map((c) => {
      const dot = c.lastIndexOf(".");
      const shortName = dot >= 0 ? c.substring(dot + 1) : c;
      return `${shortName}=${topology.get(c)!.size}`;
    }).join(", ");
    console.log(`[topology] ${summary}`);
    ctx.connection.close();
  }
});

// Readiness handshake: the harness waits for this before starting
// the publisher. Every publisher sends a fixed burst and exits, so anything
// published before the receiver is attached is discarded by the broker.
// rhea fires receiver_open once the peer confirms the attach.
let __linksOpen = 0;
container.on("receiver_open", () => {
  __linksOpen += 1;
  if (__linksOpen === CHANNELS.length) {
    console.error("[ready]");
  }
});

container.on("connection_close", () => process.exit(0));
container.on("disconnected", () => process.exit(0));

container.on("error", (err) => {
  console.error(`[S7] error: ${err}`);
  process.exit(1);
});

// Bound the wait so we don't hang forever if a stream is silent.
setTimeout(() => {
  if (seenFirstOn.size < CHANNELS.length) {
    const seen = Array.from(seenFirstOn).join(",");
    console.error(
      `[S7] timed out waiting for all five channels; saw: ${seen}`,
    );
    process.exit(2);
  }
}, 30_000).unref();

const parsed = new URL(SERVER_URL);
const isTls = parsed.protocol === "amqps:";
const host = parsed.hostname;
const port = parseInt(parsed.port || (isTls ? "5671" : "5672"), 10);
if (isTls) {
  container.connect({ host, port, transport: "tls" });
} else {
  container.connect({ host, port });
}

/**
 * Recover the subscribed channel name from a delivery context. We
 * prefer the message's `to` address (set by the broker on delivery),
 * falling back to the receiver source address, then to the receiver
 * name we set in the open_receiver call.
 */
function resolveChannel(ctx: EventContext): string | undefined {
  const to = ctx.message?.to as string | undefined;
  if (to) return to;
  const src = ctx.receiver?.source?.address;
  if (typeof src === "string") return src;
  const name = ctx.receiver?.name;
  if (typeof name === "string" && name.startsWith("s7-receiver-")) {
    return name.substring("s7-receiver-".length);
  }
  return undefined;
}

function parseJsonBody(body: unknown): unknown {
  if (typeof body === "string") {
    return JSON.parse(body);
  }
  if (Buffer.isBuffer(body)) {
    return JSON.parse(body.toString("utf-8"));
  }
  return body;
}
