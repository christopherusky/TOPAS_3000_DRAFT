# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/idempotency.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-29
# Version:     1.1
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Command idempotency: execute once, answer every attempt.

A11.2 requires command processing to be idempotent, keyed on the
authenticated principal together with `commandCorrelationId`. The subtle
part is not the store, it is that **a retry is a different transport
exchange for the same application command**:

    attempt 1:  commandCorrelationId=1234  message-id=M1  reply-to=R1
    attempt 2:  commandCorrelationId=1234  message-id=M2  reply-to=R2

One command, two exchanges. The terminal response owed to attempt 2 must
carry `to=R2` and `correlation-id=M2` — the *result* is replayed, the
transport envelope never is. Storing and re-sending attempt 1's envelope
would deliver the answer to a reply node that has probably gone away.
"""
from __future__ import annotations

import hashlib
import json
from dataclasses import dataclass, field
from enum import Enum

# Transport properties are excluded from the fingerprint: they legitimately
# differ between attempts at the same command.
TRANSPORT_PROPERTIES = ("message-id", "reply-to", "to", "correlation-id")


class RecordState(str, Enum):
    IN_PROGRESS = "inProgress"
    TERMINAL = "terminal"


@dataclass
class Attempt:
    """One transport exchange awaiting an answer."""

    reply_to: str
    message_id: object


@dataclass
class CommandRecord:
    fingerprint: str
    state: RecordState = RecordState.IN_PROGRESS
    latest_interim: dict | None = None
    terminal_result: dict | None = None
    attempts: list[Attempt] = field(default_factory=list)


class IdempotencyConflict(Exception):
    """The key is in use by a materially different command."""


def fingerprint(command: dict) -> str:
    """A stable identity for the *application* command.

    Canonical JSON of the validated payload, so key order does not matter:

        {"kind":"setPlan","planNumber":4}
        {"planNumber":4,"kind":"setPlan"}

    are the same command and must not be executed twice. Application
    fields including `requestTime` are part of the identity — a retry that
    changes one is not an exact retry, and is treated as a conflict rather
    than silently accepted.
    """
    payload = {k: v for k, v in command.items() if k not in TRANSPORT_PROPERTIES}
    canonical = json.dumps(payload, sort_keys=True, separators=(",", ":"))
    return hashlib.sha256(canonical.encode("utf-8")).hexdigest()


class IdempotencyStore:
    """In-memory command store, keyed on principal + commandCorrelationId.

    In-memory is the right choice for a reference smoke test and the wrong
    one wherever idempotency must survive a restart; A11.2's retention
    requirement makes that a deployment decision.

    `enabled=False` disables deduplication while leaving every counter in
    place. That exists so the harness can prove its own assertions fire:
    a suite that never sees `commandsExecuted` climb cannot tell
    "executed once and replayed" from "executed twice and coincidentally
    agreed".
    """

    def __init__(self, enabled: bool = True) -> None:
        self.enabled = enabled
        self._records: dict[tuple[str, object], CommandRecord] = {}
        self.commands_received = 0
        self.commands_executed = 0
        self.results_produced = 0
        self.responses_delivered = 0
        self.conflicts = 0

    @staticmethod
    def key(principal: str, command_correlation_id) -> tuple[str, object]:
        # The principal is part of the key, so the same
        # commandCorrelationId from two principals is two commands.
        return (principal, command_correlation_id)

    def begin(self, principal: str, command_correlation_id, command: dict,
              attempt: Attempt) -> tuple[CommandRecord, bool]:
        """Register an attempt. Returns (record, should_execute)."""
        self.commands_received += 1
        fp = fingerprint(command)

        if not self.enabled:
            record = CommandRecord(fingerprint=fp, attempts=[attempt])
            self.commands_executed += 1
            return record, True

        key = self.key(principal, command_correlation_id)
        record = self._records.get(key)
        if record is None:
            record = CommandRecord(fingerprint=fp, attempts=[attempt])
            self._records[key] = record
            self.commands_executed += 1
            return record, True

        if record.fingerprint != fp:
            # Same key, different command. Never answer with the stored
            # result — that would report success for something that was
            # never run.
            self.conflicts += 1
            raise IdempotencyConflict(
                f"commandCorrelationId {command_correlation_id!r} is already "
                f"in use by a different command for this principal"
            )

        # A genuine retry. Keep the new attempt so the terminal result
        # reaches its reply node too, and do not execute again.
        record.attempts.append(attempt)
        return record, False
