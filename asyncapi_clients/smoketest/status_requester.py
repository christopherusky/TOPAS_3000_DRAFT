# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/status_requester.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-11-11
# Version:     1.0
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""A Data Subscriber that sends ``requestStatus`` and records what follows.

Attaches to ``traffic-signal.measurement`` and to a dynamic reply node, then
sends ``{"kind": "requestStatus"}`` on ``traffic-signal.command`` with that
node as ``reply-to``. It collects every command response (reply node) and every
state message (data channel) for a short window, and judges nothing: it prints
one ``[status-requester-result] {json}`` line and ``a13_request_status.py``
decides whether the exchange was conformant.

Environment: ``STATUS_TIMEOUT`` seconds (default 4), ``STATUS_SIGNAL`` (default TS-0001).
"""
from __future__ import annotations

import json
import os
import random
import sys
import uuid
from datetime import datetime, timezone

from proton import Message
from proton.handlers import MessagingHandler
from proton.reactor import Container

SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
COMMAND_CHANNEL = "traffic-signal.command"
DATA_CHANNEL = "traffic-signal.measurement"
WINDOW = float(os.environ.get("STATUS_TIMEOUT", "4"))
SIGNAL = os.environ.get("STATUS_SIGNAL", "TS-0001")


class Requester(MessagingHandler):
    def __init__(self) -> None:
        super().__init__()
        self.command_correlation_id = random.randint(1, 2**31)
        self.message_id = str(uuid.uuid4())
        self.reply_address: str | None = None
        self.data_ready = False
        self.responses: list[dict] = []
        self.states: list[dict] = []
        self.sent = False

    def on_start(self, event):
        conn = event.container.connect(SERVER)
        event.container.create_receiver(conn, DATA_CHANNEL)
        event.container.create_receiver(conn, None, dynamic=True)
        self.sender = event.container.create_sender(conn, COMMAND_CHANNEL)
        event.container.schedule(WINDOW, self)

    def on_link_opened(self, event):
        if event.receiver is None:
            return
        address = event.receiver.remote_source.address
        if event.receiver.remote_source.dynamic:
            self.reply_address = address
        else:
            self.data_ready = True
        self._try_send()

    def on_sendable(self, event):
        self._try_send()

    def _try_send(self) -> None:
        if self.sent or not (self.reply_address and self.data_ready and self.sender.credit):
            return
        msg = Message(
            id=self.message_id, reply_to=self.reply_address,
            body=json.dumps({
                "kind": "requestStatus",
                "trafficSignalId": SIGNAL,
                "commandCorrelationId": self.command_correlation_id,
                "requestTime": datetime.now(timezone.utc).isoformat(),
            }),
        )
        msg.address = COMMAND_CHANNEL
        self.sender.send(msg)
        self.sent = True

    def on_message(self, event):
        msg = event.message
        body = msg.body.decode("utf-8") if isinstance(msg.body, (bytes, memoryview)) else msg.body
        item = {
            "properties": dict(msg.properties or {}),
            "correlation_id": None if msg.correlation_id is None else str(msg.correlation_id),
            "to": msg.address,
            "body": json.loads(body),
        }
        (self.states if event.link.source.address == DATA_CHANNEL else self.responses).append(item)

    def on_timer_task(self, event):
        print("[status-requester-result] " + json.dumps({
            "replyAddress": self.reply_address,
            "messageId": self.message_id,
            "commandCorrelationId": self.command_correlation_id,
            "signal": SIGNAL,
            "responses": self.responses,
            "states": self.states,
        }), flush=True)
        event.connection.close() if hasattr(event, "connection") and event.connection else None
        os._exit(0)


if __name__ == "__main__":
    Container(Requester()).run()
