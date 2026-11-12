# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Transport-route definition, configuration and segment
#              configuration: minimal and full construction, end points,
#              rejection of an empty id and negative length, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.transport_route.definition."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.common.location import Coordinates, Point
from topas_3000_model.transport_route.definition import (
    TransportRouteConfiguration,
    TransportRouteDefinition,
    TransportRouteSegmentConfiguration,
)


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_transport_route_definition_minimal():
    d = TransportRouteDefinition(
        transport_route_id="tr-001",
        type_id="1",
        creation_date=NOW,
    )
    assert d.transport_route_id == "tr-001"
    assert d.location is None
    assert d.end_point is None


def test_transport_route_definition_with_end_point():
    d = TransportRouteDefinition(
        transport_route_id="tr-001",
        type_id="1",
        creation_date=NOW,
        location=Point(coordinates=Coordinates(latitude=51.5, longitude=-0.1)),
        end_point=Point(coordinates=Coordinates(latitude=51.6, longitude=-0.2)),
        short_description="A1 northbound corridor",
    )
    assert d.location.coordinates.latitude == 51.5
    assert d.end_point.coordinates.latitude == 51.6
    assert d.short_description == "A1 northbound corridor"


def test_transport_route_definition_rejects_empty_id():
    with pytest.raises(ValidationError):
        TransportRouteDefinition(
            transport_route_id="",
            type_id="1",
            creation_date=NOW,
        )


def test_transport_route_definition_dumps_camel_case():
    d = TransportRouteDefinition(
        transport_route_id="tr-001",
        type_id="1",
        creation_date=NOW,
        end_point=Point(coordinates=Coordinates(latitude=51.6, longitude=-0.2)),
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert "transportRouteId" in dumped
    assert "typeId" in dumped
    assert "endPoint" in dumped


def test_transport_route_configuration_minimal():
    c = TransportRouteConfiguration(
        transport_route_id="tr-001",
        configuration_date=NOW,
    )
    assert c.route_name is None
    assert c.route_length_meters is None


def test_transport_route_configuration_with_all_fields():
    c = TransportRouteConfiguration(
        transport_route_id="tr-001",
        configuration_date=NOW,
        route_name="A1 Corridor",
        route_start="Junction 5",
        route_via="Junction 6",
        route_end="Junction 7",
        route_length_meters=12500.0,
    )
    assert c.route_name == "A1 Corridor"
    assert c.route_length_meters == 12500.0


def test_transport_route_configuration_rejects_negative_length():
    with pytest.raises(ValidationError):
        TransportRouteConfiguration(
            transport_route_id="tr-001",
            configuration_date=NOW,
            route_length_meters=-1.0,
        )


def test_transport_route_configuration_dumps_camel_case():
    c = TransportRouteConfiguration(
        transport_route_id="tr-001",
        configuration_date=NOW,
        route_name="A1 Corridor",
        route_length_meters=12500.0,
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["transportRouteId"] == "tr-001"
    assert dumped["routeName"] == "A1 Corridor"
    assert dumped["routeLengthMeters"] == 12500.0


def test_transport_route_segment_configuration():
    s = TransportRouteSegmentConfiguration(
        transport_route_id="tr-001",
        configuration_date=NOW,
        route_segment_id=0,
        transport_link_id="tl-001",
    )
    assert s.route_segment_id == 0
    assert s.transport_link_id == "tl-001"


def test_transport_route_segment_rejects_negative_segment_id():
    with pytest.raises(ValidationError):
        TransportRouteSegmentConfiguration(
            transport_route_id="tr-001",
            configuration_date=NOW,
            route_segment_id=-1,
            transport_link_id="tl-001",
        )


def test_transport_route_segment_dumps_camel_case():
    s = TransportRouteSegmentConfiguration(
        transport_route_id="tr-001",
        configuration_date=NOW,
        route_segment_id=3,
        transport_link_id="tl-005",
    )
    dumped = s.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["transportRouteId"] == "tr-001"
    assert dumped["routeSegmentId"] == 3
    assert dumped["transportLinkId"] == "tl-005"
