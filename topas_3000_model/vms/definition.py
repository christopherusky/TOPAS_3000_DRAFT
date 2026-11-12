# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_vms-v1.0.xsd (VmsDefinitionGroup, VmsConfigurationGroup)
# Changes:    Flattened from XSD inheritance chains (AbstractDeviceStructure -> concrete type); camelCase JSON output via Pydantic alias_generator (XSD used PascalCase element names); extra='forbid' enforces strict validation (XSD allowed extensions via xs:any); OffsetObjectGroup and PointOnPath link/path references restored; ShortDescription (1 to 32) and LongDescription (1 to 2000) limits restored
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.4
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""VMS Definition and Configuration.

Field sources: utmc_data_object_vms-v1.0.xsd VmsDefinitionGroup,
VmsConfigurationGroup.
"""

from typing import Any

from pydantic import Field, model_validator

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId, ParticipantId, TypeId
from topas_3000_model.common.location import Point
from topas_3000_model.common.network_location import (
    LinkDistanceMetres,
    NetworkLinkRef,
    NetworkPathRef,
    TransportLinkRef,
    check_network_location,
)
from topas_3000_model.common.time import Timestamp
from topas_3000_model.common.text import LongDescription, ShortDescription
from topas_3000_model.vms.enums import VmsColourCapability


class VmsDefinition(UTMCBaseModel):
    """Static metadata for a variable message sign."""

    vms_id: ObjectId = Field(description="Unique identifier of this VMS.")
    type_id: TypeId = Field(description="Unique identifier for a VMS type (sign type catalogue entry).")
    creation_date: Timestamp = Field(description="When this definition was created.")
    location: Point | None = Field(default=None, description="Physical location.")
    transport_link_id: TransportLinkRef | None = Field(
        default=None,
        description="Transport link this sign monitors or sits on. Optional.",
    )
    link_distance_metres: LinkDistanceMetres | None = Field(
        default=None,
        description="Distance along that transport link, in metres. Optional.",
    )
    network_link_id: NetworkLinkRef | None = Field(
        default=None,
        description="Network link the sign stands on. At most one of this and networkPathId. Optional.",
    )
    network_path_id: NetworkPathRef | None = Field(
        default=None,
        description="Network path the sign stands on. At most one of this and networkLinkId. Optional.",
    )
    short_description: ShortDescription | None = Field(default=None, description="Short name.")
    long_description: LongDescription | None = Field(default=None, description="Longer description.")
    sign_height: int | None = Field(default=None, ge=0, description="Sign height in pixels/units.")
    sign_width: int | None = Field(default=None, ge=0, description="Sign width in pixels/units.")
    number_of_lines: int | None = Field(default=None, ge=1, description="Number of text lines.")
    characters_per_line: int | None = Field(default=None, ge=1, description="Characters per line.")
    supports_graphics: bool | None = Field(default=None, description="Whether the sign supports graphics.")
    colour_capability: VmsColourCapability | None = Field(default=None, description="Colour capability.")
    participant_id: ParticipantId | None = Field(default=None, description="Owning system.")
    owner_operator: str | None = Field(default=None, description="Owner or operator.")
    originating_participant_id: ParticipantId | None = Field(default=None, description="Upstream system.")
    originating_object_id: ObjectId | None = Field(default=None, description="ID on originating system.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")

    @model_validator(mode="after")
    def _link_or_path(self) -> "VmsDefinition":
        check_network_location(self.network_link_id, self.network_path_id)
        return self


class VmsConfiguration(UTMCBaseModel):
    """Operational configuration for a VMS."""

    vms_id: ObjectId = Field(description="VMS this configuration applies to.")
    configuration_date: Timestamp = Field(description="When effective.")
    default_brightness_percent: float | None = Field(
        default=None, ge=0.0, le=100.0, description="Default brightness percentage.",
    )
    auto_brightness: bool | None = Field(default=None, description="Whether auto-brightness is enabled.")
    default_display_time_seconds: int | None = Field(
        default=None, gt=0, description="Default message display time in seconds.",
    )
    scroll_speed: int | None = Field(default=None, ge=0, description="Text scroll speed.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")
