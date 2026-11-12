# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_air_quality-v1.0.xsd (AirQualityDynamicGroup, AirQualityMonitorGroup)
# Changes:    Pollutant concentrations flattened directly into measurement; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Air Quality dynamic (live state) payload.

Maps to the `airQualityMeasurement` AsyncAPI channel. Pollutant
concentrations are flattened.

Field sources: utmc_data_object_air_quality-v1.0.xsd
AirQualityDynamicGroup, AirQualityMonitorGroup.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.quality import Quality
from topas_3000_model.common.time import Timestamp


class AirQualityDynamic(UTMCBaseModel):
    """Live pollutant concentration data from an air quality monitor."""

    air_quality_id: ObjectId = Field(
        description="Identifier of the monitor this reading is about.",
    )
    timestamp: Timestamp = Field(
        description="Time the measurement was taken.",
    )
    quality: Quality | None = Field(
        default=None,
        description="Optional quality statement.",
    )
    period_seconds: int | None = Field(
        default=None,
        gt=0,
        description="Data collection period in seconds.",
    )
    co: float | None = Field(
        default=None, ge=0.0, description="Carbon monoxide concentration.",
    )
    no: float | None = Field(
        default=None, ge=0.0, description="Nitric oxide concentration.",
    )
    nox: float | None = Field(
        default=None, ge=0.0, description="Nitrogen oxides concentration.",
    )
    no2: float | None = Field(
        default=None, ge=0.0, description="Nitrogen dioxide concentration.",
    )
    ozone: float | None = Field(
        default=None, ge=0.0, description="Ozone concentration.",
    )
    pah: float | None = Field(
        default=None, ge=0.0, description="Polycyclic aromatic hydrocarbons concentration.",
    )
    so2: float | None = Field(
        default=None, ge=0.0, description="Sulphur dioxide concentration.",
    )
    pm10: float | None = Field(
        default=None, ge=0.0, description="Particulate matter (PM10) concentration.",
    )
    btx: float | None = Field(
        default=None, ge=0.0, description="Benzene/toluene/xylene concentration.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )
