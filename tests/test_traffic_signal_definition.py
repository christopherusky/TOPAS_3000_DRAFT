# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-29
# Version:     1.5
# Description: TrafficSignalDefinition (TS004.0064bis:2017 section 3.2):
#              minimal and full payloads, rejection of a missing identifier
#              and extra fields, snake_case construction with camelCase dump,
#              descriptive-field round-trip, and control-method id.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for TrafficSignalDefinition (TS004.0064bis:2017 §3.2)."""

import pytest
from pydantic import ValidationError

from topas_3000_model.common.geometry import GeoJsonPoint
from topas_3000_model.traffic_signal.definition import TrafficSignalDefinition


def _minimal_payload() -> dict:
    return {"trafficSignalId": "TS-001"}


def test_minimal_payload_valid():
    d = TrafficSignalDefinition.model_validate(_minimal_payload())
    assert d.traffic_signal_id == "TS-001"
    assert d.location is None
    assert d.extensions is None


def test_missing_identifier_rejected():
    with pytest.raises(ValidationError):
        TrafficSignalDefinition.model_validate({})


def test_full_payload_round_trip():
    payload = {
        "trafficSignalId": "TS-001",
        "location": {"type": "Point", "coordinates": [-2.2426, 53.4808]},
        "extensions": {"siemensHealth": "OK"},
    }
    d = TrafficSignalDefinition.model_validate(payload)
    assert isinstance(d.location, GeoJsonPoint)
    assert d.extensions == {"siemensHealth": "OK"}
    dumped = d.model_dump(by_alias=True, exclude_none=True)
    assert dumped["trafficSignalId"] == "TS-001"
    assert dumped["location"]["type"] == "Point"
    assert dumped["location"]["coordinates"] == (-2.2426, 53.4808)
    assert dumped["extensions"] == {"siemensHealth": "OK"}


def test_extra_field_rejected():
    with pytest.raises(ValidationError):
        TrafficSignalDefinition.model_validate(
            {"trafficSignalId": "TS-001", "unexpected": 42}
        )


def test_snake_case_construction():
    # populate_by_name=True allows snake_case in Python.
    d = TrafficSignalDefinition(traffic_signal_id="TS-001")
    assert d.traffic_signal_id == "TS-001"


def test_camel_case_alias_on_dump():
    d = TrafficSignalDefinition(traffic_signal_id="TS-001")
    assert "trafficSignalId" in d.model_dump(by_alias=True)


def test_descriptive_fields_round_trip():
    payload = {
        "trafficSignalId": "TS-001",
        "creationDate": "2024-03-15T08:00:00Z",
        "shortDescription": "Main St / High St junction",
        "longDescription": "Re-commissioned 2024 with MOVA upgrade.",
        "participantId": "OWNER-A",
        "originatingParticipantId": "UPSTREAM-X",
        "originatingObjectId": "X-TS-9001",
    }
    d = TrafficSignalDefinition.model_validate(payload)
    assert d.short_description == "Main St / High St junction"
    assert d.participant_id == "OWNER-A"
    assert d.originating_object_id == "X-TS-9001"
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["creationDate"].startswith("2024-03-15")
    assert dumped["originatingParticipantId"] == "UPSTREAM-X"


def test_descriptive_fields_remain_optional():
    # Minimal payload still validates after the descriptive carry-over.
    d = TrafficSignalDefinition.model_validate({"trafficSignalId": "TS-001"})
    assert d.creation_date is None
    assert d.short_description is None
    assert d.participant_id is None
    assert d.control_method_id is None


def test_control_method_id_round_trip():
    payload = {
        "trafficSignalId": "TS-001",
        "controlMethodId": "42",
    }
    d = TrafficSignalDefinition.model_validate(payload)
    assert d.control_method_id == "42"
    dumped = d.model_dump(by_alias=True, exclude_none=True)
    assert dumped["controlMethodId"] == "42"
