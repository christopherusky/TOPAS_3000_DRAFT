# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.2
# Description: NetworkZone: minimal and full construction, link-order
#              preservation, empty-id rejection, type id, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.network.zone."""

import pytest
from pydantic import ValidationError

from topas_3000_model.common.geometry import GeoJsonLineString
from topas_3000_model.network.zone import NetworkZone


def test_network_zone_minimal():
    z = NetworkZone(zone_id="zone-001")
    assert z.zone_id == "zone-001"
    assert z.type_id is None
    assert z.link_ids is None
    assert z.permanent is None


def test_network_zone_with_all_fields():
    z = NetworkZone(
        zone_id="zone-001",
        type_id="1",
        description="City centre traffic management zone",
        permanent=True,
        link_ids=["link-001", "link-002", "link-003"],
        geometry=GeoJsonLineString(
            coordinates=[(-1.5, 53.5), (-1.51, 53.51), (-1.5, 53.52)],
        ),
        extensions={"zone_type": "congestion"},
    )
    assert z.type_id == "1"
    assert z.permanent is True
    assert len(z.link_ids) == 3
    assert z.link_ids[0] == "link-001"


def test_network_zone_preserves_link_order():
    """Link order defines the zone boundary sequence."""
    z = NetworkZone(
        zone_id="zone-001",
        link_ids=["link-C", "link-A", "link-B"],
    )
    assert z.link_ids == ["link-C", "link-A", "link-B"]


def test_network_zone_rejects_empty_id():
    with pytest.raises(ValidationError):
        NetworkZone(zone_id="")


def test_network_zone_dumps_camel_case():
    z = NetworkZone(
        zone_id="zone-001",
        description="Test zone",
        permanent=False,
        link_ids=["link-001", "link-002"],
    )
    dumped = z.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["zoneId"] == "zone-001"
    assert dumped["permanent"] is False
    assert dumped["linkIds"] == ["link-001", "link-002"]


def test_network_zone_with_type_id():
    z = NetworkZone(zone_id="zone-001", type_id="2")
    dumped = z.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["typeId"] == "2"


def test_network_zone_rejects_empty_link_list():
    with pytest.raises(ValidationError):
        NetworkZone(zone_id="zone-001", link_ids=[])


def test_network_zone_rejects_duplicate_links():
    with pytest.raises(ValidationError):
        NetworkZone(zone_id="zone-001", link_ids=["l-1", "l-2", "l-1"])
