# ──────────────────────────────────────────────────────────────────────
# Origin:     UTMC-TS004.0063:2015 (AirQuality_Configuration class)
# Changes:    Multi-level threshold configuration with hysteresis; flat, one message per tier; camelCase JSON output via Pydantic alias_generator
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Air Quality Enhanced Configuration.

Multi-level threshold configuration with hysteresis support.
Each configuration message represents one threshold tier — a monitor
with 3 levels (alert/warning/critical) publishes 3 messages.

Maps to the `airQualityEnhancedConfiguration` AsyncAPI channel.

Field sources: UTMC-TS004.0063:2015 AirQuality_Configuration class.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp
from topas_3000_model.air_quality_enhanced.enums import ThresholdType


class AirQualityEnhancedConfiguration(UTMCBaseModel):
    """Multi-level pollutant threshold configuration."""

    air_quality_id: ObjectId = Field(
        description="Monitor this configuration applies to.",
    )
    configuration_date: Timestamp = Field(
        description="When this configuration became effective.",
    )
    threshold_level: int | None = Field(
        default=None,
        description="Threshold tier (e.g. 1=alert, 2=warning, 3=critical).",
    )
    threshold_type: ThresholdType | None = Field(
        default=None,
        description="Hysteresis type (rising/falling/risingAndFalling).",
    )
    # Original 9 pollutant thresholds
    co_threshold: float | None = Field(default=None, ge=0.0, description="CO threshold.")
    no_threshold: float | None = Field(default=None, ge=0.0, description="NO threshold.")
    nox_threshold: float | None = Field(default=None, ge=0.0, description="NOx threshold.")
    no2_threshold: float | None = Field(default=None, ge=0.0, description="NO2 threshold.")
    ozone_threshold: float | None = Field(default=None, ge=0.0, description="Ozone threshold.")
    pah_threshold: float | None = Field(default=None, ge=0.0, description="PAH threshold.")
    so2_threshold: float | None = Field(default=None, ge=0.0, description="SO2 threshold.")
    pm10_threshold: float | None = Field(default=None, ge=0.0, description="PM10 threshold.")
    btx_threshold: float | None = Field(default=None, ge=0.0, description="BTX threshold.")
    # New pollutant thresholds
    co2_threshold: float | None = Field(default=None, ge=0.0, description="CO2 threshold.")
    pm2p5_threshold: float | None = Field(default=None, ge=0.0, description="PM2.5 threshold.")
    pm1_threshold: float | None = Field(default=None, ge=0.0, description="PM1 threshold.")
    bc1_threshold: float | None = Field(default=None, ge=0.0, description="BC1 threshold.")
    bc2_threshold: float | None = Field(default=None, ge=0.0, description="BC2 threshold.")
    noise_threshold: float | None = Field(default=None, ge=0.0, description="Noise threshold.")
    temp_threshold: float | None = Field(default=None, description="Temperature threshold.")
    rh_threshold: float | None = Field(
        default=None, ge=0.0, le=100.0, description="Relative humidity threshold.",
    )
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")
