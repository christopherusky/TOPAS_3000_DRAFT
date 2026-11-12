# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: CarParkTariffConfiguration, per-day-type and per-time-band
#              pricing: minimal and full construction, rejection of
#              a missing id and negative cost, acceptance of zero cost, and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for CarParkTariffConfiguration — per-day-type / per-time-band car park pricing entry."""

from __future__ import annotations

import pytest
from pydantic import ValidationError

from topas_3000_model.car_park.definition import CarParkTariffConfiguration


def test_tariff_minimal():
    t = CarParkTariffConfiguration(car_park_id="cp-001")
    assert t.car_park_id == "cp-001"
    assert t.day_type_id is None
    assert t.time_period is None
    assert t.cost is None


def test_tariff_with_all_fields():
    t = CarParkTariffConfiguration(
        car_park_id="cp-001",
        day_type_id="day-weekday",
        time_period="08:00-18:00",
        cost=4.50,
        extensions={"currency": "GBP"},
    )
    assert t.day_type_id == "day-weekday"
    assert t.time_period == "08:00-18:00"
    assert t.cost == 4.50
    assert t.extensions == {"currency": "GBP"}


def test_tariff_rejects_missing_car_park_id():
    with pytest.raises(ValidationError):
        CarParkTariffConfiguration()


def test_tariff_rejects_negative_cost():
    with pytest.raises(ValidationError):
        CarParkTariffConfiguration(car_park_id="cp-001", cost=-1.0)


def test_tariff_accepts_zero_cost():
    t = CarParkTariffConfiguration(car_park_id="cp-001", cost=0.0)
    assert t.cost == 0.0


def test_tariff_dumps_camel_case():
    t = CarParkTariffConfiguration(
        car_park_id="cp-001",
        day_type_id="day-weekday",
        time_period="08:00-18:00",
        cost=4.50,
    )
    dumped = t.model_dump(by_alias=True)
    assert dumped["carParkId"] == "cp-001"
    assert dumped["dayTypeId"] == "day-weekday"
    assert dumped["timePeriod"] == "08:00-18:00"
    assert dumped["cost"] == 4.50
