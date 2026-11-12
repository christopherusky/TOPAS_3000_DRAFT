# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_air_quality-v1.0.xsd (AirQualityDefinitionStructure, AirQualityConfigurationGroup)
# Changes:    Pollutant thresholds flattened directly into configuration; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-08-29
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Air Quality Definition and Configuration.

AirQualityDefinition is the static metadata for an air quality monitor.
AirQualityConfiguration carries pollutant threshold values — flattened (no shared nested group).

Maps to the `airQualityDefinition` AsyncAPI channel.

Field sources: utmc_data_object_air_quality-v1.0.xsd
AirQualityDefinitionStructure, AirQualityConfigurationGroup.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId, ParticipantId, TypeId
from topas_3000_model.common.location import Point
from topas_3000_model.common.time import Timestamp


class AirQualityDefinition(UTMCBaseModel):
    """Static metadata for an air quality monitor."""

    air_quality_id: ObjectId = Field(
        description="Unique identifier of this air quality monitor.",
    )
    type_id: TypeId = Field(
        description="Identifier for the monitor type (reference, indicative, ...).",
    )
    creation_date: Timestamp = Field(
        description="When this definition record was created.",
    )
    location: Point | None = Field(
        default=None,
        description="Physical location as a WGS84 point.",
    )
    short_description: str | None = Field(
        default=None,
        description="Short human-readable name.",
    )
    long_description: str | None = Field(
        default=None,
        description="Longer description.",
    )
    measurement_units: str | None = Field(
        default=None,
        description="Units the monitor reports in (e.g. 'ug/m3', 'ppb').",
    )
    participant_id: ParticipantId | None = Field(
        default=None,
        description="System that owns this definition.",
    )
    owner_operator: str | None = Field(
        default=None,
        description="Name of the monitor's owner or operator.",
    )
    originating_participant_id: ParticipantId | None = Field(
        default=None,
        description="Identifier of the upstream system that originally created this object.",
    )
    originating_object_id: ObjectId | None = Field(
        default=None,
        description="Identifier of this object on the originating system.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


class AirQualityConfiguration(UTMCBaseModel):
    """Pollutant threshold configuration for an air quality monitor.

    Each threshold is optional — only populated for pollutants the
    monitor is configured to track. Fields are flattened.
    """

    air_quality_id: ObjectId = Field(
        description="Identifier of the monitor this configuration applies to.",
    )
    configuration_date: Timestamp = Field(
        description="When this configuration became effective.",
    )
    co_threshold: float | None = Field(
        default=None, ge=0.0, description="Carbon monoxide threshold.",
    )
    no_threshold: float | None = Field(
        default=None, ge=0.0, description="Nitric oxide threshold.",
    )
    nox_threshold: float | None = Field(
        default=None, ge=0.0, description="Nitrogen oxides threshold.",
    )
    no2_threshold: float | None = Field(
        default=None, ge=0.0, description="Nitrogen dioxide threshold.",
    )
    ozone_threshold: float | None = Field(
        default=None, ge=0.0, description="Ozone threshold.",
    )
    pah_threshold: float | None = Field(
        default=None, ge=0.0, description="Polycyclic aromatic hydrocarbons threshold.",
    )
    so2_threshold: float | None = Field(
        default=None, ge=0.0, description="Sulphur dioxide threshold.",
    )
    pm10_threshold: float | None = Field(
        default=None, ge=0.0, description="Particulate matter (PM10) threshold.",
    )
    btx_threshold: float | None = Field(
        default=None, ge=0.0, description="Benzene/toluene/xylene threshold.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )
