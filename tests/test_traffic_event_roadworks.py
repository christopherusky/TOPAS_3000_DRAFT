# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Roadworks events: minimal and full construction, the roadworks-
#              type enum size, rejection of an empty id and negative lanes,
#              planned against actual dates, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.traffic_event.roadworks."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.traffic_event.roadworks import RoadworksEvent, RoadworksType
from topas_3000_model.traffic_event.common import (
    EventStatus,
    LanesAffectedType,
    SeverityType,
)


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)
LATER = datetime(2026, 4, 20, 18, 0, tzinfo=timezone.utc)


def test_roadworks_event_minimal():
    e = RoadworksEvent(
        event_id="rw-001",
        roadworks_type=RoadworksType.SURFACE,
        status=EventStatus.PLANNED,
    )
    assert e.event_id == "rw-001"
    assert e.roadworks_type == RoadworksType.SURFACE
    assert e.contractor is None
    assert e.traffic_signals is None


def test_roadworks_event_with_all_fields():
    e = RoadworksEvent(
        event_id="rw-001",
        roadworks_type=RoadworksType.SURFACE,
        status=EventStatus.ACTIVE,
        public_description="Resurfacing on A52 — expect delays",
        location_description="A52 eastbound between J25 and J26",
        severity=SeverityType.MEDIUM,
        lanes_affected=LanesAffectedType.IITT,
        lanes_restricted=2,
        lanes_operational=2,
        diversion_route="M1 via J25",
        diversion_in_force=False,
        reported_by="Highways England",
        confirmed_by="Regional Office",
        planned_start=NOW,
        planned_end=LATER,
        actual_start=NOW,
        contractor="Kier Highways",
        traffic_signals=True,
        contraflow=True,
        confirmed_date=NOW,
        external_country_code="GB",
        external_system_code="NTIS",
        external_situation_id="RW-5678",
        external_situation_version=3,
    )
    assert e.contractor == "Kier Highways"
    assert e.traffic_signals is True
    assert e.contraflow is True
    assert e.confirmed_date == NOW


def test_roadworks_type_enum_count():
    assert len(RoadworksType) == 7


def test_roadworks_event_rejects_empty_id():
    with pytest.raises(ValidationError):
        RoadworksEvent(
            event_id="",
            roadworks_type=RoadworksType.GENERAL,
            status=EventStatus.PLANNED,
        )


def test_roadworks_event_rejects_negative_lanes():
    with pytest.raises(ValidationError):
        RoadworksEvent(
            event_id="rw-001",
            roadworks_type=RoadworksType.GENERAL,
            status=EventStatus.PLANNED,
            lanes_operational=-1,
        )


def test_roadworks_event_with_planned_and_actual():
    e = RoadworksEvent(
        event_id="rw-001",
        roadworks_type=RoadworksType.WATER_MAIN_WORK,
        status=EventStatus.CLEARED,
        planned_start=NOW,
        planned_end=LATER,
        actual_start=NOW,
        actual_end=LATER,
    )
    assert e.planned_start == NOW
    assert e.actual_end == LATER
    assert e.status == EventStatus.CLEARED


def test_roadworks_event_dumps_camel_case():
    e = RoadworksEvent(
        event_id="rw-001",
        roadworks_type=RoadworksType.CENTRAL_RESERVATION,
        status=EventStatus.ACTIVE,
        planned_start=NOW,
        planned_end=LATER,
        contractor="Balfour Beatty",
        traffic_signals=True,
        contraflow=False,
        confirmed_date=NOW,
        severity=SeverityType.LOW,
        lanes_affected=LanesAffectedType.IIIT,
        lanes_restricted=1,
        lanes_operational=3,
    )
    dumped = e.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["eventId"] == "rw-001"
    assert dumped["roadworksType"] == "centralReservation"
    assert dumped["status"] == "active"
    assert dumped["plannedStart"] == "2026-04-16T10:00:00Z"
    assert dumped["plannedEnd"] == "2026-04-20T18:00:00Z"
    assert dumped["contractor"] == "Balfour Beatty"
    assert dumped["trafficSignals"] is True
    assert dumped["contraflow"] is False
    assert dumped["confirmedDate"] == "2026-04-16T10:00:00Z"
    assert dumped["lanesAffected"] == "IIIT"
    assert dumped["lanesRestricted"] == 1
    assert dumped["lanesOperational"] == 3
