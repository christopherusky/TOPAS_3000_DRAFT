# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Air-quality type identifier: minimal construction and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.air_quality.type_id."""

from topas_3000_model.air_quality.type_id import AirQualityType


def test_air_quality_type_minimal():
    t = AirQualityType(type_id="1", description="AQ Reference Monitor")
    assert t.type_id == "1"


def test_air_quality_type_dumps_camel_case():
    t = AirQualityType(type_id="1", description="AQ Reference Monitor")
    dumped = t.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["typeId"] == "1"
    assert dumped["description"] == "AQ Reference Monitor"
