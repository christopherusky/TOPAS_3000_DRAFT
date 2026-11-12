# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Enhanced air-quality measurements: original and new pollutant
#              sets, quality metadata, ratified and suspect data states,
#              rejection of negative pollutants and relative humidity over
#              100, acceptance of negative temperature, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.air_quality_enhanced.measurement."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.air_quality_enhanced.enums import CheckedType, DataStatusType
from topas_3000_model.air_quality_enhanced.measurement import AirQualityEnhancedDynamic


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_enhanced_measurement_minimal():
    d = AirQualityEnhancedDynamic(
        air_quality_id="aq-001", timestamp=NOW,
    )
    assert d.co2 is None
    assert d.pm2p5 is None
    assert d.checked_type is None
    assert d.data_status is None


def test_enhanced_measurement_original_pollutants():
    d = AirQualityEnhancedDynamic(
        air_quality_id="aq-001", timestamp=NOW,
        period_seconds=3600, no2=38.5, pm10=22.1, ozone=65.0,
    )
    assert d.no2 == 38.5
    assert d.pm10 == 22.1


def test_enhanced_measurement_new_pollutants():
    d = AirQualityEnhancedDynamic(
        air_quality_id="aq-001", timestamp=NOW,
        co2=412.5, pm2p5=15.3, pm1=8.2,
        bc1=1.5, bc2=1.2, noise=62.0, temp=18.5, rh=65.0,
    )
    assert d.co2 == 412.5
    assert d.pm2p5 == 15.3
    assert d.pm1 == 8.2
    assert d.bc1 == 1.5
    assert d.noise == 62.0
    assert d.temp == 18.5
    assert d.rh == 65.0


def test_enhanced_measurement_with_quality_metadata():
    d = AirQualityEnhancedDynamic(
        air_quality_id="aq-001", timestamp=NOW,
        no2=38.5, accuracy="+-2-3%", smoothed="15-min rolling average",
        derivation="direct measurement",
        checked_type=CheckedType.VALIDATED,
        data_status=DataStatusType.NORMAL,
    )
    assert d.accuracy == "+-2-3%"
    assert d.smoothed == "15-min rolling average"
    assert d.checked_type == CheckedType.VALIDATED
    assert d.data_status == DataStatusType.NORMAL


def test_enhanced_measurement_ratified_data():
    d = AirQualityEnhancedDynamic(
        air_quality_id="aq-001", timestamp=NOW,
        checked_type=CheckedType.RATIFIED,
        data_status=DataStatusType.NORMAL,
    )
    assert d.checked_type == CheckedType.RATIFIED


def test_enhanced_measurement_suspect_data():
    d = AirQualityEnhancedDynamic(
        air_quality_id="aq-001", timestamp=NOW,
        data_status=DataStatusType.SUSPECT,
    )
    assert d.data_status == DataStatusType.SUSPECT


def test_enhanced_measurement_temp_allows_negative():
    d = AirQualityEnhancedDynamic(
        air_quality_id="aq-001", timestamp=NOW, temp=-5.0,
    )
    assert d.temp == -5.0


def test_enhanced_measurement_rejects_negative_pollutant():
    with pytest.raises(ValidationError):
        AirQualityEnhancedDynamic(
            air_quality_id="aq-001", timestamp=NOW, pm2p5=-1.0,
        )


def test_enhanced_measurement_rejects_rh_over_100():
    with pytest.raises(ValidationError):
        AirQualityEnhancedDynamic(
            air_quality_id="aq-001", timestamp=NOW, rh=101.0,
        )


def test_enhanced_measurement_dumps_camel_case():
    d = AirQualityEnhancedDynamic(
        air_quality_id="aq-001", timestamp=NOW,
        co2=412.5, pm2p5=15.3, temp=-2.0, rh=85.0,
        checked_type=CheckedType.RAW,
        data_status=DataStatusType.INCOMPLETE,
        accuracy="+-5%",
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["airQualityId"] == "aq-001"
    assert dumped["co2"] == 412.5
    assert dumped["pm2P5"] == 15.3
    assert dumped["temp"] == -2.0
    assert dumped["rh"] == 85.0
    assert dumped["checkedType"] == "raw"
    assert dumped["dataStatus"] == "incomplete"
    assert dumped["accuracy"] == "+-5%"
