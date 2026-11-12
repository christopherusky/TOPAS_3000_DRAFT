"""Publish a mix of SCOOT/GPS/ANPR variants so S6 dispatch fires on each.

File:        smoketest/transport_link_publisher.py
Origin:      Hand-written for asyncapi-clients smoke testing
Changes:     Hand-edit allowed — this is a test harness, not generated output
Author:      Christopher Kettell, TRL Software
Date:        2026-05-02
License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)

Emits one of each TransportLinkMeasurement variant — SCOOT, GPS,
ANPR — built via the typed SDK and serialised through the
TransportLinkMeasurement RootModel envelope. Lets S6 exercise its
match-on-runtime-type dispatch (F6) against all three legs.
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

from asyncapi_clients.output.python.sdk.transport_link.transport_link_measurement_v1_0 import (
    Model as TransportLinkMeasurementEnvelope,
    TransportLinkANPRDynamic,
    TransportLinkGPSDynamic,
    TransportLinkMeasurement,
    TransportLinkSCOOTDynamic,
)


CHANNEL = "transport-link.measurement"
SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
ORIGINATOR = os.environ.get("TOPAS_ORIGINATOR", "transport-link-publisher")


def _variants():
    now = datetime.now(timezone.utc)
    return [
        TransportLinkSCOOTDynamic(
            transportLinkId="TL-SCOOT-001", timestamp=now, kind="scoot",
            currentFlowVehicles=420, occupancyPercent=18.5,
        ),
        TransportLinkGPSDynamic(
            transportLinkId="TL-GPS-002", timestamp=now, kind="gps",
            meanTravelTimeSeconds=92.0, numberOfVehicles=37,
        ),
        TransportLinkANPRDynamic(
            transportLinkId="TL-ANPR-003", timestamp=now, kind="anpr",
            numberPlatesIn=145, numberPlatesOut=139, numberPlatesMatches=128,
        ),
    ]


class TransportLinkPub(MessagingHandler):
    def __init__(self, url: str) -> None:
        super().__init__()
        self.url = url
        self.payloads = [
            TransportLinkMeasurementEnvelope(
                root=TransportLinkMeasurement(root=variant)
            ).model_dump_json()
            for variant in _variants()
        ]
        self.sender = None
        self.sent = 0
        self.confirmed = 0

    def on_start(self, event):
        conn = event.container.connect(self.url)
        self.sender = event.container.create_sender(conn, CHANNEL)

    def on_sendable(self, event):
        while event.sender.credit and self.sent < len(self.payloads):
            msg = Message(
                body=self.payloads[self.sent],
                properties=data_headers(CHANNEL, "delta"),
                id=str(uuid.uuid4()),
            )
            msg.address = CHANNEL
            event.sender.send(msg)
            self.sent += 1

    def on_accepted(self, event):
        self.confirmed += 1
        if self.confirmed >= len(self.payloads):
            print(
                f"[transport-link-publisher] {self.confirmed} variants accepted "
                f"(SCOOT+GPS+ANPR)",
                flush=True,
            )
            event.connection.close()


def main() -> int:
    Container(TransportLinkPub(SERVER)).run()
    return 0


if __name__ == "__main__":
    sys.exit(main())
