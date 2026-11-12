# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_meteorological-v1.0.xsd (MeteorologicalDefinitionStructure)
# Changes:    No configuration file; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-08-29
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Meteorological Definition.

No configuration file — met stations have no mutable operational
parameters.

Maps to the `meteorologicalDefinition` AsyncAPI channel.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId, ParticipantId, TypeId
from topas_3000_model.common.location import Point
from topas_3000_model.common.time import Timestamp


class MeteorologicalDefinition(UTMCBaseModel):
    """Static metadata for a meteorological station."""

    meteorological_id: ObjectId = Field(
        description="Unique identifier of this met station.",
    )
    type_id: TypeId = Field(
        description="Identifier for the station type (automated station, ...).",
    )
    creation_date: Timestamp = Field(
        description="When this definition record was created.",
    )
    location: Point | None = Field(default=None, description="Physical location as a WGS84 point.")
    short_description: str | None = Field(default=None, description="Short human-readable name.")
    long_description: str | None = Field(default=None, description="Longer description.")
    participant_id: ParticipantId | None = Field(default=None, description="System that owns this definition.")
    owner_operator: str | None = Field(default=None, description="Owner or operator.")
    originating_participant_id: ParticipantId | None = Field(default=None, description="Upstream system.")
    originating_object_id: ObjectId | None = Field(default=None, description="ID on originating system.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")
