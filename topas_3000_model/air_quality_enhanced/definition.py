# ──────────────────────────────────────────────────────────────────────
# Origin:     UTMC-TS004.0063:2015 (AirQuality_Basic_Data class)
# Changes:    Per-pollutant unit declarations for all 17 pollutants; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-08-29
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Air Quality Enhanced Definition.

Extends the basic air quality definition with per-pollutant unit
declarations for all 17 pollutants.

Maps to the `airQualityEnhancedDefinition` AsyncAPI channel.

Field sources: UTMC-TS004.0063:2015 AirQuality_Basic_Data class.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId, ParticipantId, TypeId
from topas_3000_model.common.location import Point
from topas_3000_model.common.time import Timestamp


class AirQualityEnhancedDefinition(UTMCBaseModel):
    """Static metadata for an enhanced air quality monitor."""

    air_quality_id: ObjectId = Field(
        description="Unique identifier of this air quality monitor.",
    )
    type_id: TypeId = Field(
        description="Identifier for the monitor type.",
    )
    creation_date: Timestamp = Field(
        description="When this definition record was created.",
    )
    location: Point | None = Field(default=None, description="Physical location.")
    short_description: str | None = Field(default=None, description="Short name.")
    long_description: str | None = Field(default=None, description="Longer description.")
    measurement_units: str | None = Field(
        default=None,
        description="Default units (deprecated -- use per-pollutant unit fields).",
    )
    # Per-pollutant units — original 9
    co_units: str | None = Field(default=None, description="Units for CO data.")
    no_units: str | None = Field(default=None, description="Units for NO data.")
    nox_units: str | None = Field(default=None, description="Units for NOx data.")
    no2_units: str | None = Field(default=None, description="Units for NO2 data.")
    ozone_units: str | None = Field(default=None, description="Units for Ozone data.")
    pah_units: str | None = Field(default=None, description="Units for PAH data.")
    so2_units: str | None = Field(default=None, description="Units for SO2 data.")
    pm10_units: str | None = Field(default=None, description="Units for PM10 data.")
    btx_units: str | None = Field(default=None, description="Units for BTX data.")
    # Per-pollutant units — new 8
    co2_units: str | None = Field(default=None, description="Units for CO2 data.")
    pm2p5_units: str | None = Field(default=None, description="Units for PM2.5 data.")
    pm1_units: str | None = Field(default=None, description="Units for PM1 data.")
    bc1_units: str | None = Field(default=None, description="Units for BC1 (black carbon) data.")
    bc2_units: str | None = Field(default=None, description="Units for BC2 (black carbon) data.")
    noise_units: str | None = Field(default=None, description="Units for noise level data.")
    temp_units: str | None = Field(default=None, description="Units for temperature data.")
    humidity_units: str | None = Field(default=None, description="Units for relative humidity data.")
    # Cross-cutting
    participant_id: ParticipantId | None = Field(default=None, description="Owning system.")
    owner_operator: str | None = Field(default=None, description="Owner or operator.")
    originating_participant_id: ParticipantId | None = Field(default=None, description="Upstream system.")
    originating_object_id: ObjectId | None = Field(default=None, description="ID on originating system.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")
