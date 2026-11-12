# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/idempotency_requester.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-29
# Version:     1.1
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""One command attempt, with a controllable identity and correlation.

A deliberately separate requester from S3. S3 is a reference example and
should stay one; this exists to vary the things A11.2 keys on — the
connection marker the principal provider resolves, the
`commandCorrelationId`, and the command body — and to prove each attempt
is answered on *its own* reply node with *its own* correlation-id.

Configuration (env):
    TOPAS_CONNECTION_MARKER    fixture key the responder maps to a principal
    TOPAS_COMMAND_CORRELATION_ID
    TOPAS_PLAN_NUMBER          varies the body, for the conflict scenario
    TOPAS_ORIGINATOR           self-asserted audit field, for the spoof test
    TOPAS_EXPECTED_RESPONSES   how many frames before exiting
"""
from __future__ import annotations

import json
import os
import sys
import uuid

from proton import Message
from proton.handlers import MessagingHandler
from proton.reactor import Container

COMMAND_CHANNEL = "vms.command"
SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
MARKER = os.environ.get("TOPAS_CONNECTION_MARKER", "conn-a")
CORRELATION = int(os.environ.get("TOPAS_COMMAND_CORRELATION_ID", "7777"))
PLAN = int(os.environ.get("TOPAS_PLAN_NUMBER", "4"))
ORIGINATOR = os.environ.get("TOPAS_ORIGINATOR", "")
EXPECTED = int(os.environ.get("TOPAS_EXPECTED_RESPONSES", "1"))


class Attempt(MessagingHandler):
    def __init__(self, url: str) -> None:
        super().__init__()
        self.url = url
        self.message_id = str(uuid.uuid4())
        self.reply_address = None
        self.sent = False
        self.statuses: list[str] = []
        self.mis_addressed = 0
        self.mis_correlated = 0

    def on_start(self, event):
        conn = event.container.connect(self.url)
        self.sender = event.container.create_sender(conn, COMMAND_CHANNEL)
        event.container.create_receiver(conn, None, dynamic=True)

    def on_link_opened(self, event):
        if event.receiver is None:
            return
        self.reply_address = event.receiver.remote_source.address
        print(f"[attempt] node={self.reply_address} messageId={self.message_id}",
              flush=True)

    def on_sendable(self, event):
        if self.sent or not self.reply_address or not event.sender.credit:
            return
        properties = {"x-conformance-connection": MARKER}
        if ORIGINATOR:
            # Self-asserted, audit only. A11.2 forbids keying on it, and
            # the spoofing scenario proves the responder does not.
            properties["x-originator"] = ORIGINATOR
        msg = Message(
            body=json.dumps({
                "kind": "setPlan",
                "vmsId": "VMS-1",
                "planNumber": PLAN,
                "commandCorrelationId": CORRELATION,
            }),
            id=self.message_id,
            reply_to=self.reply_address,
            properties=properties,
        )
        event.sender.send(msg)
        self.sent = True

    def on_message(self, event):
        msg = event.message
        # Each attempt must be answered on its own node with its own
        # correlation-id, even when the *result* was produced for an
        # earlier attempt.
        if msg.address != self.reply_address:
            self.mis_addressed += 1
        if msg.correlation_id != self.message_id:
            self.mis_correlated += 1
        body = json.loads(bytes(msg.body).decode() if isinstance(
            msg.body, (bytes, bytearray, memoryview)) else msg.body)
        status = body.get("status", "success")
        self.statuses.append(status)
        print(f"[response] {status} {json.dumps(body)}", flush=True)
        if len(self.statuses) >= EXPECTED:
            print(
                f"[idem] node={self.reply_address} messageId={self.message_id} "
                f"statuses={','.join(self.statuses)} "
                f"misAddressed={self.mis_addressed} "
                f"misCorrelated={self.mis_correlated}",
                flush=True,
            )
            event.connection.close()


def main() -> int:
    Container(Attempt(SERVER)).run()
    return 0


if __name__ == "__main__":
    sys.exit(main())
