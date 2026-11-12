# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Meteorological dynamic measurements: observations and
#              forecasts, range enforcement on wind speed, wind direction and
#              humidity, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.meteorological.measurement."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.meteorological.enums import (
    PrecipitationType,
    RoadConditionType,
    VisibilityType,
)
from topas_3000_model.meteorological.measurement import MeteorologicalDynamic


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_meteorological_dynamic_minimal():
    d = MeteorologicalDynamic(meteorological_id="met-001", timestamp=NOW, forecast=False)
    assert d.forecast is False
    assert d.road_condition is None
    assert d.air_temperature_celsius is None


def test_meteorological_dynamic_full_observation():
    d = MeteorologicalDynamic(
        meteorological_id="met-001",
        timestamp=NOW,
        forecast=False,
        area_of_effect_km=5.0,
        road_condition=RoadConditionType.WET,
        road_temperature_celsius=8.5,
        visibility_type=VisibilityType.MISTY,
        visibility_distance_meters=500.0,
        precipitation_type=PrecipitationType.DRIZZLE,
        precipitation_intensity=2,
        air_temperature_celsius=10.2,
        min_temperature_celsius=6.0,
        max_temperature_celsius=14.0,
        wind_speed_kph=25.0,
        max_wind_speed_kph=40.0,
        wind_direction_degrees=270.0,
        humidity_percent=85.0,
        pressure_mbar=1013.0,
    )
    assert d.road_condition == RoadConditionType.WET
    assert d.air_temperature_celsius == 10.2
    assert d.wind_direction_degrees == 270.0


def test_meteorological_dynamic_forecast():
    d = MeteorologicalDynamic(
        meteorological_id="met-001", timestamp=NOW, forecast=True,
        air_temperature_celsius=15.0,
    )
    assert d.forecast is True


def test_meteorological_dynamic_rejects_negative_wind_speed():
    with pytest.raises(ValidationError):
        MeteorologicalDynamic(
            meteorological_id="met-001", timestamp=NOW, forecast=False, wind_speed_kph=-1.0,
        )


def test_meteorological_dynamic_rejects_wind_direction_over_360():
    with pytest.raises(ValidationError):
        MeteorologicalDynamic(
            meteorological_id="met-001", timestamp=NOW, forecast=False, wind_direction_degrees=361.0,
        )


def test_meteorological_dynamic_rejects_humidity_over_100():
    with pytest.raises(ValidationError):
        MeteorologicalDynamic(
            meteorological_id="met-001", timestamp=NOW, forecast=False, humidity_percent=101.0,
        )


def test_meteorological_dynamic_dumps_camel_case():
    d = MeteorologicalDynamic(
        meteorological_id="met-001", timestamp=NOW, forecast=False,
        road_condition=RoadConditionType.ICE,
        visibility_type=VisibilityType.FOGGY,
        precipitation_type=PrecipitationType.SNOW,
        air_temperature_celsius=-3.0,
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["meteorologicalId"] == "met-001"
    assert dumped["forecast"] is False
    assert dumped["roadCondition"] == "ice"
    assert dumped["visibilityType"] == "foggy"
    assert dumped["precipitationType"] == "snow"
    assert dumped["airTemperatureCelsius"] == -3.0
