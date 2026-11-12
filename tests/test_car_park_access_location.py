# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: CarParkAccessLocation, the car-park entry and exit point: each variant, rejection of missing and unknown
#              entry/exit values, full-field construction, and camelCase
#              serialisation of both fields and enum values.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for CarParkAccessLocation — car park entry/exit point."""

from __future__ import annotations

import pytest
from pydantic import ValidationError

from topas_3000_model.car_park.definition import (
    CarParkAccessLocation,
    CarParkEntranceType,
)


def test_access_location_minimal_entry():
    a = CarParkAccessLocation(
        car_park_id="cp-001", entry_or_exit=CarParkEntranceType.ENTRY
    )
    assert a.car_park_id == "cp-001"
    assert a.entry_or_exit == CarParkEntranceType.ENTRY


@pytest.mark.parametrize(
    "variant",
    [
        CarParkEntranceType.ENTRY,
        CarParkEntranceType.EXIT,
        CarParkEntranceType.ENTRY_AND_EXIT,
    ],
)
def test_access_location_minimal_each_variant(variant):
    a = CarParkAccessLocation(car_park_id="cp-001", entry_or_exit=variant)
    assert a.entry_or_exit == variant


def test_access_location_rejects_missing_entry_or_exit():
    with pytest.raises(ValidationError):
        CarParkAccessLocation(car_park_id="cp-001")


def test_access_location_rejects_unknown_entry_or_exit():
    with pytest.raises(ValidationError):
        CarParkAccessLocation(car_park_id="cp-001", entry_or_exit="teleport")


def test_access_location_with_full_fields():
    a = CarParkAccessLocation(
        car_park_id="cp-001",
        entry_or_exit=CarParkEntranceType.ENTRY_AND_EXIT,
        entrance_location={
            "type": "Point",
            "coordinates": [-1.4628, 50.9097],
        },
        network_path_id="netpath-001",
    )
    assert list(a.entrance_location.coordinates) == [-1.4628, 50.9097]
    assert a.network_path_id == "netpath-001"


def test_access_location_dumps_camel_case():
    a = CarParkAccessLocation(
        car_park_id="cp-001",
        entry_or_exit=CarParkEntranceType.ENTRY,
        network_path_id="netpath-001",
    )
    dumped = a.model_dump(by_alias=True)
    assert dumped["carParkId"] == "cp-001"
    assert dumped["entryOrExit"] == "entry"
    assert dumped["networkPathId"] == "netpath-001"


def test_access_location_enum_values_are_camel_case():
    assert CarParkEntranceType.ENTRY.value == "entry"
    assert CarParkEntranceType.EXIT.value == "exit"
    assert CarParkEntranceType.ENTRY_AND_EXIT.value == "entryAndExit"
