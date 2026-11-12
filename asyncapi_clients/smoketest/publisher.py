"""Publish N synthetic detector.measurement frames so S1 has data to consume.

File:        smoketest/publisher.py
Origin:      Hand-written for asyncapi-clients smoke testing
Changes:     Hand-edit allowed — this is a test harness, not generated output
Author:      Christopher Kettell, TRL Software
Date:        2026-05-02
License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)

Run this in one terminal, then run
`uv run python asyncapi_clients/output/python/snippets/s1.py` in another
(both pointed at TOPAS_AMQP_URL=amqp://localhost:5672) to verify the
end-to-end envelope conventions.
"""
from __future__ import annotations

import json
import os
from pathlib import Path
import sys
import uuid
from datetime import datetime, timezone

from proton import Message
from proton.handlers import MessagingHandler
from proton.reactor import Container

sys.path.insert(0, str(Path(__file__).resolve().parent))
from envelope import data_headers  # noqa: E402


CHANNEL = os.environ.get("TOPAS_CHANNEL", "detector.measurement")
SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
N_MESSAGES = int(os.environ.get("TOPAS_N_MESSAGES", "5"))
ORIGINATOR = os.environ.get("TOPAS_ORIGINATOR", "smoketest-publisher")


def _payload(seq: int) -> dict:
    """Synthetic detector.measurement payload — shape only, not schema-validated."""
    return {
        "detectorId": f"DET-{seq:04d}",
        "occupancy": 0.10 + 0.05 * seq,
        "flow": 100 + 10 * seq,
        "speed": 50.0 + seq,
        "timestamp": datetime.now(timezone.utc).isoformat(),
    }


class Publisher(MessagingHandler):
    def __init__(self, url: str, channel: str, n: int) -> None:
        super().__init__()
        self.url = url
        self.channel = channel
        self.n = n
        self.sent = 0
        self.confirmed = 0

    def on_start(self, event):
        conn = event.container.connect(self.url)
        self.sender = event.container.create_sender(conn, self.channel)

    def on_sendable(self, event):
        while event.sender.credit and self.sent < self.n:
            msg = Message(
                body=json.dumps(_payload(self.sent + 1)),
                properties=data_headers(self.channel, "delta"),
                id=str(uuid.uuid4()),
            )
            event.sender.send(msg)
            self.sent += 1

    def on_accepted(self, event):
        self.confirmed += 1
        if self.confirmed >= self.n:
            print(f"[publisher] {self.confirmed} messages accepted by broker", flush=True)
            event.connection.close()


def main() -> int:
    Container(Publisher(SERVER, CHANNEL, N_MESSAGES)).run()
    return 0


if __name__ == "__main__":
    sys.exit(main())
