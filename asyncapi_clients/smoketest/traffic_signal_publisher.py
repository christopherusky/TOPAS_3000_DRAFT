"""Publish typed TrafficSignalDynamicState frames so S5 has data.

File:        smoketest/traffic_signal_publisher.py
Origin:      Hand-written for asyncapi-clients smoke testing
Changes:     Hand-edit allowed — this is a test harness, not generated output
Author:      Christopher Kettell, TRL Software
Date:        2026-05-02
License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)

Builds payloads via the typed SDK (`TrafficSignalDynamicState.model_dump_json`)
and emits them on `traffic-signal.measurement`. Validates the
serialize-then-deserialize round-trip end-to-end against a real
broker — if S5 receives and re-validates the payload via the same
class, both sides agree on the schema.
"""
from __future__ import annotations

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

from asyncapi_clients.output.python.sdk.traffic_signal.traffic_signal_measurement_v1_0 import (
    PlanLine,
    TrafficSignalDynamicState,
    TrafficSignalModeEnum,
    TrafficSignalSubModeEnum,
)


CHANNEL = "traffic-signal.measurement"
SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
ORIGINATOR = os.environ.get("TOPAS_ORIGINATOR", "traffic-signal-publisher")
N_MESSAGES = int(os.environ.get("TOPAS_N_MESSAGES", "5"))


def _state(seq: int) -> TrafficSignalDynamicState:
    now = datetime.now(timezone.utc)
    return TrafficSignalDynamicState(
        trafficSignalId=f"TS-{seq:04d}",
        timestamp=now,
        trafficSignalMode=TrafficSignalModeEnum.centralMode,
        trafficSignalSubMode=TrafficSignalSubModeEnum.utcFixedTime,
        planLine=PlanLine(planNumber=10 + (seq % 3), planStartTime=now),
    )


class TrafficSignalPub(MessagingHandler):
    def __init__(self, url: str) -> None:
        super().__init__()
        self.url = url
        self.sender = None
        self.sent = 0
        self.confirmed = 0

    def on_start(self, event):
        conn = event.container.connect(self.url)
        self.sender = event.container.create_sender(conn, CHANNEL)

    def on_sendable(self, event):
        while event.sender.credit and self.sent < N_MESSAGES:
            payload = _state(self.sent + 1).model_dump_json()
            msg = Message(
                body=payload,
                properties=data_headers(CHANNEL, "delta"),
                id=str(uuid.uuid4()),
            )
            msg.address = CHANNEL
            event.sender.send(msg)
            self.sent += 1

    def on_accepted(self, event):
        self.confirmed += 1
        if self.confirmed >= N_MESSAGES:
            print(
                f"[traffic-signal-publisher] {self.confirmed} typed states accepted",
                flush=True,
            )
            event.connection.close()


def main() -> int:
    Container(TrafficSignalPub(SERVER)).run()
    return 0


if __name__ == "__main__":
    sys.exit(main())
