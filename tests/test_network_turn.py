# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: NetworkTurn: allowed and prohibited turns, geometry, full-field
#              construction, rejection of empty turn and from-link ids, and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.network.turn."""

import pytest
from pydantic import ValidationError

from topas_3000_model.common.geometry import GeoJsonLineString
from topas_3000_model.network.turn import NetworkTurn


def test_network_turn_allowed():
    t = NetworkTurn(
        turn_id="turn-001",
        from_link_id="link-001",
        to_link_id="link-002",
        allowed=True,
    )
    assert t.allowed is True
    assert t.description is None
    assert t.geometry is None


def test_network_turn_prohibited():
    t = NetworkTurn(
        turn_id="turn-002",
        from_link_id="link-001",
        to_link_id="link-003",
        allowed=False,
        description="No right turn",
    )
    assert t.allowed is False
    assert t.description == "No right turn"


def test_network_turn_with_geometry():
    t = NetworkTurn(
        turn_id="turn-001",
        from_link_id="link-001",
        to_link_id="link-002",
        allowed=True,
        geometry=GeoJsonLineString(
            coordinates=[(-1.5, 53.5), (-1.501, 53.501), (-1.502, 53.5)],
        ),
    )
    assert len(t.geometry.coordinates) == 3


def test_network_turn_with_all_fields():
    t = NetworkTurn(
        turn_id="turn-001",
        from_link_id="link-001",
        to_link_id="link-002",
        allowed=True,
        description="Left turn onto Park Road",
        geometry=GeoJsonLineString(
            coordinates=[(-1.5, 53.5), (-1.502, 53.5)],
        ),
        extensions={"signal_phase": 3},
    )
    assert t.extensions == {"signal_phase": 3}


def test_network_turn_rejects_empty_turn_id():
    with pytest.raises(ValidationError):
        NetworkTurn(
            turn_id="", from_link_id="link-001",
            to_link_id="link-002", allowed=True,
        )


def test_network_turn_rejects_empty_from_link():
    with pytest.raises(ValidationError):
        NetworkTurn(
            turn_id="turn-001", from_link_id="",
            to_link_id="link-002", allowed=True,
        )


def test_network_turn_dumps_camel_case():
    t = NetworkTurn(
        turn_id="turn-001",
        from_link_id="link-001",
        to_link_id="link-002",
        allowed=False,
        description="No U-turn",
    )
    dumped = t.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["turnId"] == "turn-001"
    assert dumped["fromLinkId"] == "link-001"
    assert dumped["toLinkId"] == "link-002"
    assert dumped["allowed"] is False
    assert dumped["description"] == "No U-turn"
