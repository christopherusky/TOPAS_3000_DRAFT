# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Accident events: minimal, full and cleared construction,
#              rejection of an empty id and negative vehicle counts or
#              restricted lanes, the accident-type enum size, and camelCase
#              serialisation including external references.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.traffic_event.accident."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.traffic_event.accident import AccidentEvent, AccidentType
from topas_3000_model.traffic_event.common import (
    EventStatus,
    LanesAffectedType,
    SeverityType,
)
from topas_3000_model.common.location import Coordinates, Point


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_accident_event_minimal():
    e = AccidentEvent(
        event_id="acc-001",
        accident_type=AccidentType.COLLISION,
        status=EventStatus.ACTIVE,
        accident_time=NOW,
    )
    assert e.event_id == "acc-001"
    assert e.accident_type == AccidentType.COLLISION
    assert e.status == EventStatus.ACTIVE
    assert e.end_time is None
    assert e.vehicle_count is None


def test_accident_event_with_all_fields():
    e = AccidentEvent(
        event_id="acc-001",
        accident_type=AccidentType.MULTI_VEHICLE,
        status=EventStatus.ACTIVE,
        public_description="M1 northbound blocked at J32 due to multi-vehicle collision",
        location_description="M1 northbound between J32 and J33",
        location=Point(coordinates=Coordinates(latitude=53.5, longitude=-1.5)),
        severity=SeverityType.HIGH,
        phase="response",
        lanes_affected=LanesAffectedType.IITT,
        lanes_restricted=2,
        lanes_operational=2,
        diversion_route="A638 via Wakefield",
        diversion_in_force=True,
        reported_by="Traffic Officer",
        confirmed_by="Control Room",
        modified_by="Ops Manager",
        accident_time=NOW,
        end_time=NOW,
        vehicle_count=3,
        light_conditions="daylight",
        road_conditions="wet",
        weather_conditions="rain",
        statistic_date=NOW,
        external_country_code="GB",
        external_system_code="NTIS",
        external_situation_id="SIT-12345",
        external_situation_version=2,
        extensions={"source": "CCTV"},
    )
    assert e.vehicle_count == 3
    assert e.lanes_restricted == 2
    assert e.public_description.startswith("M1")
    assert e.external_situation_id == "SIT-12345"


def test_accident_event_cleared():
    e = AccidentEvent(
        event_id="acc-001",
        accident_type=AccidentType.COLLISION,
        status=EventStatus.CLEARED,
        accident_time=NOW,
        end_time=NOW,
    )
    assert e.status == EventStatus.CLEARED
    assert e.end_time is not None


def test_accident_event_rejects_empty_id():
    with pytest.raises(ValidationError):
        AccidentEvent(
            event_id="",
            accident_type=AccidentType.COLLISION,
            status=EventStatus.ACTIVE,
            accident_time=NOW,
        )


def test_accident_event_rejects_negative_vehicle_count():
    with pytest.raises(ValidationError):
        AccidentEvent(
            event_id="acc-001",
            accident_type=AccidentType.COLLISION,
            status=EventStatus.ACTIVE,
            accident_time=NOW,
            vehicle_count=-1,
        )


def test_accident_event_rejects_negative_lanes_restricted():
    with pytest.raises(ValidationError):
        AccidentEvent(
            event_id="acc-001",
            accident_type=AccidentType.COLLISION,
            status=EventStatus.ACTIVE,
            accident_time=NOW,
            lanes_restricted=-1,
        )


def test_accident_type_enum_count():
    # XSD-parity pin: fails on any deliberate addition, so update it with the decision.
    assert len(AccidentType) == 20


def test_accident_event_dumps_camel_case():
    e = AccidentEvent(
        event_id="acc-001",
        accident_type=AccidentType.SHED_LOAD,
        status=EventStatus.ACTIVE,
        accident_time=NOW,
        severity=SeverityType.MEDIUM,
        lanes_affected=LanesAffectedType.IIIT,
        lanes_restricted=1,
        lanes_operational=3,
        vehicle_count=1,
        public_description="Shed load on A1",
    )
    dumped = e.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["eventId"] == "acc-001"
    assert dumped["accidentType"] == "shedLoad"
    assert dumped["status"] == "active"
    assert dumped["severity"] == "medium"
    assert dumped["lanesAffected"] == "IIIT"
    assert dumped["lanesRestricted"] == 1
    assert dumped["lanesOperational"] == 3
    assert dumped["vehicleCount"] == 1
    assert dumped["accidentTime"] == "2026-04-16T10:00:00Z"
    assert dumped["publicDescription"] == "Shed load on A1"
    assert "externalCountryCode" not in dumped


def test_accident_event_external_refs_dump_camel_case():
    e = AccidentEvent(
        event_id="acc-001",
        accident_type=AccidentType.COLLISION,
        status=EventStatus.ACTIVE,
        accident_time=NOW,
        external_country_code="GB",
        external_system_code="NTIS",
        external_situation_id="SIT-123",
        external_situation_version=1,
    )
    dumped = e.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["externalCountryCode"] == "GB"
    assert dumped["externalSystemCode"] == "NTIS"
    assert dumped["externalSituationId"] == "SIT-123"
    assert dumped["externalSituationVersion"] == 1
