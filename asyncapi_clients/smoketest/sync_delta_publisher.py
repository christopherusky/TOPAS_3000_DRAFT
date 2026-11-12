"""Publish a sync batch followed by deltas on the configured S4 channel.

File:        smoketest/sync_delta_publisher.py
Origin:      Hand-written for asyncapi-clients smoke testing
Changes:     Hand-edit allowed — this is a test harness, not generated output
Author:      Christopher Kettell, TRL Software
Date:        2026-10-04
License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)

Exercises S4 (sync-vs-delta consumer). Emits a 3-message sync batch
sharing one `x-sync-id` and seq=1..3 / total=3, then 2 lone delta
messages with no sync headers — same shape an integrator would see
on first connect (sync snapshot) followed by steady-state telemetry.

The S4 consumer should:
  * group the three sync messages under the shared sync-id and
    declare "[sync ... complete]" once it has seen all 3
  * print each delta message as it arrives
"""
from __future__ import annotations

import json
import os
import sys
import uuid
from datetime import datetime, timezone

from pathlib import Path

from proton import Message
from proton.handlers import MessagingHandler
from proton.reactor import Container

sys.path.insert(0, str(Path(__file__).resolve().parent))
from envelope import data_headers  # noqa: E402

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from bin.scope import load_scope  # noqa: E402

# The channel comes from scope.toml ([smoke] sync_delta_channel), so the scenario
# always runs against a domain that is live. S4 reads the message mode and the
# sync headers, not the payload.
CHANNEL = load_scope().smoke["sync_delta_channel"]
SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
ORIGINATOR = os.environ.get("TOPAS_ORIGINATOR", "sync-delta-publisher")
SYNC_BATCH = 3
DELTA_COUNT = 2


def _payload(seq: int) -> dict:
    # A valid detector flow reading, the shape the default channel
    # (detector.measurement) carries.
    return {
        "kind": "flow",
        "detectorId": f"DET-{seq:03d}",
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "totalFlow": 10 + seq,
        "periodSeconds": 300,
    }


class SyncThenDelta(MessagingHandler):
    def __init__(self, url: str) -> None:
        super().__init__()
        self.url = url
        self.sender = None
        self.sent = 0
        self.confirmed = 0
        self.total = SYNC_BATCH + DELTA_COUNT
        self.sync_id = str(uuid.uuid4())

    def on_start(self, event):
        conn = event.container.connect(self.url)
        self.sender = event.container.create_sender(conn, CHANNEL)

    def on_sendable(self, event):
        while event.sender.credit and self.sent < self.total:
            seq_in_batch = self.sent + 1
            if seq_in_batch <= SYNC_BATCH:
                # A publisher-initiated republication: sync mode, no x-sync-request-id.
                props = data_headers(
                    CHANNEL, "sync",
                    sync_id=self.sync_id, sequence=seq_in_batch, total=SYNC_BATCH,
                )
            else:
                props = data_headers(CHANNEL, "delta")
            msg = Message(
                body=json.dumps(_payload(seq_in_batch)),
                properties=props,
                id=str(uuid.uuid4()),
            )
            msg.address = CHANNEL  # see fault_publisher.py for why
            event.sender.send(msg)
            self.sent += 1

    def on_accepted(self, event):
        self.confirmed += 1
        if self.confirmed >= self.total:
            print(
                f"[sync-delta-publisher] sync-id={self.sync_id} "
                f"sent {SYNC_BATCH} sync + {DELTA_COUNT} delta, all accepted",
                flush=True,
            )
            event.connection.close()


def main() -> int:
    Container(SyncThenDelta(SERVER)).run()
    return 0


if __name__ == "__main__":
    sys.exit(main())
