# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.2
# Description: GeoJSON Point and LineString: longitude/latitude ordering,
#              multi-point lines, rejection of single-point and empty lines,
#              literal type fields, WGS84 range and finiteness limits on
#              every position, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.common.geometry."""

import pytest
from pydantic import ValidationError

from topas_3000_model.common.geometry import GeoJsonLineString, GeoJsonPoint


def test_geojson_point_minimal():
    p = GeoJsonPoint(coordinates=(-1.5, 53.5))
    assert p.type == "Point"
    assert p.coordinates == (-1.5, 53.5)


def test_geojson_point_lon_lat_order():
    """GeoJSON uses [longitude, latitude] order per RFC 7946."""
    p = GeoJsonPoint(coordinates=(-0.1278, 51.5074))  # London
    lon, lat = p.coordinates
    assert lon == -0.1278
    assert lat == 51.5074


def test_geojson_point_dumps_camel_case():
    p = GeoJsonPoint(coordinates=(-1.5, 53.5))
    dumped = p.model_dump(by_alias=True, mode="json")
    assert dumped["type"] == "Point"
    assert dumped["coordinates"] == [-1.5, 53.5]


def test_geojson_linestring_minimal():
    ls = GeoJsonLineString(
        coordinates=[(-1.5, 53.5), (-1.6, 53.6)],
    )
    assert ls.type == "LineString"
    assert len(ls.coordinates) == 2


def test_geojson_linestring_multi_point():
    ls = GeoJsonLineString(
        coordinates=[(-1.5, 53.5), (-1.55, 53.55), (-1.6, 53.6), (-1.65, 53.65)],
    )
    assert len(ls.coordinates) == 4


def test_geojson_linestring_rejects_single_point():
    with pytest.raises(ValidationError):
        GeoJsonLineString(coordinates=[(-1.5, 53.5)])


def test_geojson_linestring_rejects_empty():
    with pytest.raises(ValidationError):
        GeoJsonLineString(coordinates=[])


def test_geojson_linestring_dumps_camel_case():
    ls = GeoJsonLineString(
        coordinates=[(-1.5, 53.5), (-1.6, 53.6)],
    )
    dumped = ls.model_dump(by_alias=True, mode="json")
    assert dumped["type"] == "LineString"
    assert dumped["coordinates"] == [[-1.5, 53.5], [-1.6, 53.6]]


def test_geojson_point_type_is_literal():
    """Type field must be exactly 'Point'."""
    with pytest.raises(ValidationError):
        GeoJsonPoint(type="Feature", coordinates=(-1.5, 53.5))


def test_geojson_linestring_type_is_literal():
    """Type field must be exactly 'LineString'."""
    with pytest.raises(ValidationError):
        GeoJsonLineString(type="Polygon", coordinates=[(-1.5, 53.5), (-1.6, 53.6)])


@pytest.mark.parametrize(
    "coords",
    [
        (500.0, -999.0),
        (180.1, 0.0),
        (-180.1, 0.0),
        (0.0, 90.1),
        (0.0, -90.1),
        (float("nan"), 1.0),
        (1.0, float("nan")),
        (float("inf"), 1.0),
        (1.0, float("-inf")),
    ],
)
def test_geojson_point_rejects_out_of_range_or_non_finite(coords):
    with pytest.raises(ValidationError):
        GeoJsonPoint(coordinates=coords)


@pytest.mark.parametrize("coords", [(180.0, 90.0), (-180.0, -90.0), (0.0, 0.0)])
def test_geojson_point_accepts_range_limits(coords):
    assert GeoJsonPoint(coordinates=coords).coordinates == coords


def test_geojson_linestring_rejects_any_bad_position():
    with pytest.raises(ValidationError):
        GeoJsonLineString(coordinates=[(-1.5, 53.5), (500.0, 53.6)])
    with pytest.raises(ValidationError):
        GeoJsonLineString(coordinates=[(-1.5, 53.5), (-1.6, float("nan"))])


def test_geojson_swapped_lon_lat_is_caught_outside_the_overlap():
    """Latitude 120 is impossible, so a swapped (lat, lon) pair is rejected."""
    with pytest.raises(ValidationError):
        GeoJsonPoint(coordinates=(53.5, 120.0))
