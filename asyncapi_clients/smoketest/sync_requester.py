# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/sync_requester.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-05
# Version:     1.0
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""A Data Subscriber that requests a sync and records everything it receives.

Creates a dynamic reply node, publishes a ``SyncRequest`` on
``detector.sync-request`` with that node as ``reply-to``, and collects every
frame that arrives there. It stops a moment after the ``SyncComplete`` (so a
frame sent *after* the end of the batch is still seen), or on timeout. It
judges nothing: it prints one ``[sync-requester-result] {json}`` line and
``a12_sync.py`` decides whether the exchange was conformant.

Environment: ``SYNC_OBJECT_IDS`` (comma separated filter), ``SYNC_CATEGORIES``
(comma separated), ``SYNC_TIMEOUT`` seconds (default 10), ``SYNC_LABEL``.
"""
from __future__ import annotations

import json
import os
import random
import sys
import uuid
from datetime import datetime, timezone
from pathlib import Path

from proton import Message
from proton.handlers import MessagingHandler
from proton.reactor import Container

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))

SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
REQUEST_CHANNEL = "detector.sync-request"
TIMEOUT = float(os.environ.get("SYNC_TIMEOUT", "10"))
AFTER_COMPLETE = 1.5
LABEL = os.environ.get("SYNC_LABEL", "requester")


def _csv(name: str) -> list[str] | None:
    raw = os.environ.get(name)
    return [x for x in raw.split(",") if x] if raw is not None else None


class Requester(MessagingHandler):
    def __init__(self) -> None:
        super().__init__()
        self.correlation_id = random.randint(1, 2**31)
        self.message_id = str(uuid.uuid4())
        self.reply_address: str | None = None
        self.frames: list[dict] = []
        self.completes: list[dict] = []
        self.conn = None
        self.sent = False
        self.done = False

    def request_body(self) -> dict:
        body: dict = {
            "syncCorrelationId": self.correlation_id,
            "requestTime": datetime.now(timezone.utc).isoformat(),
        }
        if (ids := _csv("SYNC_OBJECT_IDS")) is not None:
            body["objectIds"] = ids
        if (cats := _csv("SYNC_CATEGORIES")) is not None:
            body["categories"] = cats
        return body

    def on_start(self, event):
        self.conn = event.container.connect(SERVER)
        self.receiver = event.container.create_receiver(self.conn, None, dynamic=True)
        self.sender = event.container.create_sender(self.conn, REQUEST_CHANNEL)
        event.container.schedule(TIMEOUT, self)

    def on_link_opened(self, event):
        if event.receiver is not None and not self.reply_address:
            address = event.receiver.remote_source.address
            if address:
                self.reply_address = address
                self._try_send()

    def on_sendable(self, event):
        self._try_send()

    def _try_send(self) -> None:
        """Send once the reply node exists and the request link has credit."""
        if self.sent or not self.reply_address or not self.sender.credit:
            return
        msg = Message(
            id=self.message_id, reply_to=self.reply_address,
            body=json.dumps(self.request_body()),
        )
        msg.address = REQUEST_CHANNEL
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
            "after_complete": bool(self.completes),
        }
        if "syncCorrelationId" in item["body"] and "status" in item["body"]:
            self.completes.append(item)
            event.container.schedule(AFTER_COMPLETE, self)
        else:
            self.frames.append(item)

    def on_timer_task(self, event):
        # Fired by either the overall timeout or the short post-complete wait.
        self.finish(event)

    def finish(self, event):
        if self.done:
            return
        self.done = True
        print(
            "[sync-requester-result] "
            + json.dumps(
                {
                    "label": LABEL,
                    "syncCorrelationId": self.correlation_id,
                    "messageId": self.message_id,
                    "replyAddress": self.reply_address,
                    "request": self.request_body(),
                    "frames": self.frames,
                    "completes": self.completes,
                    "timedOut": not self.completes,
                }
            ),
            flush=True,
        )
        if self.conn is not None:
            self.conn.close()
        event.container.stop()


if __name__ == "__main__":
    Container(Requester()).run()
