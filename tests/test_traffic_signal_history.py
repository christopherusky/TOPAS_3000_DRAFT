# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-04-23
# Version:     1.2
# Description: Traffic-signal device history: minimal construction, user
#              attribution, rejection of a missing log entry and negative log
#              ids, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.traffic_signal.history."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.traffic_signal.history import TrafficSignalHistoryEntry


NOW = datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc)


def test_traffic_signal_device_history_minimal():
    h = TrafficSignalHistoryEntry(
        traffic_signal_id="ts-001",
        log_id=1,
        log_date=NOW,
        log_entry="Controller rebooted",
    )
    assert h.traffic_signal_id == "ts-001"
    assert h.log_id == 1
    assert h.log_entry == "Controller rebooted"
    assert h.log_user is None


def test_traffic_signal_device_history_with_user():
    h = TrafficSignalHistoryEntry(
        traffic_signal_id="ts-001",
        log_id=2,
        log_date=NOW,
        log_entry="Firmware upgrade",
        log_user="operator.a",
    )
    assert h.log_user == "operator.a"


def test_traffic_signal_device_history_rejects_missing_log_entry():
    with pytest.raises(ValidationError):
        TrafficSignalHistoryEntry(
            traffic_signal_id="ts-001",
            log_id=1,
            log_date=NOW,
        )


def test_traffic_signal_device_history_rejects_negative_log_id():
    with pytest.raises(ValidationError):
        TrafficSignalHistoryEntry(
            traffic_signal_id="ts-001",
            log_id=-1,
            log_date=NOW,
            log_entry="oops",
        )


def test_traffic_signal_device_history_dumps_camel_case():
    h = TrafficSignalHistoryEntry(
        traffic_signal_id="ts-001",
        log_id=1,
        log_date=NOW,
        log_entry="Controller rebooted",
        log_user="operator.a",
    )
    dumped = h.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["trafficSignalId"] == "ts-001"
    assert dumped["logId"] == 1
    assert dumped["logDate"] == "2026-04-11T10:00:00Z"
    assert dumped["logEntry"] == "Controller rebooted"
    assert dumped["logUser"] == "operator.a"
