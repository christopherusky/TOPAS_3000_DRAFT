# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.3
# Description: Coordinate and Point: latitude and longitude range enforcement,
#              optional altitude with its bounds, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.common.location."""

import pytest
from pydantic import ValidationError

from topas_3000_model.common.location import Coordinates, Point


def test_coordinate_valid():
    c = Coordinates(latitude=51.5074, longitude=-0.1278)
    assert c.latitude == 51.5074
    assert c.longitude == -0.1278


def test_coordinate_out_of_range_latitude():
    with pytest.raises(ValidationError):
        Coordinates(latitude=91.0, longitude=0.0)


def test_coordinate_out_of_range_longitude():
    with pytest.raises(ValidationError):
        Coordinates(latitude=0.0, longitude=181.0)


def test_point_with_altitude():
    p = Point(coordinates=Coordinates(latitude=51.5, longitude=-0.1), altitude_meters=42.0)
    assert p.altitude_meters == 42.0


def test_point_without_altitude():
    p = Point(coordinates=Coordinates(latitude=51.5, longitude=-0.1))
    assert p.altitude_meters is None


def test_point_dumps_camel_case():
    p = Point(coordinates=Coordinates(latitude=51.5, longitude=-0.1), altitude_meters=42.0)
    dumped = p.model_dump(by_alias=True, exclude_none=True)
    assert dumped == {
        "coordinates": {"latitude": 51.5, "longitude": -0.1},
        "altitudeMeters": 42.0,
    }


def test_point_rejects_altitude_below_minimum():
    with pytest.raises(ValidationError):
        Point(coordinates=Coordinates(latitude=0.0, longitude=0.0), altitude_meters=-1001.0)


def test_point_rejects_altitude_above_maximum():
    with pytest.raises(ValidationError):
        Point(coordinates=Coordinates(latitude=0.0, longitude=0.0), altitude_meters=5001.0)
