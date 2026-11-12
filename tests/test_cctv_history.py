# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: CCTV history entries: minimal and full construction, negative
#              log-id rejection, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.cctv.history."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.cctv.history import CctvHistoryEntry


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_cctv_history_minimal():
    h = CctvHistoryEntry(
        cctv_id="cam-001", log_id=1, log_date=NOW, log_entry="Camera rebooted",
    )
    assert h.log_entry == "Camera rebooted"


def test_cctv_history_with_all_fields():
    h = CctvHistoryEntry(
        cctv_id="cam-001", log_id=1, log_date=NOW,
        log_entry="Camera rebooted", log_user="tech-01", event_type="maintenance",
    )
    assert h.log_user == "tech-01"


def test_cctv_history_rejects_negative_log_id():
    with pytest.raises(ValidationError):
        CctvHistoryEntry(
            cctv_id="cam-001", log_id=-1, log_date=NOW, log_entry="test",
        )


def test_cctv_history_dumps_camel_case():
    h = CctvHistoryEntry(
        cctv_id="cam-001", log_id=1, log_date=NOW, log_entry="Camera rebooted",
    )
    dumped = h.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["cctvId"] == "cam-001"
    assert dumped["logEntry"] == "Camera rebooted"
