# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Enhanced air-quality definition and enums: per-pollutant units,
#              empty-id rejection, camelCase serialisation, and the
#              ThresholdType, CheckedType and DataStatusType value sets.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.air_quality_enhanced.definition and enums."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.air_quality_enhanced.definition import AirQualityEnhancedDefinition
from topas_3000_model.air_quality_enhanced.enums import (
    CheckedType,
    DataStatusType,
    ThresholdType,
)
from topas_3000_model.common.location import Coordinates, Point


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_enhanced_definition_minimal():
    d = AirQualityEnhancedDefinition(
        air_quality_id="aq-001", type_id="1", creation_date=NOW,
    )
    assert d.air_quality_id == "aq-001"
    assert d.co_units is None
    assert d.pm2p5_units is None


def test_enhanced_definition_with_per_pollutant_units():
    d = AirQualityEnhancedDefinition(
        air_quality_id="aq-001", type_id="1", creation_date=NOW,
        co_units="ppm", no2_units="ppb", pm10_units="ug/m3",
        pm2p5_units="ug/m3", co2_units="ppm", noise_units="dB(A)",
        temp_units="celsius", humidity_units="%",
    )
    assert d.co_units == "ppm"
    assert d.pm2p5_units == "ug/m3"
    assert d.noise_units == "dB(A)"
    assert d.temp_units == "celsius"


def test_enhanced_definition_rejects_empty_id():
    with pytest.raises(ValidationError):
        AirQualityEnhancedDefinition(
            air_quality_id="", type_id="1", creation_date=NOW,
        )


def test_enhanced_definition_dumps_camel_case():
    d = AirQualityEnhancedDefinition(
        air_quality_id="aq-001", type_id="1", creation_date=NOW,
        pm2p5_units="ug/m3", bc1_units="ng/m3",
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["airQualityId"] == "aq-001"
    assert dumped["pm2P5Units"] == "ug/m3"
    assert dumped["bc1Units"] == "ng/m3"


def test_threshold_type_enum():
    assert ThresholdType.RISING.value == "rising"
    assert ThresholdType.FALLING.value == "falling"
    assert ThresholdType.RISING_AND_FALLING.value == "risingAndFalling"
    assert len(ThresholdType) == 3


def test_checked_type_enum():
    assert CheckedType.RAW.value == "raw"
    assert CheckedType.VALIDATED.value == "validated"
    assert CheckedType.RATIFIED.value == "ratified"
    assert CheckedType.UNKNOWN.value == "unknown"
    assert len(CheckedType) == 4


def test_data_status_type_enum():
    assert DataStatusType.NORMAL.value == "normal"
    assert DataStatusType.INCOMPLETE.value == "incomplete"
    assert DataStatusType.INCONSISTENT.value == "inconsistent"
    assert DataStatusType.SUSPECT.value == "suspect"
    assert DataStatusType.UNKNOWN.value == "unknown"
    assert len(DataStatusType) == 5
