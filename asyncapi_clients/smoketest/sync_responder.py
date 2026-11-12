# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/sync_responder.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-05
# Version:     1.0
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Mock Data Publisher that answers ``detector.sync-request`` (A12).

Holds three detectors' current measurement and, for each ``SyncRequest``,
sends the matching ones as a sync batch to the request's ``reply-to`` and
closes it with a ``SyncComplete`` to the same node: exactly the behaviour
*State Synchronization* specifies. The batch is built with
``envelope.data_headers``, so it cannot carry a non-conformant envelope.

``SYNC_DEFECT`` makes it break one rule, so that ``a12_sync.py`` can prove its
checks fire on each (a clean run against a defective responder is a failure):

    broadcast        batch goes to the data channel, not the reply node
    wrong-total      x-sync-total says one more than was sent
    no-request-id    x-sync-request-id is left off
    wrong-request-id x-sync-request-id echoes the wrong request
    wrong-corr       correlation-id does not mirror the request's message-id
    bad-sequence     every frame claims position 1
    complete-early   SyncComplete is sent before the last object
    wrong-count      SyncComplete.objectCount is one too many
    wrong-sync-id    SyncComplete.syncId is not the batch's x-sync-id
    no-complete      SyncComplete is never sent

``SYNC_REJECT=1`` makes it decline every request: a ``rejected`` SyncComplete
and no batch.
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

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE.parents[1]))

from envelope import data_headers  # noqa: E402
from topas_3000_model.common.sync import SyncComplete, SyncRequest, SyncStatus  # noqa: E402

SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
REQUEST_CHANNEL = "detector.sync-request"
DATA_CHANNEL = "detector.measurement"
DEFECT = os.environ.get("SYNC_DEFECT", "")
REJECT = os.environ.get("SYNC_REJECT") == "1"
OBJECTS = ("DET-0001", "DET-0002", "DET-0003")


def _measurement(detector_id: str) -> dict:
    return {
        "kind": "flow",
        "detectorId": detector_id,
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "totalFlow": 10 + int(detector_id[-1]),
        "periodSeconds": 300,
    }


def _matching(request: SyncRequest) -> list[str]:
    """Which of the held objects the request asks for."""
    if request.categories is not None and "measurement" not in {c.value for c in request.categories}:
        return []
    ids = list(OBJECTS)
    if request.object_ids is not None:
        ids = [i for i in ids if i in request.object_ids]
    return ids


class SyncResponder(MessagingHandler):
    def __init__(self) -> None:
        super().__init__()
        self.sender = None

    def on_start(self, event):
        conn = event.container.connect(SERVER)
        event.container.create_receiver(conn, REQUEST_CHANNEL)
        # Anonymous sender: the destination travels on each message's `to`.
        self.sender = event.container.create_sender(conn, None)
        print("[responder-ready] python sync", flush=True)

    def _send(self, address: str, body: str, correlation_id, properties=None) -> None:
        msg = Message(body=body, correlation_id=correlation_id, properties=properties or {})
        msg.address = address
        self.sender.send(msg)

    def on_message(self, event):
        msg = event.message
        reply_to = msg.reply_to
        if not reply_to:
            print("[responder] FATAL: sync request carried no reply-to", file=sys.stderr, flush=True)
            event.connection.close()
            return
        body = msg.body.decode("utf-8") if isinstance(msg.body, (bytes, memoryview)) else msg.body
        request = SyncRequest.model_validate_json(body)
        corr = "wrong" if DEFECT == "wrong-corr" else msg.id

        if REJECT:
            done = SyncComplete(
                sync_correlation_id=request.sync_correlation_id,
                status=SyncStatus.REJECTED,
                object_count=0,
                completed_time=datetime.now(timezone.utc),
                message="rate limited",
            )
            self._send(reply_to, done.model_dump_json(), corr)
            print(f"[responder] rejected -> {reply_to}", flush=True)
            return

        ids = _matching(request)
        n = len(ids)
        sync_id = str(uuid.uuid4()) if n else None
        batch_to = DATA_CHANNEL if DEFECT == "broadcast" else reply_to
        request_id = request.sync_correlation_id
        if DEFECT == "wrong-request-id":
            request_id += 1

        def complete() -> None:
            count = n + 1 if DEFECT == "wrong-count" else n
            done_sync_id = str(uuid.uuid4()) if DEFECT == "wrong-sync-id" and n else sync_id
            done = SyncComplete(
                sync_correlation_id=request.sync_correlation_id,
                sync_id=done_sync_id,
                status=SyncStatus.SUCCESS,
                object_count=count,
                completed_time=datetime.now(timezone.utc),
            )
            self._send(reply_to, done.model_dump_json(), corr)

        for i, detector_id in enumerate(ids, start=1):
            if DEFECT == "complete-early" and i == n:
                complete()
            props = data_headers(
                DATA_CHANNEL, "sync",
                sync_id=sync_id,
                sequence=1 if DEFECT == "bad-sequence" else i,
                total=n + 1 if DEFECT == "wrong-total" else n,
                request_id=None if DEFECT == "no-request-id" else request_id,
            )
            self._send(batch_to, json.dumps(_measurement(detector_id)), corr, props)
        if DEFECT not in ("complete-early", "no-complete"):
            complete()
        print(f"[responder] {n} object(s) + SyncComplete -> {reply_to} (defect={DEFECT or 'none'})", flush=True)


if __name__ == "__main__":
    Container(SyncResponder()).run()
