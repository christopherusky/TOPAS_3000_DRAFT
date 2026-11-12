# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Shared fault support: the AcknowledgementState value set and
#              its parsing, and FaultTypeDefinition construction with
#              severity, missing-description rejection, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.fault_support."""

import pytest
from pydantic import ValidationError

from topas_3000_model.fault_support.enums import AcknowledgementState
from topas_3000_model.fault_support.fault import FaultTypeDefinition


def test_acknowledgement_state_values():
    assert AcknowledgementState.NOT_ACKNOWLEDGED.value == "notAcknowledged"
    assert AcknowledgementState.ACKNOWLEDGED.value == "acknowledged"
    assert AcknowledgementState.DEFERRED.value == "deferred"
    assert AcknowledgementState.RECORDED_FOR_ACTION.value == "recordedForAction"


def test_acknowledgement_state_parse():
    assert AcknowledgementState("acknowledged") == AcknowledgementState.ACKNOWLEDGED


def test_fault_type_definition_minimal():
    ft = FaultTypeDefinition(fault_type_id="101", description="Loop failure")
    assert ft.fault_type_id == "101"
    assert ft.description == "Loop failure"
    assert ft.severity is None


def test_fault_type_definition_with_severity():
    ft = FaultTypeDefinition(fault_type_id="101", description="Loop failure", severity=5)
    assert ft.severity == 5


def test_fault_type_definition_rejects_missing_description():
    with pytest.raises(ValidationError):
        FaultTypeDefinition(fault_type_id="101")


def test_fault_type_definition_dumps_camel_case():
    ft = FaultTypeDefinition(fault_type_id="101", description="Loop failure", severity=5)
    dumped = ft.model_dump(by_alias=True, exclude_none=True)
    assert dumped == {"faultTypeId": "101", "description": "Loop failure", "severity": 5}
