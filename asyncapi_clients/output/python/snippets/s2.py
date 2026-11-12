"""
File:        output/python/snippets/s2.py
Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py
Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate
Author:      Christopher Kettell, TRL Software
Date:        2026-11-12
Generator:   asyncapi_clients/generator v0.1.0
License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
"""
"""Scenario S2 — Wildcard subscription across a domain.

Demonstrates: subscribing to *.fault (every
fault from every device domain) using AMQP 1.0 source filters.
Teaches A8 (channel naming convention).
"""
import json
import os
import sys

from proton.handlers import MessagingHandler
from proton.reactor import Container


PATTERN = "*.fault"
SERVER = os.environ.get(
    "TOPAS_AMQP_URL",
    "amqps://{host}:{port}".format(host="localhost", port="5671"),
)
MAX_MESSAGES = int(os.environ.get("TOPAS_MAX_MESSAGES", "5"))


class FaultAggregator(MessagingHandler):
    def __init__(self, url: str, pattern: str) -> None:
        super().__init__()
        self.url = url
        self.pattern = pattern
        self.received = 0

    def on_start(self, event):
        conn = event.container.connect(self.url)
        # AMQP 1.0 source filter for topic-style wildcard subscription.
        # Broker-specific syntax may differ (Artemis: `*.fault`,
        # Qpid Dispatch: `mc/*.fault`); adjust to your deployment.
        event.container.create_receiver(conn, self.pattern)

    def on_link_opened(self, event):
        # Readiness handshake: the harness waits for this before
        # starting the publisher. Every publisher sends a fixed burst and
        # exits, so anything published before this link is attached is
        # discarded by the broker and the subscriber then waits for messages
        # that no longer exist. on_link_opened fires when the peer confirms
        # the attach, which is the earliest point it is safe to publish.
        if event.receiver is None:
            return
        self._links_open = getattr(self, "_links_open", 0) + 1
        if self._links_open == 1:
            print("[ready]", file=sys.stderr, flush=True)

    def on_message(self, event):
        payload = json.loads(event.message.body)
        domain = (event.message.address or self.pattern).split(".")[0]
        print(f"[fault from {domain}] {payload}", flush=True)
        self.received += 1
        if self.received >= MAX_MESSAGES:
            event.connection.close()


def main() -> int:
    Container(FaultAggregator(SERVER, PATTERN)).run()
    return 0


if __name__ == "__main__":
    sys.exit(main())
