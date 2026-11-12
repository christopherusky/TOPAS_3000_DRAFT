# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: VMS type identifier: minimal and full construction, and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.vms.type_id."""

from topas_3000_model.vms.type_id import VmsType


def test_vms_type_minimal():
    t = VmsType(
        type_id="1",
        description="Matrix sign",
    )
    assert t.type_id == "1"
    assert t.description == "Matrix sign"
    assert t.notes is None
    assert t.specification_link is None


def test_vms_type_with_all_fields():
    t = VmsType(
        type_id="2",
        description="MS4 motorway sign",
        notes="Full matrix sign with graphics capability",
        specification_link="https://example.com/ms4-spec",
        extensions={"manufacturer": "acme"},
    )
    assert t.notes == "Full matrix sign with graphics capability"
    assert str(t.specification_link) == "https://example.com/ms4-spec"


def test_vms_type_dumps_camel_case():
    t = VmsType(
        type_id="1",
        description="Matrix sign",
    )
    dumped = t.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["typeId"] == "1"
    assert dumped["description"] == "Matrix sign"
