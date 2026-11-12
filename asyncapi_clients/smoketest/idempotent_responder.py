# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/idempotent_responder.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-29
# Version:     1.1
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""A stateful VMS responder that executes each command exactly once.

The routing half of A11 is proven by the isolation harness; this is the
other half. It keeps an idempotency store keyed on the authenticated
principal plus `commandCorrelationId`, and answers every attempt while
executing only the first.

The response envelope is always built from the *attempt being answered*:

    to              = that attempt's reply-to
    correlation-id  = that attempt's message-id
    payload         = the stored result

A retry arrives with a new message-id and usually a new reply node, so
replaying the stored envelope would answer a node that no longer exists.

Configuration (env):
    TOPAS_IDEMPOTENCY          "on" (default) or "off" — off disables
                               deduplication so the harness can prove its
                               own assertions fire.
    TOPAS_RESPONDER_DELAY      seconds between `pending` and the terminal
                               frame; the window a retry lands in.
    TOPAS_RESPONDER_MAX_REPLIES exit after N attempts have been answered.
    TOPAS_PRINCIPAL_MAP        JSON, connection marker -> principal.
"""
from __future__ import annotations

import json
import os
import signal
import sys
from datetime import datetime, timezone

from proton import Message
from proton.handlers import MessagingHandler
from proton.reactor import Container

from asyncapi_clients.smoketest.idempotency import (
    Attempt,
    IdempotencyConflict,
    IdempotencyStore,
    RecordState,
)
from asyncapi_clients.smoketest.principal import (
    ConformancePrincipalProvider,
    PrincipalUnavailable,
)

COMMAND_CHANNEL = "vms.command"
SERVER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
DELAY = float(os.environ.get("TOPAS_RESPONDER_DELAY", "0.0"))
MAX_REPLIES = int(os.environ.get("TOPAS_RESPONDER_MAX_REPLIES", "0")) or None
ENABLED = os.environ.get("TOPAS_IDEMPOTENCY", "on").lower() != "off"
PRINCIPAL_MAP = json.loads(os.environ.get("TOPAS_PRINCIPAL_MAP", "{}"))


def _decode(body):
    if isinstance(body, (bytes, bytearray, memoryview)):
        body = bytes(body).decode("utf-8")
    if not isinstance(body, str):
        raise TypeError(f"unhandled body representation {type(body).__name__!r}")
    return json.loads(body)


class IdempotentResponder(MessagingHandler):
    def __init__(self, url: str) -> None:
        super().__init__()
        self.url = url
        self.store = IdempotencyStore(enabled=ENABLED)
        self.principals = ConformancePrincipalProvider(PRINCIPAL_MAP)
        self.answered = 0

    def on_start(self, event):
        conn = self._connection = event.container.connect(self.url)
        event.container.create_receiver(conn, COMMAND_CHANNEL)
        # Anonymous sender: each response's `to` routes it (A11.4).
        self.sender = event.container.create_sender(conn, None)
        print(f"[responder-ready] idempotency={'on' if ENABLED else 'OFF'}",
              flush=True)

    def _send(self, attempt: Attempt, frame: dict) -> None:
        """Answer one attempt, with that attempt's transport identity."""
        reply = Message(
            body=json.dumps(frame),
            correlation_id=attempt.message_id,
        )
        reply.address = attempt.reply_to
        self.sender.send(reply)
        self.store.responses_delivered += 1
        self.answered += 1
        print(f"[responder] {frame['status']} -> {attempt.reply_to} "
              f"corr={attempt.message_id}", flush=True)

    def _frame(self, command: dict, status: str, message: str | None = None) -> dict:
        frame = {
            "vmsId": command.get("vmsId"),
            "commandCorrelationId": command.get("commandCorrelationId"),
            "status": status,
            "responseTime": datetime.now(timezone.utc).isoformat(),
        }
        if message:
            frame["message"] = message
        return frame

    def on_message(self, event):
        msg = event.message
        command = _decode(msg.body)
        attempt = Attempt(reply_to=msg.reply_to, message_id=msg.id)

        if not attempt.reply_to:
            print("[responder] FATAL: command carried no reply-to",
                  file=sys.stderr, flush=True)
            return

        try:
            principal = self.principals.principal_for(msg)
        except PrincipalUnavailable as exc:
            # Refused rather than guessed: the idempotency guarantee is
            # only as good as the identity it is keyed on.
            print(f"[responder] refused: {exc}", file=sys.stderr, flush=True)
            self._send(attempt, self._frame(command, "invalid", str(exc)))
            return

        correlation = command.get("commandCorrelationId")
        print(f"[responder] received principal={principal} "
              f"commandCorrelationId={correlation}", flush=True)

        try:
            record, execute = self.store.begin(
                principal, correlation, command, attempt
            )
        except IdempotencyConflict as exc:
            # Same key, different command. Answering with the stored
            # result would report success for something never run.
            print(f"[responder] conflict: {exc}", file=sys.stderr, flush=True)
            self._send(attempt, self._frame(command, "invalid", str(exc)))
            return

        if not execute:
            if record.state is RecordState.TERMINAL:
                # Replay the *result*, in a fresh envelope.
                self._send(attempt, dict(record.terminal_result))
            else:
                # Still running: tell the retry what we know, and keep it
                # registered so it also gets the terminal frame.
                self._send(attempt, record.latest_interim
                           or self._frame(command, "pending"))
            self._maybe_exit(event)
            return

        # First time for this key: execute once.
        record.latest_interim = self._frame(command, "pending")
        for waiting in list(record.attempts):
            self._send(waiting, record.latest_interim)

        if DELAY > 0:
            # Scheduled, never slept. time.sleep() blocks the Proton
            # reactor, so a retry sent during the window would not be
            # *received* until execution had already finished — the
            # in-progress case would be untestable and would look like it
            # passed. Scheduling keeps the reactor live, so the retry is
            # registered as a waiter mid-flight, which is the whole point.
            event.reactor.schedule(DELAY, _Complete(self, record, command))
        else:
            self._complete(record, command, event.reactor)
        self._maybe_exit(event)

    def _complete(self, record, command: dict, reactor) -> None:
        record.terminal_result = self._frame(command, "success")
        record.state = RecordState.TERMINAL
        self.store.results_produced += 1
        # Every attempt registered by now — including any that arrived
        # while this was executing — gets the terminal result addressed to
        # its own node with its own correlation-id.
        for waiting in list(record.attempts):
            self._send(waiting, dict(record.terminal_result))
        if MAX_REPLIES and self.answered >= MAX_REPLIES:
            self._summary()
            self._connection.close()

    def _maybe_exit(self, event):
        if MAX_REPLIES and self.answered >= MAX_REPLIES:
            self._summary()
            event.connection.close()

    def _summary(self):
        s = self.store
        print(
            f"[idempotency] received={s.commands_received} "
            f"executed={s.commands_executed} produced={s.results_produced} "
            f"delivered={s.responses_delivered} conflicts={s.conflicts}",
            flush=True,
        )


class _Complete:
    """Deferred completion, run on the reactor rather than in a sleep."""

    def __init__(self, responder, record, command) -> None:
        self.responder = responder
        self.record = record
        self.command = command

    def on_timer_task(self, event):
        self.responder._complete(self.record, self.command, event.reactor)


def main() -> int:
    responder = IdempotentResponder(SERVER)

    def _on_term(_signum, _frame):
        # The harness stops the responder with SIGTERM once its attempts
        # are done. Without this the counters — the whole point — would
        # never be printed.
        responder._summary()
        sys.exit(0)

    signal.signal(signal.SIGTERM, _on_term)
    try:
        Container(responder).run()
    finally:
        responder._summary()
    return 0


if __name__ == "__main__":
    sys.exit(main())
