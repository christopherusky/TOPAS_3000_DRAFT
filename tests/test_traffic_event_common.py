# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Shared traffic-event types: the EventStatus, SeverityType and
#              LanesAffectedType value sets, including the single-lane and
#              four-lane affected patterns.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.traffic_event.common."""

from topas_3000_model.traffic_event.common import (
    EventStatus,
    LanesAffectedType,
    SeverityType,
)


def test_event_status_values():
    assert EventStatus.ACTIVE.value == "active"
    assert EventStatus.PLANNED.value == "planned"
    assert EventStatus.SUSPENDED.value == "suspended"
    assert EventStatus.CLEARED.value == "cleared"
    assert len(EventStatus) == 4


def test_severity_type_values():
    assert SeverityType.UNKNOWN.value == "unknown"
    assert SeverityType.LOW.value == "low"
    assert SeverityType.MEDIUM.value == "medium"
    assert SeverityType.HIGH.value == "high"
    assert len(SeverityType) == 4


def test_lanes_affected_type_values():
    assert LanesAffectedType.IIIT.value == "IIIT"
    assert LanesAffectedType.ALL_OPEN.value == "allOpen"
    assert LanesAffectedType.T.value == "T"
    # XSD-parity pin: fails on any deliberate addition, so update it with the decision.
    assert len(LanesAffectedType) == 21


def test_lanes_affected_single_lane_patterns():
    assert LanesAffectedType.I.value == "I"
    assert LanesAffectedType.T.value == "T"


def test_lanes_affected_four_lane_patterns():
    four_lane = [
        LanesAffectedType.IIII,
        LanesAffectedType.IIIT,
        LanesAffectedType.IITT,
        LanesAffectedType.ITTT,
        LanesAffectedType.TIII,
        LanesAffectedType.TTTT,
    ]
    assert len(four_lane) == 6
    assert all(len(l.value) == 4 for l in four_lane)
