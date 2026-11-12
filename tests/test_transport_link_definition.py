# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.3
# Description: Transport-link definition: minimal construction, location,
#              empty-id rejection, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.transport_link.definition."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.common.location import Coordinates, Point
from topas_3000_model.transport_link.definition import TransportLinkDefinition


NOW = datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc)


def test_tl_definition_minimal():
    d = TransportLinkDefinition(
        transport_link_id="tl-001",
        type_id="1",
        creation_date=NOW,
    )
    assert d.transport_link_id == "tl-001"
    assert d.type_id == "1"
    assert d.location is None


def test_tl_definition_with_location():
    d = TransportLinkDefinition(
        transport_link_id="tl-001",
        type_id="1",
        creation_date=NOW,
        location=Point(coordinates=Coordinates(latitude=51.5, longitude=-0.1)),
        short_description="A1 northbound",
    )
    assert d.location.coordinates.latitude == 51.5
    assert d.short_description == "A1 northbound"


def test_tl_definition_rejects_empty_id():
    with pytest.raises(ValidationError):
        TransportLinkDefinition(
            transport_link_id="",
            type_id="1",
            creation_date=NOW,
        )


def test_tl_definition_dumps_camel_case():
    d = TransportLinkDefinition(
        transport_link_id="tl-001",
        type_id="1",
        creation_date=NOW,
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert "transportLinkId" in dumped
    assert "typeId" in dumped
    assert "creationDate" in dumped
