# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Enhanced air-quality configuration: the full threshold set
#              including the new pollutants, level and type fields, rejection
#              of negative thresholds and relative humidity over 100,
#              acceptance of negative temperature, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.air_quality_enhanced.configuration."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.air_quality_enhanced.configuration import AirQualityEnhancedConfiguration
from topas_3000_model.air_quality_enhanced.enums import ThresholdType


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_enhanced_configuration_minimal():
    c = AirQualityEnhancedConfiguration(
        air_quality_id="aq-001", configuration_date=NOW,
    )
    assert c.threshold_level is None
    assert c.threshold_type is None
    assert c.co2_threshold is None


def test_enhanced_configuration_with_level_and_type():
    c = AirQualityEnhancedConfiguration(
        air_quality_id="aq-001", configuration_date=NOW,
        threshold_level=1, threshold_type=ThresholdType.RISING,
        no2_threshold=40.0, pm10_threshold=50.0, pm2p5_threshold=25.0,
    )
    assert c.threshold_level == 1
    assert c.threshold_type == ThresholdType.RISING
    assert c.pm2p5_threshold == 25.0


def test_enhanced_configuration_all_new_thresholds():
    c = AirQualityEnhancedConfiguration(
        air_quality_id="aq-001", configuration_date=NOW,
        threshold_level=2, threshold_type=ThresholdType.RISING_AND_FALLING,
        co2_threshold=1000.0, pm2p5_threshold=50.0, pm1_threshold=25.0,
        bc1_threshold=5.0, bc2_threshold=5.0, noise_threshold=70.0,
        temp_threshold=35.0, rh_threshold=95.0,
    )
    assert c.co2_threshold == 1000.0
    assert c.noise_threshold == 70.0
    assert c.rh_threshold == 95.0


def test_enhanced_configuration_rejects_negative_threshold():
    with pytest.raises(ValidationError):
        AirQualityEnhancedConfiguration(
            air_quality_id="aq-001", configuration_date=NOW,
            pm2p5_threshold=-1.0,
        )


def test_enhanced_configuration_rejects_rh_over_100():
    with pytest.raises(ValidationError):
        AirQualityEnhancedConfiguration(
            air_quality_id="aq-001", configuration_date=NOW,
            rh_threshold=101.0,
        )


def test_enhanced_configuration_temp_allows_negative():
    c = AirQualityEnhancedConfiguration(
        air_quality_id="aq-001", configuration_date=NOW,
        temp_threshold=-10.0,
    )
    assert c.temp_threshold == -10.0


def test_enhanced_configuration_dumps_camel_case():
    c = AirQualityEnhancedConfiguration(
        air_quality_id="aq-001", configuration_date=NOW,
        threshold_level=1, threshold_type=ThresholdType.FALLING,
        pm2p5_threshold=25.0, co2_threshold=800.0,
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["airQualityId"] == "aq-001"
    assert dumped["thresholdLevel"] == 1
    assert dumped["thresholdType"] == "falling"
    assert dumped["pm2P5Threshold"] == 25.0
    assert dumped["co2Threshold"] == 800.0
