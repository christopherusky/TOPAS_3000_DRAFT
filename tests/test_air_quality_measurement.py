# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Air-quality dynamic measurements: pollutant readings, rejection
#              of negative concentration and zero period, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.air_quality.measurement."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.air_quality.measurement import AirQualityDynamic


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_air_quality_dynamic_minimal():
    d = AirQualityDynamic(air_quality_id="aq-001", timestamp=NOW)
    assert d.co is None
    assert d.pm10 is None


def test_air_quality_dynamic_with_readings():
    d = AirQualityDynamic(
        air_quality_id="aq-001",
        timestamp=NOW,
        period_seconds=3600,
        no2=38.5,
        pm10=22.1,
        ozone=65.0,
        co=0.3,
    )
    assert d.no2 == 38.5
    assert d.pm10 == 22.1
    assert d.period_seconds == 3600


def test_air_quality_dynamic_rejects_negative_concentration():
    with pytest.raises(ValidationError):
        AirQualityDynamic(air_quality_id="aq-001", timestamp=NOW, no2=-1.0)


def test_air_quality_dynamic_rejects_zero_period():
    with pytest.raises(ValidationError):
        AirQualityDynamic(
            air_quality_id="aq-001", timestamp=NOW, period_seconds=0,
        )


def test_air_quality_dynamic_dumps_camel_case():
    d = AirQualityDynamic(
        air_quality_id="aq-001",
        timestamp=NOW,
        no2=38.5,
        pm10=22.1,
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["airQualityId"] == "aq-001"
    assert dumped["no2"] == 38.5
    assert dumped["pm10"] == 22.1
