# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-06
# Version:     1.3
# Description: TrafficSignalFault (TS004.0064bis:2017 section 3.3): minimal
#              and full payloads, at least one of faultTypeId and the DATEX
#              enum, rejection of a missing last-update time or
#              identifier, templated
#              faults carrying parameters, and rejection of unknown fault
#              types and extra fields.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for TrafficSignalFault (TS004.0064bis:2017 §3.3)."""

import pytest
from pydantic import ValidationError

from topas_3000_model.traffic_signal.enums import (
    FaultSeverityEnum,
    TrafficSignalFaultEnum,
    UrgencyEnum,
)
from topas_3000_model.traffic_signal.fault import TrafficSignalFault


def _minimal() -> dict:
    return {
        "trafficSignalId": "TS-001",
        "faultTypeId": "A1",
        "trafficSignalFaultType": "lampFault",
        "faultLastUpdateTime": "2026-04-22T10:00:00+00:00",
    }


def test_minimal_payload_valid():
    f = TrafficSignalFault.model_validate(_minimal())
    assert f.traffic_signal_fault_type is TrafficSignalFaultEnum.LAMP_FAULT
    assert f.fault_identifier is None
    assert f.fault_severity is None
    assert f.traffic_signal_fault_urgency is None
    assert f.traffic_signal_manufacturer_fault_code is None
    assert f.traffic_signal_fault_clearance_time is None
    assert f.fault_creation_time is None
    assert f.fault_description is None
    assert f.extensions is None


def test_full_payload_valid():
    payload = {
        "trafficSignalId": "TS-001",
        "faultTypeId": "A1",
        "trafficSignalFaultType": "electronicsFault",
        "faultLastUpdateTime": "2026-04-22T10:05:00Z",
        "faultIdentifier": "FLT-42",
        "faultDescription": "Conflict detector tripped",
        "faultCreationTime": "2026-04-22T09:59:00Z",
        "faultSeverity": "high",
        "trafficSignalFaultClearanceTime": "2026-04-22T10:15:00Z",
        "trafficSignalFaultUrgency": "extremelyUrgent",
        "trafficSignalManufacturerFaultCode": "SIE-E042",
        "extensions": {"cabinetTempC": 42.5},
    }
    f = TrafficSignalFault.model_validate(payload)
    assert f.fault_severity is FaultSeverityEnum.HIGH
    assert f.traffic_signal_fault_urgency is UrgencyEnum.EXTREMELY_URGENT
    assert f.traffic_signal_manufacturer_fault_code == "SIE-E042"
    assert f.extensions == {"cabinetTempC": 42.5}
    assert f.model_dump(by_alias=True, exclude_none=True, mode="json") == payload


def test_a_2017_profile_fault_without_fault_type_id_is_accepted():
    """TS004.0064bis:2017 requires trafficSignalFaultType and has no
    faultTypeId, so a fault written to that profile carries only the enum.
    It is accepted (a relaxation)."""
    payload = _minimal()
    del payload["faultTypeId"]
    fault = TrafficSignalFault.model_validate(payload)
    assert fault.fault_type_id is None
    assert fault.traffic_signal_fault_type is TrafficSignalFaultEnum.LAMP_FAULT


def test_a_fault_with_neither_classification_is_rejected():
    """At least one of faultTypeId and trafficSignalFaultType."""
    payload = _minimal()
    del payload["faultTypeId"]
    del payload["trafficSignalFaultType"]
    with pytest.raises(ValidationError, match="faultTypeId, trafficSignalFaultType or both"):
        TrafficSignalFault.model_validate(payload)


def test_the_schema_states_the_same_rule():
    """The rule reaches the JSON Schema, so non-Python SDKs and schema
    validators apply it too, not only Python."""
    schema = TrafficSignalFault.model_json_schema()
    assert schema["anyOf"] == [{"required": ["faultTypeId"]}, {"required": ["trafficSignalFaultType"]}]
    assert "faultTypeId" not in schema["required"]
    assert "trafficSignalFaultType" not in schema["required"]


def test_datex_enum_is_optional():
    """The coarse DATEX II bucket is retained for interoperability but no
    longer required — it bucketed 36 of 46 published codes into `other`, so
    it cannot carry the classification on its own."""
    payload = _minimal()
    del payload["trafficSignalFaultType"]
    fault = TrafficSignalFault.model_validate(payload)
    assert fault.traffic_signal_fault_type is None
    assert fault.fault_type_id == "A1"


def test_templated_fault_carries_parameters():
    """A timing fault — the case a closed enum could not express."""
    payload = _minimal()
    payload["faultTypeId"] = "F3"
    payload["parameters"] = ["3", "4", "7"]
    fault = TrafficSignalFault.model_validate(payload)
    assert fault.parameters == ["3", "4", "7"]
    assert fault.model_dump(mode="json", exclude_none=True)["parameters"] == ["3", "4", "7"]


def test_missing_last_update_time_rejected():
    payload = _minimal()
    del payload["faultLastUpdateTime"]
    with pytest.raises(ValidationError):
        TrafficSignalFault.model_validate(payload)


def test_missing_identifier_rejected():
    payload = _minimal()
    del payload["trafficSignalId"]
    with pytest.raises(ValidationError):
        TrafficSignalFault.model_validate(payload)


def test_unknown_fault_type_rejected():
    with pytest.raises(ValidationError):
        TrafficSignalFault.model_validate(
            {**_minimal(), "trafficSignalFaultType": "notAFault"}
        )


def test_extra_field_rejected():
    with pytest.raises(ValidationError):
        TrafficSignalFault.model_validate({**_minimal(), "stray": "no"})
