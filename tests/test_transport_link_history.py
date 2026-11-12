# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Transport-link history entries: minimal and optional-field
#              construction, rejection of a missing log entry and negative log
#              ids, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.transport_link.history."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.transport_link.history import TransportLinkHistoryEntry


NOW = datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc)


def test_transport_link_history_entry_minimal():
    h = TransportLinkHistoryEntry(
        transport_link_id="tl-001",
        log_id=1,
        log_date=NOW,
        log_entry="Link reconfigured",
    )
    assert h.transport_link_id == "tl-001"
    assert h.log_id == 1
    assert h.log_entry == "Link reconfigured"
    assert h.log_user is None
    assert h.event_type is None


def test_transport_link_history_entry_with_optional_fields():
    h = TransportLinkHistoryEntry(
        transport_link_id="tl-001",
        log_id=2,
        log_date=NOW,
        log_entry="Capacity updated",
        log_user="operator.a",
        event_type="capacity_update",
    )
    assert h.log_user == "operator.a"
    assert h.event_type == "capacity_update"


def test_transport_link_history_entry_rejects_missing_log_entry():
    with pytest.raises(ValidationError):
        TransportLinkHistoryEntry(
            transport_link_id="tl-001",
            log_id=1,
            log_date=NOW,
        )


def test_transport_link_history_entry_rejects_negative_log_id():
    with pytest.raises(ValidationError):
        TransportLinkHistoryEntry(
            transport_link_id="tl-001",
            log_id=-1,
            log_date=NOW,
            log_entry="oops",
        )


def test_transport_link_history_entry_dumps_camel_case():
    h = TransportLinkHistoryEntry(
        transport_link_id="tl-001",
        log_id=1,
        log_date=NOW,
        log_entry="Link reconfigured",
        log_user="operator.a",
        event_type="reconfiguration",
    )
    dumped = h.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["transportLinkId"] == "tl-001"
    assert dumped["logId"] == 1
    assert dumped["logDate"] == "2026-04-11T10:00:00Z"
    assert dumped["logEntry"] == "Link reconfigured"
    assert dumped["logUser"] == "operator.a"
    assert dumped["eventType"] == "reconfiguration"
