# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_support_typeID-v1.0.xsd (TypeDefinitionGroup, TypeDefinitionStructure)
# Changes:    Catalogue class (not constrained enum) preserves vendor extensibility; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-05
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Transport Link Type Definitions.

Catalogue entries for transport links. Two kinds exist in the Stage 1
scope, corresponding to the two type-definition arrays the XSD static
block lists for this package (TransportLinkTypeDefinition and
LinkStatusTypeDefinition):

- TransportLinkType — one kind of transport link (SCOOT, ANPR, SCOOT
  Entry, SCOOT Exit, ...). Referenced by TransportLinkDefinition.type_id.
- LinkStatusType    — one operational status of a link. The v1 values are
  0 normal, 1 suspect and 2 noVehiclesInPeriod. Nothing in a live payload
  refers to it: the v1 LinkStatusTypeRef was dropped from the measurement
  variants, so this catalogue is published for completeness only.

Both map to the `transportLinkType` AsyncAPI channel as a single
discriminated union (TransportLinkTypeUnion) keyed on `kind`.

Field sources: utmc_support_typeID-v1.0.xsd TypeDefinitionGroup and
TypeDefinitionStructure, plus TransportLinkTypeDefinition and
LinkStatusTypeDefinition from utmc_data_object_transport_link-v1.0.xsd.
"""

from typing import Annotated, Any, Literal, Union

from pydantic import AnyUrl, Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import TypeId


class TransportLinkType(UTMCBaseModel):
    """Catalogue entry for one kind of transport link."""

    kind: Literal["transportLink"] = Field(
        default="transportLink",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"transportLink" selects the TransportLinkType payload; it is not free-text '
            "command content."
        ),
    )
    type_id: TypeId = Field(description="Identifier of this transport link type.")
    description: str = Field(
        description="Human-readable description of the transport link kind.",
    )
    notes: str | None = Field(
        default=None,
        description="Additional notes, e.g. deployment details.",
    )
    specification_link: AnyUrl | None = Field(
        default=None,
        description="URL to an external specification document for this transport link type.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


class LinkStatusType(UTMCBaseModel):
    """Catalogue entry for one operational status of a transport link."""

    kind: Literal["linkStatus"] = Field(
        default="linkStatus",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"linkStatus" selects the LinkStatusType payload; it is not free-text '
            "command content."
        ),
    )
    type_id: TypeId = Field(description="Identifier of this link status type.")
    description: str = Field(
        description="Human-readable description of the link status (v1: 'normal', 'suspect', 'noVehiclesInPeriod').",
    )
    notes: str | None = Field(
        default=None,
        description="Additional notes about this status value.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


TransportLinkTypeUnion = Annotated[
    Union[TransportLinkType, LinkStatusType],
    Field(discriminator="kind"),
]
"""Canonical payload type for the `transportLinkType` AsyncAPI channel.

A discriminated union over the two type-definition classes, keyed by
the `kind` property. This is the file-level $ref target when emit.py
produces transport_link_type-v1.0.json.
"""
