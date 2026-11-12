# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_meteorological-v1.0.xsd (various enumeration types)
# Changes:    String enum values replace XSD integer codes; inline in domain module
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Meteorological domain-specific enumerations.

Inline in this domain module.
"""

from enum import Enum


class RoadConditionType(str, Enum):
    """Road surface condition."""

    DRY = "dry"
    WET = "wet"
    DEEP_WATER = "deepWater"
    FROST = "frost"
    ICE = "ice"
    BLACK_ICE = "blackIce"
    SNOW = "snow"
    SLUSH = "slush"
    FLOODING = "flooding"


class VisibilityType(str, Enum):
    """Visibility condition."""

    SUNNY = "sunny"
    SUNNY_PERIODS = "sunnyPeriods"
    SUNNY_INTERVALS = "sunnyIntervals"
    BRIGHT = "bright"
    CLOUDY = "cloudy"
    MISTY = "misty"
    HAZY = "hazy"
    FOGGY = "foggy"
    OTHER = "other"


class PrecipitationType(str, Enum):
    """Precipitation type."""

    DRY = "dry"
    DRIZZLE = "drizzle"
    RAIN = "rain"
    SHOWERS = "showers"
    SLEET = "sleet"
    SNOW = "snow"
    SLEET_SHOWERS = "sleetShowers"
    SNOW_SHOWERS = "snowShowers"
    THUNDERSTORMS = "thunderstorms"
    HAIL = "hail"
