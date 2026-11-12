# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: NetworkLink: minimal and full construction, rejection of empty
#              link, from-node and to-node ids, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.network.link."""

import pytest
from pydantic import ValidationError

from topas_3000_model.common.geometry import GeoJsonLineString
from topas_3000_model.network.link import NetworkLink


def test_network_link_minimal():
    lk = NetworkLink(
        link_id="link-001",
        from_node_id="node-001",
        to_node_id="node-002",
    )
    assert lk.link_id == "link-001"
    assert lk.from_node_id == "node-001"
    assert lk.to_node_id == "node-002"
    assert lk.geometry is None


def test_network_link_with_all_fields():
    lk = NetworkLink(
        link_id="link-001",
        from_node_id="node-001",
        to_node_id="node-002",
        description="High Street northbound",
        geometry=GeoJsonLineString(
            coordinates=[(-1.5, 53.5), (-1.51, 53.51), (-1.52, 53.52)],
        ),
        extensions={"road_class": "A"},
    )
    assert lk.description == "High Street northbound"
    assert len(lk.geometry.coordinates) == 3


def test_network_link_rejects_empty_link_id():
    with pytest.raises(ValidationError):
        NetworkLink(link_id="", from_node_id="node-001", to_node_id="node-002")


def test_network_link_rejects_empty_from_node():
    with pytest.raises(ValidationError):
        NetworkLink(link_id="link-001", from_node_id="", to_node_id="node-002")


def test_network_link_rejects_empty_to_node():
    with pytest.raises(ValidationError):
        NetworkLink(link_id="link-001", from_node_id="node-001", to_node_id="")


def test_network_link_dumps_camel_case():
    lk = NetworkLink(
        link_id="link-001",
        from_node_id="node-001",
        to_node_id="node-002",
        geometry=GeoJsonLineString(
            coordinates=[(-1.5, 53.5), (-1.6, 53.6)],
        ),
    )
    dumped = lk.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["linkId"] == "link-001"
    assert dumped["fromNodeId"] == "node-001"
    assert dumped["toNodeId"] == "node-002"
    assert dumped["geometry"]["type"] == "LineString"
    assert len(dumped["geometry"]["coordinates"]) == 2
