# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.4
# Description: Car-park fault payloads across the acknowledged and cleared
#              states, with rejection of a missing creation date and negative
#              fault ids, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.car_park.fault."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.car_park.fault import CarParkFault
from topas_3000_model.fault_support.enums import AcknowledgementState


NOW = datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc)


def test_car_park_fault_minimal():
    f = CarParkFault(
        car_park_id="cp-001",
        fault_id=101,
        fault_type_id="5",
        creation_date=NOW,
    )
    assert f.car_park_id == "cp-001"
    assert f.fault_id == 101
    assert f.fault_type_id == "5"
    assert f.cleared_date is None
    assert f.acknowledgement_state is None


def test_car_park_fault_acknowledged():
    f = CarParkFault(
        car_park_id="cp-001",
        fault_id=101,
        fault_type_id="5",
        creation_date=NOW,
        acknowledgement_state=AcknowledgementState.ACKNOWLEDGED,
        acknowledgement_date=NOW,
    )
    assert f.acknowledgement_state == AcknowledgementState.ACKNOWLEDGED


def test_car_park_fault_cleared():
    f = CarParkFault(
        car_park_id="cp-001",
        fault_id=101,
        fault_type_id="5",
        creation_date=NOW,
        cleared_date=NOW,
        description="Counter recalibrated",
    )
    assert f.cleared_date == NOW
    assert f.description == "Counter recalibrated"


def test_car_park_fault_rejects_missing_creation_date():
    with pytest.raises(ValidationError):
        CarParkFault(
            car_park_id="cp-001",
            fault_id=101,
            fault_type_id="5",
        )


def test_car_park_fault_rejects_negative_fault_id():
    with pytest.raises(ValidationError):
        CarParkFault(
            car_park_id="cp-001",
            fault_id=-1,
            fault_type_id="5",
            creation_date=NOW,
        )


def test_car_park_fault_dumps_camel_case():
    f = CarParkFault(
        car_park_id="cp-001",
        fault_id=101,
        fault_type_id="5",
        creation_date=NOW,
        acknowledgement_state=AcknowledgementState.ACKNOWLEDGED,
    )
    dumped = f.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["carParkId"] == "cp-001"
    assert dumped["faultTypeId"] == "5"
    assert dumped["acknowledgementState"] == "acknowledged"
