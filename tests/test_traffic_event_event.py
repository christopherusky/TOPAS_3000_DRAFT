# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Planned events: minimal and full construction, planned against
#              actual dates, the event-type enum size, empty-id rejection, and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.traffic_event.event."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.traffic_event.event import PlannedEvent, PlannedEventType
from topas_3000_model.traffic_event.common import EventStatus, SeverityType


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)
LATER = datetime(2026, 4, 16, 18, 0, tzinfo=timezone.utc)


def test_planned_event_minimal():
    e = PlannedEvent(
        event_id="evt-001",
        planned_event_type=PlannedEventType.FOOTBALL,
        status=EventStatus.PLANNED,
    )
    assert e.event_id == "evt-001"
    assert e.planned_event_type == PlannedEventType.FOOTBALL
    assert e.planned_start is None
    assert e.organiser is None


def test_planned_event_with_all_fields():
    e = PlannedEvent(
        event_id="evt-001",
        planned_event_type=PlannedEventType.MARATHON,
        status=EventStatus.ACTIVE,
        public_description="City marathon — expect road closures in centre",
        location_description="City centre, High Street to Park Road",
        severity=SeverityType.MEDIUM,
        lanes_affected=None,
        diversion_route="Ring road via A52",
        diversion_in_force=True,
        reported_by="Events Team",
        confirmed_by="Traffic Management",
        planned_start=NOW,
        planned_end=LATER,
        actual_start=NOW,
        organiser="City Athletics Club",
        venue_name="City Centre",
    )
    assert e.organiser == "City Athletics Club"
    assert e.venue_name == "City Centre"
    assert e.planned_start == NOW
    assert e.actual_end is None


def test_planned_event_with_planned_and_actual_dates():
    e = PlannedEvent(
        event_id="evt-001",
        planned_event_type=PlannedEventType.FOOTBALL,
        status=EventStatus.CLEARED,
        planned_start=NOW,
        planned_end=LATER,
        actual_start=NOW,
        actual_end=LATER,
    )
    assert e.planned_start == NOW
    assert e.actual_end == LATER


def test_planned_event_type_enum_count():
    # XSD-parity pin: fails on any deliberate addition, so update it with the decision.
    assert len(PlannedEventType) == 32


def test_planned_event_rejects_empty_id():
    with pytest.raises(ValidationError):
        PlannedEvent(
            event_id="",
            planned_event_type=PlannedEventType.FOOTBALL,
            status=EventStatus.PLANNED,
        )


def test_planned_event_dumps_camel_case():
    e = PlannedEvent(
        event_id="evt-001",
        planned_event_type=PlannedEventType.SHOW_JUMPING,
        status=EventStatus.PLANNED,
        planned_start=NOW,
        planned_end=LATER,
        organiser="County Show Society",
        venue_name="Showground",
        severity=SeverityType.LOW,
    )
    dumped = e.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["eventId"] == "evt-001"
    assert dumped["plannedEventType"] == "showJumping"
    assert dumped["status"] == "planned"
    assert dumped["plannedStart"] == "2026-04-16T10:00:00Z"
    assert dumped["plannedEnd"] == "2026-04-16T18:00:00Z"
    assert dumped["organiser"] == "County Show Society"
    assert dumped["venueName"] == "Showground"
