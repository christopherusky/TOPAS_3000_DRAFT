# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-29
# Version:     1.0
# Description: The A11.2 idempotency store and principal seam:
#              identity keying independent of field order, exclusion of
#              transport properties, execute-once with retries registered as
#              waiters, separation by principal, conflict detection on a
#              reused key, disabled-store behaviour, and attempt counting.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for the A11.2 idempotency store and principal seam.

The conformance harness proves the behaviour against a live broker; these
cover the units it is built from and run without one.
"""
import pytest

from asyncapi_clients.smoketest.idempotency import (
    Attempt,
    IdempotencyConflict,
    IdempotencyStore,
    RecordState,
    fingerprint,
)
from asyncapi_clients.smoketest.principal import (
    BrokerValidatedUserProvider,
    ConformancePrincipalProvider,
    PrincipalUnavailable,
)


class _Msg:
    def __init__(self, properties):
        self.properties = properties


# ── fingerprinting ──

def test_key_order_does_not_change_identity():
    a = {"kind": "setPlan", "planNumber": 4}
    b = {"planNumber": 4, "kind": "setPlan"}
    assert fingerprint(a) == fingerprint(b)


def test_transport_properties_are_excluded():
    base = {"kind": "setPlan", "planNumber": 4}
    retry = dict(base, **{"message-id": "M2", "reply-to": "R2"})
    assert fingerprint(base) == fingerprint(retry)


def test_application_fields_are_part_of_identity():
    a = {"kind": "setPlan", "planNumber": 4, "requestTime": "2026-08-29T10:00:00Z"}
    b = {"kind": "setPlan", "planNumber": 4, "requestTime": "2026-08-29T10:00:01Z"}
    assert fingerprint(a) != fingerprint(b)


def test_a_different_command_differs():
    assert fingerprint({"kind": "setPlan", "planNumber": 4}) != \
           fingerprint({"kind": "setPlan", "planNumber": 7})


# ── the store ──

CMD = {"kind": "setPlan", "planNumber": 4, "commandCorrelationId": 7777}


def _attempt(n):
    return Attempt(reply_to=f"R{n}", message_id=f"M{n}")


def test_first_attempt_executes():
    store = IdempotencyStore()
    record, execute = store.begin("principal-a", 7777, CMD, _attempt(1))
    assert execute is True
    assert store.commands_executed == 1
    assert record.state is RecordState.IN_PROGRESS


def test_retry_does_not_execute_and_is_registered_as_a_waiter():
    store = IdempotencyStore()
    store.begin("principal-a", 7777, CMD, _attempt(1))
    record, execute = store.begin("principal-a", 7777, CMD, _attempt(2))
    assert execute is False
    assert store.commands_executed == 1
    # The retry must be kept so the terminal result reaches its new node.
    assert [a.message_id for a in record.attempts] == ["M1", "M2"]
    assert [a.reply_to for a in record.attempts] == ["R1", "R2"]


def test_same_correlation_from_another_principal_is_a_separate_command():
    store = IdempotencyStore()
    store.begin("principal-a", 7777, CMD, _attempt(1))
    _, execute = store.begin("principal-b", 7777, CMD, _attempt(2))
    assert execute is True
    assert store.commands_executed == 2


def test_a_different_command_on_the_same_key_conflicts():
    store = IdempotencyStore()
    store.begin("principal-a", 7777, CMD, _attempt(1))
    other = dict(CMD, planNumber=7)
    with pytest.raises(IdempotencyConflict):
        store.begin("principal-a", 7777, other, _attempt(2))
    # And it must not have been executed.
    assert store.commands_executed == 1
    assert store.conflicts == 1


def test_disabling_the_store_executes_every_time():
    """The seam the harness uses to prove its own counters move."""
    store = IdempotencyStore(enabled=False)
    store.begin("principal-a", 7777, CMD, _attempt(1))
    _, execute = store.begin("principal-a", 7777, CMD, _attempt(2))
    assert execute is True
    assert store.commands_executed == 2


def test_received_counts_every_attempt():
    store = IdempotencyStore()
    for i in range(3):
        try:
            store.begin("principal-a", 7777, CMD, _attempt(i))
        except IdempotencyConflict:
            pass
    assert store.commands_received == 3
    assert store.commands_executed == 1


# ── the principal seam ──

def test_conformance_provider_resolves_a_mapped_marker():
    p = ConformancePrincipalProvider({"conn-a": "principal-a"})
    assert p.principal_for(_Msg({"x-conformance-connection": "conn-a"})) == "principal-a"


def test_an_unmapped_marker_is_refused_not_trusted():
    """Otherwise this is x-originator wearing a different name."""
    p = ConformancePrincipalProvider({"conn-a": "principal-a"})
    with pytest.raises(PrincipalUnavailable):
        p.principal_for(_Msg({"x-conformance-connection": "conn-unknown"}))


def test_a_missing_marker_is_refused():
    p = ConformancePrincipalProvider({"conn-a": "principal-a"})
    with pytest.raises(PrincipalUnavailable):
        p.principal_for(_Msg({}))


def test_x_originator_is_never_consulted():
    p = ConformancePrincipalProvider({"conn-a": "principal-a"})
    msg = _Msg({"x-conformance-connection": "conn-a", "x-originator": "principal-b"})
    assert p.principal_for(msg) == "principal-a"


def test_broker_provider_reads_the_validated_user():
    p = BrokerValidatedUserProvider()
    assert p.principal_for(_Msg({"_AMQ_VALIDATED_USER": "principal-a"})) == "principal-a"


def test_broker_provider_refuses_when_the_broker_asserted_nothing():
    p = BrokerValidatedUserProvider()
    with pytest.raises(PrincipalUnavailable):
        p.principal_for(_Msg({"x-originator": "principal-a"}))
