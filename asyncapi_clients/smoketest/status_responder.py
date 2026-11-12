# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/status_responder.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-11-11
# Version:     1.0
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Mock Data Publisher that answers ``requestStatus`` on ``traffic-signal.command``.

*Requesting status* says the publisher acknowledges the command on the
requester's reply node (``success``, or ``notSupported`` if it does not
implement it) and then publishes the equipment's current state as an
**ordinary update**: ``traffic-signal.measurement``, ``x-message-mode: delta``.
Other commands are answered ``notSupported``, as *Optional commands* requires.

``STATUS_DEFECT`` breaks one rule so ``a13_request_status.py`` can prove its
checks fire on each:

    no-state         acknowledges, never publishes the state
    as-sync publishes the state as a one-object ``sync`` (the earlier design)
    wrong-object     publishes the state of a different signal
    state-to-reply   sends the state to the requester's reply node, not the data channel
    ack-broadcast    sends the acknowledgement to the data channel, not the reply node
    wrong-corr       response correlation-id does not mirror the request's message-id
    wrong-cmd-corr   response commandCorrelationId is not the command's
    no-ack           publishes the state, never answers the command
    bad-status       acknowledges with ``failed``

``STATUS_UNSUPPORTED=1`` makes it decline: ``notSupported`` and no state.
"""
from __future__ import annotations

import json
import os
import sys
from datetime import datetime, timezone
from pathlib import Path

from proton import Message
from proton.handlers import MessagingHandler
from proton.reactor import Container

HERE = Path(__file__).resolve().parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(HERE.parents[1]))

from envelope import data_headers  # noqa: E402
from topas_3000_model.traffic_signal.command import RequestStatusCommand  # noqa: E402
from topas_3000_model.traffic_signal.measurement import TrafficSignalDynamicState  # noqa: E402
from topas_3000_model.traffic_signal.response import TrafficSignalCommandResponse  # noqa: E402
from topas_3000_model.common.command import CommandResponseStatus  # noqa: E402

SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
COMMAND_CHANNEL = "traffic-signal.command"
DATA_CHANNEL = "traffic-signal.measurement"
DEFECT = os.environ.get("STATUS_DEFECT", "")
UNSUPPORTED = os.environ.get("STATUS_UNSUPPORTED") == "1"


def _state(signal_id: str) -> dict:
    """The equipment's current state, as the publisher holds it."""
    return {
        "trafficSignalId": signal_id,
        "timestamp": datetime.now(timezone.utc).isoformat(),
        "trafficSignalMode": "centralMode",
        "trafficSignalSubMode": "utcFixedTime",
        "planLine": {"planNumber": 12, "planStartTime": datetime.now(timezone.utc).isoformat()},
    }


class StatusResponder(MessagingHandler):
    def __init__(self) -> None:
        super().__init__()
        self.sender = None

    def on_start(self, event):
        conn = event.container.connect(SERVER)
        event.container.create_receiver(conn, COMMAND_CHANNEL)
        self.sender = event.container.create_sender(conn, None)  # anonymous: `to` per message
        print("[responder-ready] python request-status", flush=True)

    def _send(self, address: str, body: dict, correlation_id=None, properties=None) -> None:
        msg = Message(body=json.dumps(body), correlation_id=correlation_id, properties=properties or {})
        msg.address = address
        self.sender.send(msg)

    def on_message(self, event):
        msg = event.message
        reply_to = msg.reply_to
        if not reply_to:
            print("[responder] FATAL: command carried no reply-to", file=sys.stderr, flush=True)
            event.connection.close()
            return
        raw = msg.body.decode("utf-8") if isinstance(msg.body, (bytes, memoryview)) else msg.body
        command = json.loads(raw)
        kind = command.get("kind")
        corr = "wrong" if DEFECT == "wrong-corr" else msg.id
        signal_id = command["trafficSignalId"]
        cmd_corr = command["commandCorrelationId"] + (1 if DEFECT == "wrong-cmd-corr" else 0)

        supported = kind == "requestStatus" and not UNSUPPORTED
        status = CommandResponseStatus.SUCCESS if supported else CommandResponseStatus.NOT_SUPPORTED
        if DEFECT == "bad-status":
            status = CommandResponseStatus.FAILED
        # Validate against the model, so the harness cannot send a malformed response.
        ack = TrafficSignalCommandResponse(
            traffic_signal_id=signal_id,
            command_correlation_id=cmd_corr,
            status=status,
            response_time=datetime.now(timezone.utc),
        )
        if DEFECT != "no-ack":
            to = DATA_CHANNEL if DEFECT == "ack-broadcast" else reply_to
            self._send(to, json.loads(ack.model_dump_json()), corr)

        if supported and DEFECT != "no-state":
            RequestStatusCommand.model_validate(command)
            state = _state("TS-9999" if DEFECT == "wrong-object" else signal_id)
            TrafficSignalDynamicState.model_validate(state)
            if DEFECT == "as-sync":
                props = data_headers(DATA_CHANNEL, "sync", sync_id=str(__import__("uuid").uuid4()),
                                     sequence=1, total=1)
            else:
                props = data_headers(DATA_CHANNEL, "delta")
            self._send(reply_to if DEFECT == "state-to-reply" else DATA_CHANNEL, state, None, props)
        print(f"[responder] {kind} -> {status.value} (defect={DEFECT or 'none'})", flush=True)


if __name__ == "__main__":
    Container(StatusResponder()).run()
