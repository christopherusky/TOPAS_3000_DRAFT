"""Publish one of each network object so S7 can build a full topology.

File:        smoketest/network_publisher.py
Origin:      Hand-written for asyncapi-clients smoke testing
Changes:     Hand-edit allowed — this is a test harness, not generated output
Author:      Christopher Kettell, TRL Software
Date:        2026-05-02
License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)

S7 opens five receivers (one per network object type: node, link,
zone, path, turn) and disconnects when each has produced its first
message. This publisher emits exactly one typed object per channel
to drive S7 to its disconnection condition.
"""
from __future__ import annotations

import os
from pathlib import Path
import sys
import uuid

from proton import Message
from proton.handlers import MessagingHandler
from proton.reactor import Container

sys.path.insert(0, str(Path(__file__).resolve().parent))
from envelope import data_headers  # noqa: E402

from asyncapi_clients.output.python.sdk.network.network_link_v1_0 import NetworkLink
from asyncapi_clients.output.python.sdk.network.network_node_v1_0 import NetworkNode
from asyncapi_clients.output.python.sdk.network.network_path_v1_0 import NetworkPath
from asyncapi_clients.output.python.sdk.network.network_turn_v1_0 import NetworkTurn
from asyncapi_clients.output.python.sdk.network.network_zone_v1_0 import NetworkZone


SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
ORIGINATOR = os.environ.get("TOPAS_ORIGINATOR", "network-publisher")


def _build():
    return {
        "network.node": NetworkNode(nodeId="N-001", description="junction-A"),
        "network.link": NetworkLink(
            linkId="L-001", fromNodeId="N-001", toNodeId="N-002",
            description="A->B link",
        ),
        "network.zone": NetworkZone(
            zoneId="Z-001", typeId="1", linkIds=["L-001"],
        ),
        "network.path": NetworkPath(
            pathId="P-001", typeId="2", linkIds=["L-001"],
        ),
        "network.turn": NetworkTurn(
            turnId="T-001", fromLinkId="L-001", toLinkId="L-001", allowed=True,
        ),
    }


class NetworkPub(MessagingHandler):
    """Uses an anonymous-relay sender (target=None) so a single link can
    publish to all five network.* addresses by setting msg.address per
    message. Multiple per-address senders worked unreliably against the
    Artemis 2.31 default config (some links never got credit); the
    anonymous-relay idiom is the canonical AMQP 1.0 multi-destination
    pattern and avoids the per-link credit race.
    """

    def __init__(self, url: str) -> None:
        super().__init__()
        self.url = url
        self.objects = _build()
        self.channels = list(self.objects.keys())
        self.sync_id = str(uuid.uuid4())
        self.sender = None
        self.sent = 0
        self.confirmed = 0

    def on_start(self, event):
        conn = event.container.connect(self.url)
        self.sender = event.container.create_sender(conn, None)

    def on_sendable(self, event):
        while event.sender.credit and self.sent < len(self.channels):
            channel = self.channels[self.sent]
            obj = self.objects[channel]
            # One sync batch across the domain's categories: a single x-sync-id and
            # a 1..N position on every message (A5), so the batch is identifiable.
            msg = Message(
                body=obj.model_dump_json(),
                properties=data_headers(
                    channel, "sync",
                    sync_id=self.sync_id,
                    sequence=self.sent + 1,
                    total=len(self.channels),
                ),
                id=str(uuid.uuid4()),
            )
            msg.address = channel
            event.sender.send(msg)
            self.sent += 1

    def on_accepted(self, event):
        self.confirmed += 1
        if self.confirmed >= len(self.channels):
            print(
                f"[network-publisher] {self.confirmed} typed network objects "
                f"accepted across {len(self.channels)} channels",
                flush=True,
            )
            event.connection.close()


def main() -> int:
    Container(NetworkPub(SERVER)).run()
    return 0


if __name__ == "__main__":
    sys.exit(main())
