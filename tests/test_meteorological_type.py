# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Meteorological type identifier: minimal construction and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.meteorological.type_id."""

from topas_3000_model.meteorological.type_id import MeteorologicalType


def test_meteorological_type_minimal():
    t = MeteorologicalType(type_id="1", description="Automated Met Station")
    assert t.type_id == "1"


def test_meteorological_type_dumps_camel_case():
    t = MeteorologicalType(type_id="1", description="Automated Met Station")
    dumped = t.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["typeId"] == "1"
    assert dumped["description"] == "Automated Met Station"
