/*
 * File:        output/typescript/snippets/s4.ts
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
import rhea from "rhea";

/**
 * Scenario S4 — Distinguish sync vs delta messages and reassemble a sync batch.
 *
 * Demonstrates: reading the x-message-mode AMQP application
 * property (A3) to tell snapshot data from change events; using the
 * x-sync-id / x-sync-sequence /
 * x-sync-total triple (A5) to detect when a sync batch is
 * complete.
 *
 * Sync progress is tracked in a `Map<string, Set<number>>` keyed by
 * x-sync-id; each set entry corresponds to one
 * x-sync-sequence value seen on the wire.
 */

const CHANNEL = "detector.measurement";
const SERVER_URL =
  process.env.TOPAS_AMQP_URL ??
  "amqps://{host}:{port}".replace("{host}", "localhost").replace("{port}", "5671");
const MAX_MESSAGES = parseInt(process.env.TOPAS_MAX_MESSAGES ?? "20", 10);

let received = 0;
const syncProgress = new Map<string, Set<number>>();

const container = rhea.create_container();

container.on("connection_open", (ctx) => {
  ctx.connection.open_receiver({
    source: { address: CHANNEL, capabilities: ["topic"] },
  });
});

container.on("message", (ctx) => {
  const props =
    (ctx.message?.application_properties as Record<string, unknown> | undefined) ??
    {};
  const mode = stringOrUndef(props["x-message-mode"]) ?? "delta";
  const payload = parseJsonBody(ctx.message?.body);

  if (mode === "delta") {
    console.log(`[delta] ${JSON.stringify(payload)}`);
  } else {
    // mode === "sync" — track batch completion.
    const syncId = stringOrUndef(props["x-sync-id"]);
    const seq = intOrZero(props["x-sync-sequence"]);
    const total = intOrZero(props["x-sync-total"]);

    if (!syncId || total === 0) {
      console.log(
        `[sync*] ${JSON.stringify(payload)}  (incomplete sync metadata)`,
      );
    } else {
      let seen = syncProgress.get(syncId);
      if (!seen) {
        seen = new Set<number>();
        syncProgress.set(syncId, seen);
      }
      seen.add(seq);
      const progress = seen.size;
      console.log(
        `[sync ${syncId} ${progress}/${total}] ${JSON.stringify(payload)}`,
      );
      if (progress === total) {
        console.log(`[sync ${syncId}] complete`);
        syncProgress.delete(syncId);
      }
    }
  }

  received += 1;
  if (received >= MAX_MESSAGES) {
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
  if (__linksOpen === 1) {
    console.error("[ready]");
  }
});

container.on("connection_close", () => process.exit(0));
container.on("disconnected", () => process.exit(0));

container.on("error", (err) => {
  console.error(`[S4] error: ${err}`);
  process.exit(1);
});

const parsed = new URL(SERVER_URL);
const isTls = parsed.protocol === "amqps:";
const host = parsed.hostname;
const port = parseInt(parsed.port || (isTls ? "5671" : "5672"), 10);
if (isTls) {
  container.connect({ host, port, transport: "tls" });
} else {
  container.connect({ host, port });
}

function stringOrUndef(v: unknown): string | undefined {
  if (v === undefined || v === null) return undefined;
  return typeof v === "string" ? v : String(v);
}

function intOrZero(v: unknown): number {
  if (v === undefined || v === null) return 0;
  if (typeof v === "number") return Math.trunc(v);
  if (typeof v === "string") {
    const n = parseInt(v, 10);
    return Number.isFinite(n) ? n : 0;
  }
  if (typeof v === "bigint") return Number(v);
  const n = Number(v);
  return Number.isFinite(n) ? Math.trunc(n) : 0;
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
