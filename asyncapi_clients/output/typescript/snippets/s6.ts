/*
 * File:        output/typescript/snippets/s6.ts
 * Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
 * Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-11-12
 * Generator:   asyncapi_clients/generator v0.1.0
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 */
import rhea from "rhea";

/**
 * Scenario S6 — Discriminated-union consumer for transport-link.measurement.
 *
 * Demonstrates: client-side dispatch on the `kind` discriminator
 *. The TransportLinkMeasurement payload is one of three
 * variants — SCOOT, GPS, or ANPR — distinguished by the `kind`
 * field. The snippet reads the JSON, switches on the discriminator,
 * and dispatches to a per-variant handler.
 *
 * This is the idiom for handling any of the discriminated unions in
 * TOPAS 3000A (F6 in the cross-cutting design log).
 *
 * Tip: a typed SDK is not yet generated for TypeScript. When it
 * ships, replace the inline interfaces below with the generated
 * tagged-union types and let the compiler narrow each branch
 * automatically.
 */

const CHANNEL = "transport-link.measurement";
const SERVER_URL =
  process.env.TOPAS_AMQP_URL ??
  "amqps://{host}:{port}".replace("{host}", "localhost").replace("{port}", "5671");
const MAX_MESSAGES = parseInt(process.env.TOPAS_MAX_MESSAGES ?? "5", 10);

interface ScootVariant {
  kind: "scoot";
  currentFlowVehicles?: number | string;
  occupancyPercent?: number | string;
}

interface GpsVariant {
  kind: "gps";
  meanTravelTimeSeconds?: number | string;
}

interface AnprVariant {
  kind: "anpr";
  numberPlatesIn?: number | string;
  numberPlatesOut?: number | string;
  numberPlatesMatches?: number | string;
}

type TransportLinkMeasurement =
  | ScootVariant
  | GpsVariant
  | AnprVariant
  | { kind: string };

let received = 0;

const container = rhea.create_container();

container.on("connection_open", (ctx) => {
  ctx.connection.open_receiver({
    source: { address: CHANNEL, capabilities: ["topic"] },
  });
});

container.on("message", (ctx) => {
  const variant = parseJsonBody(ctx.message?.body) as TransportLinkMeasurement;
  received += 1;
  console.log(`[${received}] ${handle(variant)}`);
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
  console.error(`[S6] error: ${err}`);
  process.exit(1);
});

/** Dispatch on the `kind` JSON discriminator. */
function handle(variant: TransportLinkMeasurement): string {
  switch (variant.kind) {
    case "scoot": {
      const v = variant as ScootVariant;
      return `SCOOT flow=${v.currentFlowVehicles ?? "n/a"} occupancy%=${v.occupancyPercent ?? "n/a"}`;
    }
    case "gps": {
      const v = variant as GpsVariant;
      return `GPS meanTravelTime=${v.meanTravelTimeSeconds ?? "n/a"}s`;
    }
    case "anpr": {
      const v = variant as AnprVariant;
      return `ANPR in=${v.numberPlatesIn ?? "n/a"} out=${v.numberPlatesOut ?? "n/a"} matches=${v.numberPlatesMatches ?? "n/a"}`;
    }
    default:
      return `unknown variant kind=${variant.kind || "<missing>"}`;
  }
}

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
