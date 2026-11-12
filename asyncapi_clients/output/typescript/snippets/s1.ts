/*
 * File:        output/typescript/snippets/s1.ts
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
import rhea from "rhea";

/**
 * Scenario S1 — Subscribe to detector.measurement and print each message.
 *
 * Demonstrates: connect over AMQP 1.0 + mTLS, subscribe to a single
 * channel, parse the JSON payload, print, exit after the first N
 * messages.
 *
 * Tip: a typed SDK is not yet generated for TypeScript; use a Zod
 * schema or an inline interface for static typing. Once the SDK
 * ships, replace the `unknown` cast below with the generated
 * `DetectorMeasurement` type.
 */

const CHANNEL = "detector.measurement";
const SERVER_URL =
  process.env.TOPAS_AMQP_URL ??
  "amqps://{host}:{port}".replace("{host}", "localhost").replace("{port}", "5671");
const MAX_MESSAGES = parseInt(process.env.TOPAS_MAX_MESSAGES ?? "5", 10);

let received = 0;

const container = rhea.create_container();

container.on("connection_open", (ctx) => {
  ctx.connection.open_receiver({
    source: { address: CHANNEL, capabilities: ["topic"] },
  });
});

container.on("message", (ctx) => {
  const payload = parseJsonBody(ctx.message?.body);
  received += 1;
  console.log(`[${received}] ${JSON.stringify(payload)}`);
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
  console.error(`[S1] error: ${err}`);
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

/**
 * Read the AMQP message body as a parsed JSON value. TOPAS 3000A
 * carries JSON in the AMQP value/data section (A1); rhea may surface
 * that as a string, a Buffer, or — when the broker sends an AMQP
 * value section that is already a map — as a plain object. Tolerant
 * readers across all three are cheap insurance.
 */
function parseJsonBody(body: unknown): unknown {
  if (typeof body === "string") {
    return JSON.parse(body);
  }
  if (Buffer.isBuffer(body)) {
    return JSON.parse(body.toString("utf-8"));
  }
  // Already-decoded AMQP value section; pass through.
  return body;
}
