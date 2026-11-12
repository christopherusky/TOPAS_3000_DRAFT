# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: VMS history entries: minimal and full construction, negative
#              log-id rejection, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.vms.history."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.vms.history import VmsHistoryEntry


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_vms_history_minimal():
    h = VmsHistoryEntry(
        vms_id="vms-001",
        log_id=1,
        log_date=NOW,
        log_entry="Sign powered on",
    )
    assert h.log_entry == "Sign powered on"
    assert h.log_user is None


def test_vms_history_with_all_fields():
    h = VmsHistoryEntry(
        vms_id="vms-001",
        log_id=1,
        log_date=NOW,
        log_entry="LED panel replaced",
        log_user="engineer-01",
        event_type="maintenance",
        extensions={"work_order": "WO-789"},
    )
    assert h.log_user == "engineer-01"
    assert h.event_type == "maintenance"


def test_vms_history_rejects_negative_log_id():
    with pytest.raises(ValidationError):
        VmsHistoryEntry(
            vms_id="vms-001",
            log_id=-1,
            log_date=NOW,
            log_entry="test",
        )


def test_vms_history_dumps_camel_case():
    h = VmsHistoryEntry(
        vms_id="vms-001",
        log_id=1,
        log_date=NOW,
        log_entry="Sign powered on",
        log_user="engineer-01",
    )
    dumped = h.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["vmsId"] == "vms-001"
    assert dumped["logId"] == 1
    assert dumped["logEntry"] == "Sign powered on"
    assert dumped["logUser"] == "engineer-01"
