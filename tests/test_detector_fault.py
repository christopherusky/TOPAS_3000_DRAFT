# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.2
# Description: Detector fault payloads across the acknowledged and cleared
#              states, with rejection of a missing creation date, and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.detector.fault."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.detector.fault import DetectorFault
from topas_3000_model.fault_support.enums import AcknowledgementState


NOW = datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc)


def test_detector_fault_minimal():
    f = DetectorFault(
        detector_id="det-001",
        fault_id=101,
        fault_type_id="5",
        creation_date=NOW,
    )
    assert f.detector_id == "det-001"
    assert f.fault_id == 101
    assert f.fault_type_id == "5"
    assert f.cleared_date is None
    assert f.acknowledgement_state is None


def test_detector_fault_acknowledged():
    f = DetectorFault(
        detector_id="det-001",
        fault_id=101,
        fault_type_id="5",
        creation_date=NOW,
        acknowledgement_state=AcknowledgementState.ACKNOWLEDGED,
        acknowledgement_date=NOW,
    )
    assert f.acknowledgement_state == AcknowledgementState.ACKNOWLEDGED


def test_detector_fault_cleared():
    f = DetectorFault(
        detector_id="det-001",
        fault_id=101,
        fault_type_id="5",
        creation_date=NOW,
        cleared_date=NOW,
        description="Loop replaced",
    )
    assert f.cleared_date == NOW
    assert f.description == "Loop replaced"


def test_detector_fault_rejects_missing_creation_date():
    with pytest.raises(ValidationError):
        DetectorFault(detector_id="det-001", fault_id=101, fault_type_id="5")


def test_detector_fault_dumps_camel_case():
    f = DetectorFault(
        detector_id="det-001",
        fault_id=101,
        fault_type_id="5",
        creation_date=NOW,
        acknowledgement_state=AcknowledgementState.ACKNOWLEDGED,
    )
    dumped = f.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["detectorId"] == "det-001"
    assert dumped["faultTypeId"] == "5"
    assert dumped["acknowledgementState"] == "acknowledged"
