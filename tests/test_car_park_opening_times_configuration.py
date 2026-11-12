# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: CarParkOpeningTimesConfiguration, per-day-type open and close
#              times: minimal and full construction, missing-id
#              rejection, ISO time serialisation, and day-type reference
#              round-trip.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for CarParkOpeningTimesConfiguration — per-day-type car park open/close times."""

from __future__ import annotations

from datetime import time

import pytest
from pydantic import ValidationError

from topas_3000_model.car_park.definition import CarParkOpeningTimesConfiguration


def test_opening_times_minimal():
    o = CarParkOpeningTimesConfiguration(car_park_id="cp-001")
    assert o.car_park_id == "cp-001"
    assert o.opening_time is None
    assert o.closing_time is None


def test_opening_times_with_all_fields():
    o = CarParkOpeningTimesConfiguration(
        car_park_id="cp-001",
        day_type_id="day-weekday",
        opening_time=time(6, 0),
        closing_time=time(22, 0),
    )
    assert o.opening_time == time(6, 0)
    assert o.closing_time == time(22, 0)


def test_opening_times_rejects_missing_car_park_id():
    with pytest.raises(ValidationError):
        CarParkOpeningTimesConfiguration()


def test_opening_times_dumps_camel_case():
    o = CarParkOpeningTimesConfiguration(
        car_park_id="cp-001",
        day_type_id="day-weekday",
        opening_time=time(6, 0),
        closing_time=time(22, 0),
    )
    dumped = o.model_dump(by_alias=True)
    assert dumped["carParkId"] == "cp-001"
    assert dumped["dayTypeId"] == "day-weekday"


def test_opening_times_serialises_time_as_iso_string():
    o = CarParkOpeningTimesConfiguration(
        car_park_id="cp-001",
        opening_time=time(6, 0),
        closing_time=time(22, 0),
    )
    dumped = o.model_dump(mode="json", by_alias=True)
    assert dumped["openingTime"] == "06:00:00"
    assert dumped["closingTime"] == "22:00:00"


def test_opening_times_day_type_ref_roundtrip():
    o = CarParkOpeningTimesConfiguration(
        car_park_id="cp-001", day_type_id="day-bankholiday"
    )
    rehydrated = CarParkOpeningTimesConfiguration.model_validate(
        o.model_dump(by_alias=True)
    )
    assert rehydrated.day_type_id == "day-bankholiday"
