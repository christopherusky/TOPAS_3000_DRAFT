# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_access_control-v1.0.xsd (AccessControlDefinitionStructure)
# Changes:    Flattened from XSD inheritance chains (AbstractDeviceStructure -> concrete type); camelCase JSON output via Pydantic alias_generator (XSD used PascalCase element names); extra='forbid' enforces strict validation (XSD allowed extensions via xs:any)
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-08-29
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Access Control Definition and Configuration.

AccessControlDefinition is the static metadata for an access control
device (bollard, barrier, gate, sign). AccessControlConfiguration
carries operational parameters — no domain-specific fields are defined
in the XSD beyond the base, so this exists for vendor extensibility
via extensions.

Maps to the `accessControlDefinition` AsyncAPI channel.

Field sources: utmc_data_object_access_control-v1.0.xsd
AccessControlDefinitionStructure.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId, ParticipantId, TypeId
from topas_3000_model.common.location import Point
from topas_3000_model.common.time import Timestamp


class AccessControlDefinition(UTMCBaseModel):
    """Static metadata for an access control device."""

    access_control_id: ObjectId = Field(
        description="Unique identifier of this access control device.",
    )
    type_id: TypeId = Field(
        description="Identifier for the device type (bollard, barrier, gate, sign, ...).",
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
    participant_id: ParticipantId | None = Field(
        default=None,
        description="System that owns this definition.",
    )
    owner_operator: str | None = Field(
        default=None,
        description="Name of the device's owner or operator.",
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


class AccessControlConfiguration(UTMCBaseModel):
    """Operational configuration for an access control device.

    No domain-specific configuration fields are defined in the XSD
    beyond the base; the schema exists for vendor extensibility via
    the extensions field.
    """

    access_control_id: ObjectId = Field(
        description="Identifier of the device this configuration applies to.",
    )
    configuration_date: Timestamp = Field(
        description="When this configuration record became effective.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )
