# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/envelope_tap.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-05
# Version:     1.0
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Subscribe to every data channel and print each message's properties.

A tap, not a judge: it prints one ``[tap] {json}`` line per message and
``[tap-ready]`` once every receiver is attached, and leaves the verdict to
``envelope_conformance.py``. It attaches one receiver per channel (rather than
a wildcard) because a message's ``to`` is only populated if the publisher
chose to set it; the channel is the link it arrived on.

The data channels are those the AsyncAPI document says a Data Publisher
sends on (``x-sender: data-publisher``), so a new domain is tapped without an
edit here.
"""
from __future__ import annotations

import json
import os
import sys
from pathlib import Path

import yaml
from proton.handlers import MessagingHandler
from proton.reactor import Container

ROOT = Path(__file__).resolve().parents[2]
SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")


def data_channels() -> list[str]:
    doc = yaml.safe_load((ROOT / "asyncapi.yaml").read_text())
    return sorted(
        name for name, chan in doc["channels"].items() if chan.get("x-sender") == "data-publisher"
    )


class Tap(MessagingHandler):
    def __init__(self, channels: list[str]) -> None:
        super().__init__()
        self.channels = channels
        self.opened = 0

    def on_start(self, event):
        conn = event.container.connect(SERVER)
        for channel in self.channels:
            event.container.create_receiver(conn, channel)

    def on_link_opened(self, event):
        if event.receiver is None:
            return
        self.opened += 1
        if self.opened == len(self.channels):
            print("[tap-ready]", len(self.channels), "channels", flush=True)

    def on_message(self, event):
        msg = event.message
        print(
            "[tap] "
            + json.dumps(
                {
                    "channel": event.link.source.address,
                    "properties": dict(msg.properties or {}),
                    "id": str(msg.id),
                    "has_to": bool(msg.address),
                },
                default=str,
            ),
            flush=True,
        )


def main() -> int:
    Container(Tap(data_channels())).run()
    return 0


if __name__ == "__main__":
    sys.exit(main())
