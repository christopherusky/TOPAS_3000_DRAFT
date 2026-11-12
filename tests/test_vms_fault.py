# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-29
# Version:     1.3
# Description: VMS fault payloads: minimal and full construction, rejection of
#              negative fault ids and severities, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.vms.fault."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.fault_support.enums import AcknowledgementState, FaultStatus
from topas_3000_model.vms.fault import VmsFault


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_vms_fault_minimal():
    f = VmsFault(
        vms_id="vms-001",
        fault_id=101,
        fault_type_id="5",
        creation_date=NOW,
    )
    assert f.fault_id == 101
    assert f.acknowledgement_state is None
    assert f.status is None


def test_vms_fault_with_all_fields():
    f = VmsFault(
        vms_id="vms-001",
        fault_id=101,
        fault_type_id="5",
        creation_date=NOW,
        last_updated=NOW,
        cleared_date=NOW,
        description="LED panel failure sector 3",
        acknowledgement_state=AcknowledgementState.ACKNOWLEDGED,
        acknowledgement_date=NOW,
        severity=3,
        status=FaultStatus.CLEARED,
        extensions={"ticket": "INC-789"},
    )
    assert f.acknowledgement_state == AcknowledgementState.ACKNOWLEDGED
    assert f.status == FaultStatus.CLEARED
    assert f.description == "LED panel failure sector 3"


def test_vms_fault_rejects_negative_fault_id():
    with pytest.raises(ValidationError):
        VmsFault(
            vms_id="vms-001",
            fault_id=-1,
            fault_type_id="5",
            creation_date=NOW,
        )


def test_vms_fault_rejects_negative_severity():
    with pytest.raises(ValidationError):
        VmsFault(
            vms_id="vms-001",
            fault_id=101,
            fault_type_id="5",
            creation_date=NOW,
            severity=-1,
        )


def test_vms_fault_dumps_camel_case():
    f = VmsFault(
        vms_id="vms-001",
        fault_id=101,
        fault_type_id="5",
        creation_date=NOW,
        acknowledgement_state=AcknowledgementState.ACKNOWLEDGED,
        status=FaultStatus.ACTIVE,
    )
    dumped = f.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["vmsId"] == "vms-001"
    assert dumped["faultId"] == 101
    assert dumped["acknowledgementState"] == "acknowledged"
    assert dumped["status"] == "active"
