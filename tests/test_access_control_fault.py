# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-29
# Version:     1.2
# Description: Access-control fault payloads: minimal and full construction,
#              rejection of negative fault ids and severities, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.access_control.fault."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.access_control.fault import AccessControlFault
from topas_3000_model.fault_support.enums import AcknowledgementState, FaultStatus


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_access_control_fault_minimal():
    f = AccessControlFault(
        access_control_id="ac-001",
        fault_id=101,
        fault_type_id="5",
        creation_date=NOW,
    )
    assert f.fault_id == 101
    assert f.acknowledgement_state is None
    assert f.status is None


def test_access_control_fault_with_all_fields():
    f = AccessControlFault(
        access_control_id="ac-001",
        fault_id=101,
        fault_type_id="5",
        creation_date=NOW,
        last_updated=NOW,
        cleared_date=NOW,
        description="Bollard motor failure",
        acknowledgement_state=AcknowledgementState.ACKNOWLEDGED,
        acknowledgement_date=NOW,
        severity=3,
        status=FaultStatus.CLEARED,
        extensions={"ticket": "INC-123"},
    )
    assert f.acknowledgement_state == AcknowledgementState.ACKNOWLEDGED
    assert f.status == FaultStatus.CLEARED
    assert f.description == "Bollard motor failure"


def test_access_control_fault_rejects_negative_fault_id():
    with pytest.raises(ValidationError):
        AccessControlFault(
            access_control_id="ac-001",
            fault_id=-1,
            fault_type_id="5",
            creation_date=NOW,
        )


def test_access_control_fault_rejects_negative_severity():
    with pytest.raises(ValidationError):
        AccessControlFault(
            access_control_id="ac-001",
            fault_id=101,
            fault_type_id="5",
            creation_date=NOW,
            severity=-1,
        )


def test_access_control_fault_dumps_camel_case():
    f = AccessControlFault(
        access_control_id="ac-001",
        fault_id=101,
        fault_type_id="5",
        creation_date=NOW,
        acknowledgement_state=AcknowledgementState.ACKNOWLEDGED,
        status=FaultStatus.ACTIVE,
    )
    dumped = f.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["accessControlId"] == "ac-001"
    assert dumped["faultId"] == 101
    assert dumped["acknowledgementState"] == "acknowledged"
    assert dumped["status"] == "active"
