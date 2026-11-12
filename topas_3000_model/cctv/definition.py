# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_cctv-v1.0.xsd (CctvDefinitionStructure, CctvConfigurationGroup)
# Changes:    Flattened from XSD inheritance chains (AbstractDeviceStructure -> concrete type); camelCase JSON output via Pydantic alias_generator (XSD used PascalCase element names); extra='forbid' enforces strict validation (XSD allowed extensions via xs:any)
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-08-29
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""CCTV Definition and Configuration.

Field sources: utmc_data_object_cctv-v1.0.xsd.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId, ParticipantId, TypeId
from topas_3000_model.common.location import Point
from topas_3000_model.common.time import Timestamp


class CctvDefinition(UTMCBaseModel):
    """Static metadata for a CCTV camera."""

    cctv_id: ObjectId = Field(description="Unique identifier of this camera.")
    type_id: TypeId = Field(description="Camera type (colour CCD, B/W CCD, ANPR, ...).")
    creation_date: Timestamp = Field(description="When this definition was created.")
    location: Point | None = Field(default=None, description="Physical location.")
    short_description: str | None = Field(default=None, description="Short name.")
    long_description: str | None = Field(default=None, description="Longer description.")
    participant_id: ParticipantId | None = Field(default=None, description="Owning system.")
    owner_operator: str | None = Field(default=None, description="Owner or operator.")
    originating_participant_id: ParticipantId | None = Field(default=None, description="Upstream system.")
    originating_object_id: ObjectId | None = Field(default=None, description="ID on originating system.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")


class CctvConfiguration(UTMCBaseModel):
    """Operational configuration for a CCTV camera."""

    cctv_id: ObjectId = Field(description="Camera this configuration applies to.")
    configuration_date: Timestamp = Field(description="When effective.")
    serial_number: str | None = Field(default=None, description="Camera serial number.")
    camera_details: str | None = Field(default=None, description="Camera details / model info.")
    fixed: bool | None = Field(default=None, description="Whether the camera is fixed (not PTZ).")
    camera_height_meters: int | None = Field(default=None, ge=0, description="Camera mounting height.")
    camera_sub_site_id: ObjectId | None = Field(default=None, description="Sub-site reference.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")
