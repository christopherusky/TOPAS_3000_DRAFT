# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-05-07
# Version:     1.2
# Description: TrafficSignalConfiguration (TS004.0064bis:2017 section 3.4):
#              minimal and full payloads, rejection of a missing identifier,
#              an unknown type value and extra fields, descriptive-field
#              round-trip and optionality, and user-code length enforcement.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for TrafficSignalConfiguration (TS004.0064bis:2017 §3.4)."""

import pytest
from pydantic import ValidationError

from topas_3000_model.traffic_signal.configuration import TrafficSignalConfiguration
from topas_3000_model.traffic_signal.enums import TrafficSignalTypeEnum


def test_minimal_payload_valid():
    c = TrafficSignalConfiguration.model_validate({"trafficSignalId": "TS-001"})
    assert c.traffic_signal_id == "TS-001"
    assert c.controller_type is None
    assert c.supplier is None
    assert c.traffic_signal_type is None
    assert c.extensions is None


def test_full_payload_valid():
    payload = {
        "trafficSignalId": "TS-001",
        "controllerType": "ST950",
        "supplier": "Siemens",
        "trafficSignalType": "intersection",
        "extensions": {"firmware": "v3.2.1"},
    }
    c = TrafficSignalConfiguration.model_validate(payload)
    assert c.controller_type == "ST950"
    assert c.supplier == "Siemens"
    assert c.traffic_signal_type is TrafficSignalTypeEnum.INTERSECTION
    assert c.extensions == {"firmware": "v3.2.1"}
    assert c.model_dump(by_alias=True, exclude_none=True) == payload


def test_missing_identifier_rejected():
    with pytest.raises(ValidationError):
        TrafficSignalConfiguration.model_validate({})


def test_unknown_type_value_rejected():
    with pytest.raises(ValidationError):
        TrafficSignalConfiguration.model_validate(
            {"trafficSignalId": "TS-001", "trafficSignalType": "notAValidType"}
        )


def test_extra_field_rejected():
    with pytest.raises(ValidationError):
        TrafficSignalConfiguration.model_validate(
            {"trafficSignalId": "TS-001", "surprise": "bad"}
        )


def test_descriptive_fields_round_trip():
    payload = {
        "trafficSignalId": "TS-001",
        "configurationDate": "2024-06-01T00:00:00Z",
        "subArea": "City Centre West",
        "userCodeNumber": "OP-REF-42",
    }
    c = TrafficSignalConfiguration.model_validate(payload)
    assert c.sub_area == "City Centre West"
    assert c.user_code_number == "OP-REF-42"
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["configurationDate"].startswith("2024-06-01")
    assert dumped["subArea"] == "City Centre West"


def test_user_code_number_max_length_enforced():
    with pytest.raises(ValidationError):
        TrafficSignalConfiguration.model_validate(
            {"trafficSignalId": "TS-001", "userCodeNumber": "x" * 33}
        )


def test_descriptive_fields_remain_optional():
    c = TrafficSignalConfiguration.model_validate({"trafficSignalId": "TS-001"})
    assert c.configuration_date is None
    assert c.sub_area is None
    assert c.user_code_number is None
