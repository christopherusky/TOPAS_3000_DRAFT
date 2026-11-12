# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Access-control type identifier: minimal and full construction,
#              and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.access_control.type_id."""

from topas_3000_model.access_control.type_id import AccessControlType


def test_access_control_type_minimal():
    t = AccessControlType(
        type_id="1",
        description="Bollard",
    )
    assert t.type_id == "1"
    assert t.description == "Bollard"
    assert t.notes is None
    assert t.specification_link is None


def test_access_control_type_with_all_fields():
    t = AccessControlType(
        type_id="2",
        description="Barrier",
        notes="Rising arm barrier for car park entrance",
        specification_link="https://example.com/barrier-spec",
        extensions={"manufacturer": "acme"},
    )
    assert t.notes == "Rising arm barrier for car park entrance"
    assert str(t.specification_link) == "https://example.com/barrier-spec"


def test_access_control_type_dumps_camel_case():
    t = AccessControlType(
        type_id="1",
        description="Bollard",
    )
    dumped = t.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["typeId"] == "1"
    assert dumped["description"] == "Bollard"
