/*
 * File:        output/typescript/snippets/s2.ts
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
import rhea from "rhea";

/**
 * Scenario S2 — Wildcard subscription across a domain.
 *
 * Demonstrates: subscribing to *.fault (every
 * fault from every device domain) using the AMQP 1.0 source address.
 * Teaches A8 (channel naming convention).
 *
 * Wildcard syntax is broker-specific. rhea passes the source address
 * through verbatim, so the exact pattern depends on the broker:
 *   - Apache ActiveMQ Artemis treats `*.fault` as an address-pattern.
 *   - Apache Qpid Dispatch Router uses the multicast prefix
 *     (e.g. `mc/*.fault`).
 * Adjust to your deployment.
 */

const PATTERN = "*.fault";
const SERVER_URL =
  process.env.TOPAS_AMQP_URL ??
  "amqps://{host}:{port}".replace("{host}", "localhost").replace("{port}", "5671");
const MAX_MESSAGES = parseInt(process.env.TOPAS_MAX_MESSAGES ?? "5", 10);

let received = 0;

const container = rhea.create_container();

container.on("connection_open", (ctx) => {
  ctx.connection.open_receiver({
    source: { address: PATTERN, capabilities: ["topic"] },
  });
});

container.on("message", (ctx) => {
  const payload = parseJsonBody(ctx.message?.body);

  // Concrete address arrives in `message.to` (set by the broker on
  // delivery for wildcard sources). Fall back to the pattern if the
  // broker did not populate it.
  const concrete = (ctx.message?.to as string | undefined) ?? PATTERN;
  const dotIdx = concrete.indexOf(".");
  const domain = dotIdx > 0 ? concrete.substring(0, dotIdx) : concrete;

  received += 1;
  console.log(`[fault from ${domain}] ${JSON.stringify(payload)}`);
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
  console.error(`[S2] error: ${err}`);
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

function parseJsonBody(body: unknown): unknown {
  if (typeof body === "string") {
    return JSON.parse(body);
  }
  if (Buffer.isBuffer(body)) {
    return JSON.parse(body.toString("utf-8"));
  }
  return body;
}
