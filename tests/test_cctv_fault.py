# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: CCTV fault payloads: minimal and full construction, negative
#              fault-id rejection, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.cctv.fault."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.cctv.fault import CctvFault
from topas_3000_model.fault_support.enums import AcknowledgementState, FaultStatus


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_cctv_fault_minimal():
    f = CctvFault(
        cctv_id="cam-001", fault_id=101, fault_type_id="5", creation_date=NOW,
    )
    assert f.fault_id == 101


def test_cctv_fault_with_all_fields():
    f = CctvFault(
        cctv_id="cam-001", fault_id=101, fault_type_id="5", creation_date=NOW,
        description="Camera offline",
        acknowledgement_state=AcknowledgementState.ACKNOWLEDGED,
        acknowledgement_date=NOW,
        severity=2, status=FaultStatus.ACTIVE,
    )
    assert f.acknowledgement_state == AcknowledgementState.ACKNOWLEDGED


def test_cctv_fault_rejects_negative_fault_id():
    with pytest.raises(ValidationError):
        CctvFault(cctv_id="cam-001", fault_id=-1, fault_type_id="5", creation_date=NOW)


def test_cctv_fault_dumps_camel_case():
    f = CctvFault(
        cctv_id="cam-001", fault_id=101, fault_type_id="5", creation_date=NOW,
        status=FaultStatus.ACTIVE,
    )
    dumped = f.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["cctvId"] == "cam-001"
    assert dumped["status"] == "active"
