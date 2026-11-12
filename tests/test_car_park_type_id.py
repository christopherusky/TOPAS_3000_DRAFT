# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Car-park type identifier: minimal and full construction,
#              rejection of a missing description and an invalid URL, and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.car_park.type_id."""

import pytest
from pydantic import ValidationError

from topas_3000_model.car_park.type_id import CarParkType


def test_car_park_type_minimal():
    t = CarParkType(type_id="1", description="Multi-storey")
    assert t.type_id == "1"
    assert t.description == "Multi-storey"
    assert t.notes is None
    assert t.specification_link is None


def test_car_park_type_full():
    t = CarParkType(
        type_id="2",
        description="Surface",
        notes="Open air, no height restrictions",
        specification_link="https://example.invalid/surface-spec",
    )
    assert t.notes == "Open air, no height restrictions"


def test_car_park_type_rejects_missing_description():
    with pytest.raises(ValidationError):
        CarParkType(type_id="1")


def test_car_park_type_invalid_url():
    with pytest.raises(ValidationError):
        CarParkType(type_id="1", description="Surface", specification_link="not a url")


def test_car_park_type_dumps_camel_case():
    t = CarParkType(type_id="1", description="Multi-storey")
    dumped = t.model_dump(by_alias=True, exclude_none=True)
    assert dumped == {"typeId": "1", "description": "Multi-storey"}
