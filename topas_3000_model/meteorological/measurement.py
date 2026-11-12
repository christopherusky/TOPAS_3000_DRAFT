# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_meteorological-v1.0.xsd (MeteorologicalDynamicGroup plus sub-groups)
# Changes:    Single flat type with optional fields for missing sensors; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Meteorological dynamic (live state) payload.

Single flat type — weather stations report combined observations.

Maps to the `meteorologicalMeasurement` AsyncAPI channel.

Field sources: utmc_data_object_meteorological-v1.0.xsd
MeteorologicalDynamicGroup plus sub-groups.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.quality import Quality
from topas_3000_model.common.time import Timestamp
from topas_3000_model.meteorological.enums import (
    PrecipitationType,
    RoadConditionType,
    VisibilityType,
)


class MeteorologicalDynamic(UTMCBaseModel):
    """Live weather observation from a meteorological station."""

    meteorological_id: ObjectId = Field(description="Identifier of the met station.")
    timestamp: Timestamp = Field(description="Time the observation was taken.")
    quality: Quality | None = Field(default=None, description="Optional quality statement.")
    forecast: bool = Field(description="False = actual observation, True = forecast.")
    area_of_effect_km: float | None = Field(
        default=None, ge=0.0, description="Radius of the area this observation covers, in km.",
    )
    # Road
    road_condition: RoadConditionType | None = Field(default=None, description="Road surface condition.")
    road_temperature_celsius: float | None = Field(default=None, description="Road surface temperature.")
    # Visibility
    visibility_type: VisibilityType | None = Field(default=None, description="Visibility condition.")
    visibility_distance_meters: float | None = Field(default=None, ge=0.0, description="Visibility distance.")
    # Precipitation
    precipitation_type: PrecipitationType | None = Field(default=None, description="Precipitation type.")
    precipitation_intensity: int | None = Field(default=None, ge=0, description="Precipitation intensity.")
    # Temperature
    air_temperature_celsius: float | None = Field(default=None, description="Air temperature.")
    min_temperature_celsius: float | None = Field(default=None, description="Minimum temperature.")
    max_temperature_celsius: float | None = Field(default=None, description="Maximum temperature.")
    # Wind
    wind_speed_kph: float | None = Field(default=None, ge=0.0, description="Wind speed in km/h.")
    max_wind_speed_kph: float | None = Field(default=None, ge=0.0, description="Maximum wind speed gust.")
    wind_direction_degrees: float | None = Field(
        default=None, ge=0.0, le=360.0, description="Wind direction in degrees from north.",
    )
    # Other
    humidity_percent: float | None = Field(default=None, ge=0.0, le=100.0, description="Relative humidity.")
    pressure_mbar: float | None = Field(default=None, ge=0.0, description="Atmospheric pressure in mbar.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")
