# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Transport-route type identifier: minimal construction and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.transport_route.type_id."""

from topas_3000_model.transport_route.type_id import TransportRouteType


def test_transport_route_type_minimal():
    t = TransportRouteType(type_id="1", description="Road route")
    assert t.type_id == "1"


def test_transport_route_type_dumps_camel_case():
    t = TransportRouteType(type_id="1", description="Road route")
    dumped = t.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["typeId"] == "1"
    assert dumped["description"] == "Road route"
