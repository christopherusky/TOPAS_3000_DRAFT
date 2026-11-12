"""Mock VMS that consumes vms.command and replies to each request's reply-to node.

File:        smoketest/vms_responder.py
Origin:      Hand-written for asyncapi-clients smoke testing
Changes:     Hand-edit allowed — this is a test harness, not generated output
             2026-05-06: emit a configurable status progression
             (default pending,pending,success) per command, sharing one
             correlation-id, to exercise S3 multi-response receive loops.
Author:      Christopher Kettell, TRL Software
Date:        2026-05-06
License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)

Lets the S3 snippet exercise its full request/response round-trip
without a real VMS in the loop. For each command received, emits a
sequence of synthetic command_response frames addressed to the request's
reply-to node (A11), all sharing the request's correlation-id. Responses are
never broadcast on a channel and filtered by originator (A4, replaced).

Configuration (env):
    TOPAS_RESPONDER_PROGRESSION  Comma-separated CommandResponseStatus
                                 values to emit per command.
                                 Default: "pending,pending,success".
    TOPAS_RESPONDER_DELAY        Seconds between successive frames in
                                 a progression (default 0.15).
    TOPAS_RESPONDER_MAX_REPLIES  Exit after N whole progressions
                                 (one per command). Unset = run forever.
"""
from __future__ import annotations

import json
import os
import sys
import time
from datetime import datetime, timezone

from proton import Message
from proton.handlers import MessagingHandler
from proton.reactor import Container


COMMAND_CHANNEL = "vms.command"
SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
PROGRESSION_ENV = os.environ.get("TOPAS_RESPONDER_PROGRESSION", "pending,pending,success")
PROGRESSION = [s.strip() for s in PROGRESSION_ENV.split(",") if s.strip()]
INTER_FRAME_DELAY = float(os.environ.get("TOPAS_RESPONDER_DELAY", "0.15"))
MAX_REPLIES_ENV = os.environ.get("TOPAS_RESPONDER_MAX_REPLIES")
MAX_REPLIES = int(MAX_REPLIES_ENV) if MAX_REPLIES_ENV else None


class UndecodableBody(Exception):
    """A command body in a representation this responder does not handle."""


def _decode_body(body):
    """Decode a command body, or fail loudly.

    An explicit compatibility layer over the representations clients
    actually produce: a str from Proton, bytes from a JMS BytesMessage, a
    buffer from others. Anything else raises.

    It deliberately does not degrade to ``{"raw": str(body)}``. That is what
    it used to do, and it turned a Java BytesMessage into
    ``{"raw": "<memory at 0x...>"}`` — so the response carried no
    commandCorrelationId, and every requester's "if present, compare" check
    passed vacuously. In a conformance responder, an
    unrecognised body must be a visible failure rather than a plausible
    looking placeholder.
    """
    if isinstance(body, (bytes, bytearray, memoryview)):
        body = bytes(body).decode("utf-8")
    if not isinstance(body, str):
        raise UndecodableBody(
            f"unhandled body representation {type(body).__name__!r}; "
            "add it to _decode_body rather than degrading"
        )
    return json.loads(body)


class VmsResponder(MessagingHandler):
    def __init__(self, url: str) -> None:
        super().__init__()
        self.url = url
        self.sender = None
        self.replied = 0

    def on_start(self, event):
        conn = event.container.connect(self.url)
        event.container.create_receiver(conn, COMMAND_CHANNEL)
        # Anonymous sender (target=None): the destination travels on each
        # message's `to` property, so one link answers every requester's
        # reply node without a sender per requester (A11.4). This is the
        # preferred mechanism where the peer offers ANONYMOUS-RELAY.
        self.sender = event.container.create_sender(conn, None)
        # Readiness handshake — the harness waits for this rather than
        # sleeping, so a command published before this consumer attached
        # cannot be silently discarded. The line is identical
        # across every language's responder so the harness can drive any
        # of them without knowing which.
        print("[responder-ready] python (anonymous sender)", flush=True)

    def on_message(self, event):
        msg = event.message
        # Bodies arrive as str from Proton clients and as bytes/memoryview
        # from a JMS BytesMessage. Decoding both matters: without it a Java
        # command parsed as {"raw": "<memory at 0x...>"}, the response never
        # carried commandCorrelationId back, and any requester that only
        # checks the field "if present" passed vacuously.
        try:
            command = _decode_body(msg.body)
        except (UndecodableBody, ValueError, UnicodeDecodeError) as exc:
            # Loud, and does not answer: a requester that receives nothing
            # fails on its expected-response count, which is the correct
            # outcome for a malformed command.
            print(f"[responder] FATAL: {exc}", file=sys.stderr, flush=True)
            event.connection.close()
            return
        originator = (msg.properties or {}).get("x-originator", "unknown")
        # A11: the response goes to the address the requester supplied, and
        # correlation-id mirrors the request's message-id.
        #
        # The shared-channel fallback that used to live here is gone.
        # It existed to serve snippets not yet converted, and every language
        # the harness executes now speaks A11. Keeping it would mean a
        # requester that forgot `reply-to` still received an answer, which
        # would let a regression back into fan-out addressing pass unnoticed
        # — the precise thing A11 replaced.
        reply_to = msg.reply_to
        if not reply_to:
            print(
                "[responder] FATAL: command carried no reply-to; under A11 "
                "there is nowhere to address the response",
                file=sys.stderr, flush=True,
            )
            event.connection.close()
            return
        # correlation-id mirrors the request's message-id, which A11 requires
        # and which the conformance suite asserts as typed equality.
        correlation_id = msg.id
        vms_id = command.get("vmsId") if isinstance(command, dict) else None
        command_id = command.get("commandId") if isinstance(command, dict) else None
        # A11 requires the application correlation identifier to be carried
        # from request to response unchanged, at the top level rather than
        # buried in an echo.
        command_correlation_id = (
            command.get("commandCorrelationId") if isinstance(command, dict) else None
        )

        for idx, status in enumerate(PROGRESSION):
            frame = {
                "vmsId": vms_id,
                "commandId": command_id,
                "commandCorrelationId": command_correlation_id,
                "status": status,
                "responseTime": datetime.now(timezone.utc).isoformat(),
                "echo": command,
            }
            reply = Message(
                body=json.dumps(frame),
                correlation_id=correlation_id,
                properties={
                    # Audit only under A11 — not how the response is routed.
                    "x-originator": originator,
                },
            )
            reply.address = reply_to
            self.sender.send(reply)
            print(
                f"[responder] {status} -> {reply_to} corr={correlation_id}",
                flush=True,
            )
            if idx < len(PROGRESSION) - 1 and INTER_FRAME_DELAY > 0:
                time.sleep(INTER_FRAME_DELAY)

        self.replied += 1
        if MAX_REPLIES is not None and self.replied >= MAX_REPLIES:
            event.connection.close()


def main() -> int:
    Container(VmsResponder(SERVER)).run()
    return 0


if __name__ == "__main__":
    sys.exit(main())
