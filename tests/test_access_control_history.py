# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Access-control history entries: minimal and full construction,
#              negative log-id rejection, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.access_control.history."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.access_control.history import AccessControlHistoryEntry


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_access_control_history_minimal():
    h = AccessControlHistoryEntry(
        access_control_id="ac-001",
        log_id=1,
        log_date=NOW,
        log_entry="Bollard serviced",
    )
    assert h.log_entry == "Bollard serviced"
    assert h.log_user is None


def test_access_control_history_with_all_fields():
    h = AccessControlHistoryEntry(
        access_control_id="ac-001",
        log_id=1,
        log_date=NOW,
        log_entry="Bollard serviced",
        log_user="engineer-01",
        event_type="maintenance",
        extensions={"work_order": "WO-456"},
    )
    assert h.log_user == "engineer-01"
    assert h.event_type == "maintenance"


def test_access_control_history_rejects_negative_log_id():
    with pytest.raises(ValidationError):
        AccessControlHistoryEntry(
            access_control_id="ac-001",
            log_id=-1,
            log_date=NOW,
            log_entry="test",
        )


def test_access_control_history_dumps_camel_case():
    h = AccessControlHistoryEntry(
        access_control_id="ac-001",
        log_id=1,
        log_date=NOW,
        log_entry="Bollard serviced",
        log_user="engineer-01",
    )
    dumped = h.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["accessControlId"] == "ac-001"
    assert dumped["logId"] == 1
    assert dumped["logEntry"] == "Bollard serviced"
    assert dumped["logUser"] == "engineer-01"
