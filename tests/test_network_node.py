# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: NetworkNode: minimal and full construction, empty-id rejection,
#              and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.network.node."""

import pytest
from pydantic import ValidationError

from topas_3000_model.common.geometry import GeoJsonPoint
from topas_3000_model.network.node import NetworkNode


def test_network_node_minimal():
    n = NetworkNode(node_id="node-001")
    assert n.node_id == "node-001"
    assert n.description is None
    assert n.location is None


def test_network_node_with_all_fields():
    n = NetworkNode(
        node_id="node-001",
        description="High Street / Park Road junction",
        location=GeoJsonPoint(coordinates=(-1.5, 53.5)),
        extensions={"signal_id": "sig-042"},
    )
    assert n.description == "High Street / Park Road junction"
    assert n.location.coordinates == (-1.5, 53.5)
    assert n.extensions == {"signal_id": "sig-042"}


def test_network_node_rejects_empty_id():
    with pytest.raises(ValidationError):
        NetworkNode(node_id="")


def test_network_node_dumps_camel_case():
    n = NetworkNode(
        node_id="node-001",
        description="Junction A",
        location=GeoJsonPoint(coordinates=(-1.5, 53.5)),
    )
    dumped = n.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["nodeId"] == "node-001"
    assert dumped["description"] == "Junction A"
    assert dumped["location"]["type"] == "Point"
    assert dumped["location"]["coordinates"] == [-1.5, 53.5]
