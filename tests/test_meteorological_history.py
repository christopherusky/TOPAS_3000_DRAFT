# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Meteorological history entries: minimal and full construction,
#              negative log-id rejection, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.meteorological.history."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.meteorological.history import MeteorologicalHistoryEntry


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_meteorological_history_minimal():
    h = MeteorologicalHistoryEntry(
        meteorological_id="met-001", log_id=1, log_date=NOW, log_entry="Sensor calibrated",
    )
    assert h.log_entry == "Sensor calibrated"


def test_meteorological_history_with_all_fields():
    h = MeteorologicalHistoryEntry(
        meteorological_id="met-001", log_id=1, log_date=NOW,
        log_entry="Sensor calibrated", log_user="tech-01", event_type="calibration",
    )
    assert h.log_user == "tech-01"


def test_meteorological_history_rejects_negative_log_id():
    with pytest.raises(ValidationError):
        MeteorologicalHistoryEntry(
            meteorological_id="met-001", log_id=-1, log_date=NOW, log_entry="test",
        )


def test_meteorological_history_dumps_camel_case():
    h = MeteorologicalHistoryEntry(
        meteorological_id="met-001", log_id=1, log_date=NOW, log_entry="Sensor calibrated",
    )
    dumped = h.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["meteorologicalId"] == "met-001"
    assert dumped["logEntry"] == "Sensor calibrated"
