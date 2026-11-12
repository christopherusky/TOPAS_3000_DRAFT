# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/reply_node_noise.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Send command responses to somewhere that is not the requester under test.

The negative half of the A11 isolation proof. While a requester waits on
its private reply node, this sends well-formed command responses to a
*different* reply address and to the legacy shared response channel.

Under A4 both would have reached every subscriber, which is why the old
tests filtered on arrival. Under A11 the requester must never be delivered
either — so the harness asserts `foreign == 0` rather than asserting the
requester coped.
"""
from __future__ import annotations

import json
import os
import sys
import uuid
from datetime import datetime, timezone

from proton import Message
from proton.handlers import MessagingHandler
from proton.reactor import Container

SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
LEGACY_RESPONSE_CHANNEL = "vms.command-response"
# A reply address belonging to nobody in this run.
DECOY_NODE = os.environ.get("TOPAS_DECOY_NODE", f"decoy-reply-{uuid.uuid4()}")
COUNT = int(os.environ.get("TOPAS_NOISE_COUNT", "5"))


class Noise(MessagingHandler):
    def __init__(self, url: str) -> None:
        super().__init__()
        self.url = url
        self.sent = 0

    def on_start(self, event):
        conn = event.container.connect(self.url)
        # Anonymous sender so one link can address both destinations.
        self.sender = event.container.create_sender(conn, None)

    def on_sendable(self, event):
        while event.sender.credit and self.sent < COUNT:
            frame = {
                "vmsId": "VMS-DECOY",
                "status": "success",
                "responseTime": datetime.now(timezone.utc).isoformat(),
                "echo": {"kind": "DisplayMessage", "commandCorrelationId": 999999},
            }
            # Alternate between a decoy reply node and the legacy shared
            # channel: neither should reach a requester's private node.
            target = DECOY_NODE if self.sent % 2 == 0 else LEGACY_RESPONSE_CHANNEL
            msg = Message(
                body=json.dumps(frame),
                correlation_id=str(uuid.uuid4()),
                properties={"x-originator": "not-the-requester"},
            )
            msg.address = target
            event.sender.send(msg)
            self.sent += 1
        if self.sent >= COUNT:
            print(f"[noise] sent {self.sent} foreign responses", flush=True)
            event.connection.close()


def main() -> int:
    Container(Noise(SERVER)).run()
    return 0


if __name__ == "__main__":
    sys.exit(main())
