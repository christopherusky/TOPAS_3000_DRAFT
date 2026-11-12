# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Incident events: minimal and full construction, the incident-
#              type enum size including its weather and rare values, rejection
#              of an empty id and negative lanes, suspended status, and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.traffic_event.incident."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.traffic_event.incident import IncidentEvent, IncidentType
from topas_3000_model.traffic_event.common import (
    EventStatus,
    LanesAffectedType,
    SeverityType,
)


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_incident_event_minimal():
    e = IncidentEvent(
        event_id="inc-001",
        incident_type=IncidentType.CONGESTION,
        status=EventStatus.ACTIVE,
        incident_time=NOW,
    )
    assert e.event_id == "inc-001"
    assert e.incident_type == IncidentType.CONGESTION
    assert e.end_time is None


def test_incident_event_with_all_fields():
    e = IncidentEvent(
        event_id="inc-001",
        incident_type=IncidentType.ROAD_CLOSURE,
        status=EventStatus.ACTIVE,
        public_description="A52 closed due to burst water main",
        location_description="A52 westbound at Derby",
        severity=SeverityType.HIGH,
        lanes_affected=LanesAffectedType.TTTT,
        lanes_restricted=4,
        lanes_operational=0,
        diversion_route="A38 via Burton",
        diversion_in_force=True,
        reported_by="Highways Team",
        incident_time=NOW,
        external_country_code="GB",
        external_system_code="NTIS",
        external_situation_id="SIT-999",
        external_situation_version=1,
    )
    assert e.lanes_restricted == 4
    assert e.lanes_operational == 0
    assert e.diversion_in_force is True


def test_incident_type_enum_count():
    # XSD-parity pin: fails on any deliberate addition, so update it with the decision.
    assert len(IncidentType) == 78


def test_incident_type_weather_values():
    """Weather-related incident types exist (keep all)."""
    weather_types = [
        IncidentType.FOG, IncidentType.RAIN, IncidentType.SNOW,
        IncidentType.ICE, IncidentType.HAIL, IncidentType.WIND,
        IncidentType.TORNADOES, IncidentType.SANDSTORMS,
        IncidentType.FLOOD, IncidentType.WASHOUT,
    ]
    assert all(t.value for t in weather_types)


def test_incident_type_rare_values():
    """Rare but valid incident types exist (keep all)."""
    assert IncidentType.RADIOACTIVE_LEAK.value == "radioactiveLeak"
    assert IncidentType.AIR_CRASH.value == "airCrash"
    assert IncidentType.TERRORIST_INCIDENT.value == "terroristIncident"
    assert IncidentType.GUNFIRE.value == "gunfire"


def test_incident_event_rejects_empty_id():
    with pytest.raises(ValidationError):
        IncidentEvent(
            event_id="",
            incident_type=IncidentType.CONGESTION,
            status=EventStatus.ACTIVE,
            incident_time=NOW,
        )


def test_incident_event_rejects_negative_lanes():
    with pytest.raises(ValidationError):
        IncidentEvent(
            event_id="inc-001",
            incident_type=IncidentType.CONGESTION,
            status=EventStatus.ACTIVE,
            incident_time=NOW,
            lanes_restricted=-1,
        )


def test_incident_event_suspended_status():
    e = IncidentEvent(
        event_id="inc-001",
        incident_type=IncidentType.ROAD_CLOSURE,
        status=EventStatus.SUSPENDED,
        incident_time=NOW,
    )
    assert e.status == EventStatus.SUSPENDED


def test_incident_event_dumps_camel_case():
    e = IncidentEvent(
        event_id="inc-001",
        incident_type=IncidentType.BROKEN_DOWN_VEHICLE,
        status=EventStatus.ACTIVE,
        incident_time=NOW,
        severity=SeverityType.LOW,
        lanes_affected=LanesAffectedType.IIIT,
        public_description="Broken down vehicle on hard shoulder",
    )
    dumped = e.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["eventId"] == "inc-001"
    assert dumped["incidentType"] == "brokenDownVehicle"
    assert dumped["status"] == "active"
    assert dumped["incidentTime"] == "2026-04-16T10:00:00Z"
    assert dumped["severity"] == "low"
    assert dumped["lanesAffected"] == "IIIT"
    assert dumped["publicDescription"] == "Broken down vehicle on hard shoulder"
