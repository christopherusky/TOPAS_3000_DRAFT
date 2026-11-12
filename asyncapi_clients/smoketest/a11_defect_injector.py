# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/a11_defect_injector.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Deliver deliberately defective responses to a requester's reply node.

An assertion nobody has watched fail is not evidence. Twice already a
check in this suite passed because the condition it tested could not
arise: a foreign-traffic scenario that ran after the requester had closed,
and a commandCorrelationId comparison against a field the responder never
sent. Both read as green.

So each A11 counter gets a defect that must trigger it, and the harness
asserts the counter moves. A requester that cannot detect these is not
conformant, however many valid round-trips it completes.

    mis-delivered        correlation-id belongs to someone else, and `to`
                         names another node   -> foreign      (routing)
    mis-correlated       correlation-id belongs to someone else, but `to`
                         names this node      -> wrongCorr    (transformation)
    missing-correlation  no correlation-id at all
                                              -> missingCorr  (absence)
    missing-to           no `to`              -> missingTo    (absence)
    missing-app          no commandCorrelationId
                                              -> missingApp   (absence)
    wrong-app            commandCorrelationId altered
                                              -> wrongApp     (transformation)

Delivery is by an explicitly attached link whose target is the reply node,
not by an anonymous sender. That is what makes `mis-delivered` possible to
stage at all: routing follows the link, so the `to` property can name a
different node — which is precisely the shape a broker mis-delivery has,
and the shape a requester must not accept.

Every defect carries status `pending` so the requester keeps listening;
the real responder's terminal frame is what ends the run.
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
TARGET_NODE = os.environ["TOPAS_TARGET_NODE"]
TARGET_MESSAGE_ID = os.environ["TOPAS_TARGET_MESSAGE_ID"]
APP_CORRELATION_ID = int(os.environ.get("TOPAS_TARGET_APP_ID", "0"))
DEFECTS = [d for d in os.environ.get(
    "TOPAS_DEFECTS",
    "mis-delivered,mis-correlated,missing-correlation,missing-to,"
    "missing-app,wrong-app",
).split(",") if d]
DECOY_NODE = f"decoy-reply-{uuid.uuid4()}"


def _frame(app_id: int | None) -> str:
    body = {
        "vmsId": "VMS-DEFECT",
        "status": "pending",
        "responseTime": datetime.now(timezone.utc).isoformat(),
    }
    if app_id is not None:
        body["commandCorrelationId"] = app_id
    return json.dumps(body)


def _build(defect: str) -> Message:
    """One well-formed AMQP message per defect — only the property under
    test is wrong, so a counter that moves identifies exactly one fault."""
    if defect == "mis-delivered":
        msg = Message(body=_frame(APP_CORRELATION_ID),
                      correlation_id=str(uuid.uuid4()))
        msg.address = DECOY_NODE
    elif defect == "mis-correlated":
        msg = Message(body=_frame(APP_CORRELATION_ID),
                      correlation_id=str(uuid.uuid4()))
        msg.address = TARGET_NODE
    elif defect == "missing-correlation":
        msg = Message(body=_frame(APP_CORRELATION_ID))
        msg.address = TARGET_NODE
    elif defect == "missing-to":
        msg = Message(body=_frame(APP_CORRELATION_ID),
                      correlation_id=TARGET_MESSAGE_ID)
    elif defect == "missing-app":
        msg = Message(body=_frame(None), correlation_id=TARGET_MESSAGE_ID)
        msg.address = TARGET_NODE
    elif defect == "wrong-app":
        msg = Message(body=_frame(APP_CORRELATION_ID + 1),
                      correlation_id=TARGET_MESSAGE_ID)
        msg.address = TARGET_NODE
    else:
        raise SystemExit(f"unknown defect {defect!r}")
    return msg


class Injector(MessagingHandler):
    def __init__(self, url: str) -> None:
        super().__init__()
        self.url = url
        self.sent = 0

    def on_start(self, event):
        conn = event.container.connect(self.url)
        # Link-routed, not anonymous: delivery follows the link target, so
        # `to` is free to name somewhere else.
        self.sender = event.container.create_sender(conn, TARGET_NODE)

    def on_sendable(self, event):
        while event.sender.credit and self.sent < len(DEFECTS):
            defect = DEFECTS[self.sent]
            event.sender.send(_build(defect))
            print(f"[inject] {defect}", flush=True)
            self.sent += 1
        if self.sent >= len(DEFECTS):
            print(f"[inject] sent {self.sent} defective responses", flush=True)
            event.connection.close()


def main() -> int:
    Container(Injector(SERVER)).run()
    return 0


if __name__ == "__main__":
    sys.exit(main())
