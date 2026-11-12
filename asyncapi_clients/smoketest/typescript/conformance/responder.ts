/*
 * File:        smoketest/typescript/conformance/responder.ts
 * Origin:      Hand-written for asyncapi-clients conformance testing
 * Author:      Christopher Kettell, TRL Software
 * Date:        2026-08-28
 * License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
 *
 * A11 command responder — TypeScript half of the cross-language matrix.
 *
 * The mirror image of S3: consume vms.command, read the requester's
 * reply-to, and answer *that node* with correlation-id mirroring the
 * request's message-id and commandCorrelationId carried through
 * unchanged. Every requester asserts exactly those three things.
 *
 * The sender uses an anonymous terminus — a target with no address — so
 * one link answers every reply node, with each message's `to` routing it
 * (A11.4).
 *
 * Configuration mirrors vms_responder.py so the harness can drive any
 * language's responder without knowing which it is.
 */
import rhea from "rhea";
import type { Sender } from "rhea";

const COMMAND_CHANNEL = "vms.command";
const SERVER_URL = process.env.TOPAS_AMQP_URL ?? "amqp://localhost:5672";
const PROGRESSION = (process.env.TOPAS_RESPONDER_PROGRESSION ?? "pending,pending,success")
  .split(",")
  .map((s) => s.trim())
  .filter((s) => s.length > 0);
const DELAY_MS = parseFloat(process.env.TOPAS_RESPONDER_DELAY ?? "0.15") * 1000;
const MAX_REPLIES = process.env.TOPAS_RESPONDER_MAX_REPLIES
  ? parseInt(process.env.TOPAS_RESPONDER_MAX_REPLIES, 10)
  : null;

const container = rhea.create_container();
let sender: Sender | undefined;
let replied = 0;
let announced = false;

container.on("connection_open", (ctx) => {
  ctx.connection.open_receiver({
    source: { address: COMMAND_CHANNEL, capabilities: ["topic"] },
  });
  // Anonymous terminus: a target with no address. Each message's `to`
  // routes it, so one link serves every reply node (A11.4).
  sender = ctx.connection.open_sender({ target: {} });
});

container.on("receiver_open", () => {
  if (!announced) {
    announced = true;
    // Readiness handshake, identical across languages.
    console.log("[responder-ready] typescript (anonymous terminus)");
  }
});

container.on("message", async (ctx) => {
  const command = ctx.message;
  if (command == null || sender == null) return;

  const replyTo = command.reply_to;
  if (replyTo == null || replyTo === "") {
    // Under A11 there is nowhere to address the response. Loud, and does
    // not answer.
    console.error(
      "[responder] FATAL: command carried no reply-to; under A11 there is " +
        "nowhere to address the response",
    );
    ctx.connection.close();
    return;
  }
  // A11: correlation-id mirrors the request's message-id. Kept as the
  // value rhea produced rather than a string rendering, so a uuid — which
  // rhea surfaces as a Buffer whose String() is empty — survives the
  // round trip.
  const requestMessageId = command.message_id;

  // Decoding does not degrade: an unrecognised body is a visible failure
  // rather than a placeholder.
  const body = decodeCommandBody(command.body) as Record<string, unknown>;

  for (let i = 0; i < PROGRESSION.length; i++) {
    const frame = {
      vmsId: body.vmsId ?? null,
      // Carried through unchanged, at the top level rather than buried
      // in an echo.
      commandCorrelationId: body.commandCorrelationId ?? null,
      status: PROGRESSION[i],
      responseTime: new Date().toISOString(),
      echo: body,
    };
    sender.send({
      body: JSON.stringify(frame),
      correlation_id: requestMessageId,
      to: replyTo,
      content_type: "application/json",
    });
    console.log(
      `[responder] ${PROGRESSION[i]} -> ${replyTo} corr=${describeId(requestMessageId)}`,
    );
    if (i < PROGRESSION.length - 1 && DELAY_MS > 0) {
      await new Promise((r) => setTimeout(r, DELAY_MS));
    }
  }

  replied += 1;
  if (MAX_REPLIES !== null && replied >= MAX_REPLIES) {
    ctx.connection.close();
  }
});

container.on("connection_close", () => process.exit(0));
container.on("disconnected", () => process.exit(0));
container.on("error", (err) => {
  console.error(`[responder] error: ${err}`);
  process.exit(1);
});

setTimeout(() => {
  console.error("[responder] no command within 60s — exiting");
  process.exit(0);
}, 60_000).unref();

const parsed = new URL(SERVER_URL);
container.connect({
  host: parsed.hostname,
  port: parseInt(parsed.port || "5672", 10),
});

/** Names the representation, since String() hides a uuid entirely. */
function describeId(id: unknown): string {
  if (typeof id === "string") return id;
  if (typeof id === "number") return `ulong:${id}`;
  if (Buffer.isBuffer(id)) return `binary/uuid:${id.toString("hex")}`;
  return `${typeof id}`;
}

/**
 * An explicit compatibility layer over the body representations clients
 * produce — a string from an AMQP value section, a Buffer from a data
 * section. Throws rather than degrading.
 */
function decodeCommandBody(body: unknown): unknown {
  if (typeof body === "string") return JSON.parse(body);
  if (Buffer.isBuffer(body)) return JSON.parse(body.toString("utf-8"));
  if (
    body != null &&
    typeof body === "object" &&
    "content" in (body as Record<string, unknown>)
  ) {
    return decodeCommandBody((body as Record<string, unknown>).content);
  }
  throw new Error(
    `unhandled body representation ${Object.prototype.toString.call(body)}; ` +
      "add it to decodeCommandBody rather than degrading",
  );
}
