# ──────────────────────────────────────────────────────────────────────
# Origin:     UTMC-TS004.0063:2015 (AirQuality_Dynamic, AirQuality_Dynamic_Quality classes)
# Changes:    All 17 pollutants plus quality metadata flattened; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Air Quality Enhanced Dynamic (measurement).

All 17 pollutants plus quality metadata.

Maps to the `airQualityEnhancedMeasurement` AsyncAPI channel.

Field sources: UTMC-TS004.0063:2015 AirQuality_Dynamic class and
AirQuality_Dynamic_Quality class.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.quality import Quality
from topas_3000_model.common.time import Timestamp
from topas_3000_model.air_quality_enhanced.enums import CheckedType, DataStatusType


class AirQualityEnhancedDynamic(UTMCBaseModel):
    """Live pollutant concentrations with quality metadata."""

    air_quality_id: ObjectId = Field(
        description="Monitor this reading is about.",
    )
    timestamp: Timestamp = Field(description="When measured.")
    quality: Quality | None = Field(default=None, description="Quality statement.")
    period_seconds: int | None = Field(
        default=None, gt=0, description="Collection period in seconds.",
    )
    # Original 9 pollutants
    co: float | None = Field(default=None, ge=0.0, description="Carbon monoxide.")
    no: float | None = Field(default=None, ge=0.0, description="Nitric oxide.")
    nox: float | None = Field(default=None, ge=0.0, description="Nitrogen oxides.")
    no2: float | None = Field(default=None, ge=0.0, description="Nitrogen dioxide.")
    ozone: float | None = Field(default=None, ge=0.0, description="Ozone.")
    pah: float | None = Field(default=None, ge=0.0, description="Polycyclic aromatic hydrocarbons.")
    so2: float | None = Field(default=None, ge=0.0, description="Sulphur dioxide.")
    pm10: float | None = Field(default=None, ge=0.0, description="PM10 particulate.")
    btx: float | None = Field(default=None, ge=0.0, description="Benzene/toluene/xylene.")
    # New 8 pollutants
    co2: float | None = Field(default=None, ge=0.0, description="Carbon dioxide.")
    pm2p5: float | None = Field(default=None, ge=0.0, description="PM2.5 particulate.")
    pm1: float | None = Field(default=None, ge=0.0, description="PM1 particulate.")
    bc1: float | None = Field(default=None, ge=0.0, description="Black carbon (channel 1).")
    bc2: float | None = Field(default=None, ge=0.0, description="Black carbon (channel 2).")
    noise: float | None = Field(default=None, ge=0.0, description="Noise level.")
    temp: float | None = Field(default=None, description="Temperature (can be negative).")
    rh: float | None = Field(default=None, ge=0.0, le=100.0, description="Relative humidity %.")
    # Quality metadata
    accuracy: str | None = Field(
        default=None, description="Accuracy description (e.g. '+-2-3%').",
    )
    smoothed: str | None = Field(
        default=None, description="Smoothing applied (e.g. '15-min rolling average').",
    )
    derivation: str | None = Field(
        default=None, description="How value was derived (e.g. 'estimated from CO2e').",
    )
    checked_type: CheckedType | None = Field(
        default=None, description="Validation level (raw/validated/ratified/unknown).",
    )
    data_status: DataStatusType | None = Field(
        default=None, description="Data quality status.",
    )
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")
