# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: CCTV type identifier: minimal construction and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.cctv.type_id."""

from topas_3000_model.cctv.type_id import CctvType


def test_cctv_type_minimal():
    t = CctvType(type_id="1", description="Colour CCD")
    assert t.type_id == "1"


def test_cctv_type_dumps_camel_case():
    t = CctvType(type_id="1", description="Colour CCD")
    dumped = t.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["typeId"] == "1"
    assert dumped["description"] == "Colour CCD"
