"""Spray synthetic fault frames across multiple device-domain channels.

File:        smoketest/fault_publisher.py
Origin:      Hand-written for asyncapi-clients smoke testing
Changes:     Hand-edit allowed — this is a test harness, not generated output
Author:      Christopher Kettell, TRL Software
Date:        2026-10-04
License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)

Lets the S2 snippet (wildcard `*.fault` subscriber) actually receive
something. Publishes one fault to each live domain named in scope.toml
([smoke] fault_domains: by default detector.fault, vms.fault and
traffic-signal.fault) by opening one sender per address — broker-side wildcard
matching then fans them all into the wildcard receiver's queue.
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


SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
ORIGINATOR = os.environ.get("TOPAS_ORIGINATOR", "fault-publisher")
# The default domains come from scope.toml ([smoke] fault_domains), so S2 only
# ever sprays to live domains. TOPAS_FAULT_DOMAINS overrides it for a local run.
DOMAINS = os.environ.get(
    "TOPAS_FAULT_DOMAINS", ",".join(load_scope().smoke["fault_domains"])
).split(",")


def _payload(domain: str) -> dict:
    return {
        "deviceId": f"{domain.upper()}-001",
        "severity": "WARNING",
        "code": "TEST_FAULT",
        "message": f"smoke-test fault from {domain} domain",
        "timestamp": datetime.now(timezone.utc).isoformat(),
    }


class FaultSpray(MessagingHandler):
    def __init__(self, url: str, domains: list[str]) -> None:
        super().__init__()
        self.url = url
        self.domains = domains
        self.senders: list = []
        self.sent_per_sender: dict = {}
        self.confirmed = 0

    def on_start(self, event):
        conn = event.container.connect(self.url)
        for domain in self.domains:
            channel = f"{domain}.fault"
            sender = event.container.create_sender(conn, channel)
            self.senders.append(sender)

    def on_sendable(self, event):
        sender = event.sender
        if self.sent_per_sender.get(sender):
            return
        channel = sender.target.address  # e.g. "detector.fault"
        domain = channel.split(".")[0]
        msg = Message(
            body=json.dumps(_payload(domain)),
            properties=data_headers(channel, "delta"),
            id=str(uuid.uuid4()),
        )
        # qpid-proton does not auto-populate the AMQP `to` field from
        # the sender's target — set it explicitly so wildcard receivers
        # (S2) can recover the original publish address from
        # `event.message.address`.
        msg.address = channel
        sender.send(msg)
        self.sent_per_sender[sender] = True

    def on_accepted(self, event):
        self.confirmed += 1
        if self.confirmed >= len(self.domains):
            print(
                f"[fault-publisher] {self.confirmed} faults accepted across "
                f"{len(self.domains)} domains: {','.join(self.domains)}",
                flush=True,
            )
            event.connection.close()


def main() -> int:
    Container(FaultSpray(SERVER, DOMAINS)).run()
    return 0


if __name__ == "__main__":
    sys.exit(main())
