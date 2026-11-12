# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.2
# Description: NetworkPath: minimal and full construction, link-order
#              preservation, rejection of negative initial and final trim and
#              of an empty id, type id, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.network.path."""

import pytest
from pydantic import ValidationError

from topas_3000_model.network.path import NetworkPath


def test_network_path_minimal():
    p = NetworkPath(path_id="path-001")
    assert p.path_id == "path-001"
    assert p.type_id is None
    assert p.link_ids is None
    assert p.initial_trim_meters is None
    assert p.final_trim_meters is None


def test_network_path_with_all_fields():
    p = NetworkPath(
        path_id="path-001",
        type_id="3",
        description="Bus route 42",
        permanent=True,
        link_ids=["link-010", "link-011", "link-012", "link-013"],
        initial_trim_meters=50.0,
        final_trim_meters=25.0,
        extensions={"route_number": "42"},
    )
    assert p.type_id == "3"
    assert len(p.link_ids) == 4
    assert p.initial_trim_meters == 50.0
    assert p.final_trim_meters == 25.0


def test_network_path_preserves_link_order():
    """Link order defines the route sequence."""
    p = NetworkPath(
        path_id="path-001",
        link_ids=["link-C", "link-A", "link-B"],
    )
    assert p.link_ids == ["link-C", "link-A", "link-B"]


def test_network_path_rejects_negative_initial_trim():
    with pytest.raises(ValidationError):
        NetworkPath(path_id="path-001", initial_trim_meters=-1.0)


def test_network_path_rejects_negative_final_trim():
    with pytest.raises(ValidationError):
        NetworkPath(path_id="path-001", final_trim_meters=-1.0)


def test_network_path_rejects_empty_id():
    with pytest.raises(ValidationError):
        NetworkPath(path_id="")


def test_network_path_dumps_camel_case():
    p = NetworkPath(
        path_id="path-001",
        permanent=True,
        link_ids=["link-010", "link-011"],
        initial_trim_meters=50.0,
        final_trim_meters=25.0,
    )
    dumped = p.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["pathId"] == "path-001"
    assert dumped["permanent"] is True
    assert dumped["linkIds"] == ["link-010", "link-011"]
    assert dumped["initialTrimMeters"] == 50.0
    assert dumped["finalTrimMeters"] == 25.0


def test_network_path_with_type_id():
    p = NetworkPath(path_id="path-001", type_id="5")
    dumped = p.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["typeId"] == "5"


def test_network_path_rejects_empty_link_list():
    with pytest.raises(ValidationError):
        NetworkPath(path_id="p-1", link_ids=[])


def test_network_path_allows_a_link_to_repeat():
    """A route can traverse the same link twice, so a path is not a set."""
    p = NetworkPath(path_id="p-1", link_ids=["l-1", "l-2", "l-1"])
    assert p.link_ids == ["l-1", "l-2", "l-1"]
