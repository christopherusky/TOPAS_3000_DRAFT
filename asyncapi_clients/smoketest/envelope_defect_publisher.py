# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/envelope_defect_publisher.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-05
# Version:     1.0
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Publish deliberately non-conformant data messages, one defect each.

A check that cannot fail is not a check. ``envelope_conformance.py``
runs this and requires every defect to be reported; a clean run here is a
failure of the checker. It writes raw properties on purpose and does not use
``envelope.data_headers``, which refuses to build a bad set.
"""
from __future__ import annotations

import json
import os
import sys
import uuid

from proton import Message
from proton.handlers import MessagingHandler
from proton.reactor import Container

SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
CHANNEL = "detector.measurement"
GOOD_SYNC_ID = str(uuid.uuid4())

DEFECTS = {
    "missing-content-type": {"x-message-mode": "delta"},
    "missing-mode": {"x-content-type": f"{CHANNEL}-v1.0"},
    "originator-on-data": {
        "x-message-mode": "delta", "x-content-type": f"{CHANNEL}-v1.0", "x-originator": "someone",
    },
    "wrong-channel-in-content-type": {
        "x-message-mode": "delta", "x-content-type": "vms.measurement-v1.0",
    },
    "sync-without-triple": {"x-message-mode": "sync", "x-content-type": f"{CHANNEL}-v1.0"},
    "sync-id-not-a-uuid": {
        "x-message-mode": "sync", "x-content-type": f"{CHANNEL}-v1.0",
        "x-sync-id": "SYNC-1a2b3c4d", "x-sync-sequence": 1, "x-sync-total": 1,
    },
    "sequence-beyond-total": {
        "x-message-mode": "sync", "x-content-type": f"{CHANNEL}-v1.0",
        "x-sync-id": GOOD_SYNC_ID, "x-sync-sequence": 3, "x-sync-total": 2,
    },
    "request-id-on-delta": {
        "x-message-mode": "delta", "x-content-type": f"{CHANNEL}-v1.0", "x-sync-request-id": 5,
    },
    "geo-tile-without-system": {
        "x-message-mode": "delta", "x-content-type": f"{CHANNEL}-v1.0", "x-geo-tile": "gcpvj0",
    },
}


class DefectPublisher(MessagingHandler):
    def __init__(self) -> None:
        super().__init__()
        self.sent = 0
        self.confirmed = 0
        self.items = list(DEFECTS.items())

    def on_start(self, event):
        self.sender = event.container.create_sender(event.container.connect(SERVER), CHANNEL)

    def on_sendable(self, event):
        while event.sender.credit and self.sent < len(self.items):
            _name, props = self.items[self.sent]
            event.sender.send(Message(body=json.dumps({"detectorId": "D"}), properties=dict(props)))
            self.sent += 1

    def on_accepted(self, event):
        self.confirmed += 1
        if self.confirmed >= len(self.items):
            print(f"[defect-publisher] {self.confirmed} defective messages accepted", flush=True)
            event.connection.close()


if __name__ == "__main__":
    Container(DefectPublisher()).run()
    sys.exit(0)
