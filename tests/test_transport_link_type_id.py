# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.2
# Description: Transport-link and link-status type identifiers: minimal and
#              full construction, rejection of a missing description and an
#              invalid URL, union dispatch between the two with unknown-kind
#              rejection, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.transport_link.type_id."""

import pytest
from pydantic import TypeAdapter, ValidationError

from topas_3000_model.transport_link.type_id import (
    LinkStatusType,
    TransportLinkType,
    TransportLinkTypeUnion,
)


def test_tl_type_id_minimal():
    t = TransportLinkType(type_id="1", description="SCOOT")
    assert t.kind == "transportLink"
    assert t.type_id == "1"
    assert t.description == "SCOOT"
    assert t.notes is None
    assert t.specification_link is None


def test_tl_type_id_full():
    t = TransportLinkType(
        type_id="2",
        description="ANPR",
        notes="Outer ring road cameras",
        specification_link="https://example.invalid/anpr-spec",
    )
    assert t.notes == "Outer ring road cameras"


def test_tl_type_id_rejects_missing_description():
    with pytest.raises(ValidationError):
        TransportLinkType(type_id="1")


def test_tl_type_id_invalid_url():
    with pytest.raises(ValidationError):
        TransportLinkType(type_id="1", description="SCOOT", specification_link="not a url")


def test_link_status_type_id_minimal():
    t = LinkStatusType(type_id="10", description="Congested")
    assert t.kind == "linkStatus"
    assert t.type_id == "10"
    assert t.description == "Congested"
    assert t.notes is None


def test_link_status_type_id_full():
    t = LinkStatusType(
        type_id="11",
        description="Blocked",
        notes="No passable lane",
    )
    assert t.notes == "No passable lane"


def test_link_status_type_id_rejects_missing_description():
    with pytest.raises(ValidationError):
        LinkStatusType(type_id="1")


def test_tl_type_union_routes_transport_link():
    adapter = TypeAdapter(TransportLinkTypeUnion)
    instance = adapter.validate_python({
        "kind": "transportLink",
        "typeId": "1",
        "description": "SCOOT",
    })
    assert isinstance(instance, TransportLinkType)
    assert instance.type_id == "1"


def test_tl_type_union_routes_link_status():
    adapter = TypeAdapter(TransportLinkTypeUnion)
    instance = adapter.validate_python({
        "kind": "linkStatus",
        "typeId": "10",
        "description": "Congested",
    })
    assert isinstance(instance, LinkStatusType)
    assert instance.type_id == "10"


def test_tl_type_union_rejects_unknown_kind():
    adapter = TypeAdapter(TransportLinkTypeUnion)
    with pytest.raises(ValidationError):
        adapter.validate_python({
            "kind": "bogus",
            "typeId": "1",
            "description": "Nope",
        })


def test_tl_type_id_dumps_camel_case():
    t = TransportLinkType(type_id="1", description="SCOOT")
    dumped = t.model_dump(by_alias=True, exclude_none=True)
    assert dumped == {"kind": "transportLink", "typeId": "1", "description": "SCOOT"}


def test_link_status_type_id_dumps_camel_case():
    t = LinkStatusType(type_id="10", description="Congested")
    dumped = t.model_dump(by_alias=True, exclude_none=True)
    assert dumped == {"kind": "linkStatus", "typeId": "10", "description": "Congested"}
