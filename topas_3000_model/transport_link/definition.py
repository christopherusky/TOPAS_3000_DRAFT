# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_transport_link-v1.0.xsd (TransportLinkDefinitionStructure, TransportLinkSCOOTConfigurationGroup)
# Changes:    Flattened from XSD inheritance chains (AbstractDeviceStructure -> concrete type); camelCase JSON output via Pydantic alias_generator (XSD used PascalCase element names); extra='forbid' enforces strict validation (XSD allowed extensions via xs:any); TransportLinkSpeedTimeGroup (at capacity, freeflow, average) not carried; EndPoint restored as end_point and location is the link start; ShortDescription (1 to 32) and LongDescription (1 to 2000) limits restored
#             2026-04-17 — Split TransportLinkSCOOTConfiguration into a discriminated union (SCOOT/GPS/ANPR) in response to the colleague completeness review.
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.3
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Transport Link Definition and SCOOT Configuration.

TransportLinkDefinition is the static metadata for a transport link (typically a
SCOOT or ANPR-monitored section of road): its identifier, type, start and end
points and human-readable description. TransportLinkSCOOTConfiguration
is the (changeable) operational configuration covering physical link
parameters (length, lane count, capacity) used when the link is
modelled by SCOOT.

Both map to the `transportLinkDefinition` AsyncAPI channel (which
carries both definition and configuration payloads — the channel
pattern is `{object}Definition` for both per the spec \u00a71).

Field sources: utmc_data_object_transport_link-v1.0.xsd
TransportLinkDefinitionStructure, TransportLinkConfigurationGroup and
TransportLinkScootConfiguration. Freeflow extensions on the definition
(Colour, ImageLink, HistoricDate) are dropped per the spec prohibited-patterns
list. The link keeps both of its v1 points: ``location`` is the start of the
link (PointOnPathGroup ``Point``) and ``end_point`` its end (``EndPoint``,
LinkLocationGroup); the full geometry belongs to the Network package.
"""

from typing import Annotated, Any, Literal, Union

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId, ParticipantId, TypeId
from topas_3000_model.common.location import Point
from topas_3000_model.common.time import Timestamp
from topas_3000_model.common.text import LongDescription, ShortDescription


class TransportLinkDefinition(UTMCBaseModel):
    """Static metadata for a transport link."""

    transport_link_id: ObjectId = Field(
        description="Unique identifier of this transport link within its owning system.",
    )
    type_id: TypeId = Field(
        description="Unique identifier for a transport link type (SCOOT, ANPR, SCOOT Entry, ...).",
    )
    creation_date: Timestamp = Field(
        description="When this definition record was created.",
    )
    location: Point | None = Field(
        default=None,
        description="Start of the transport link as a WGS84 point. Optional.",
    )
    end_point: Point | None = Field(
        default=None,
        description=(
            "End of the transport link as a WGS84 point. Optional here although the v1 XSD "
            "requires it, matching transport_route."
        ),
    )
    short_description: ShortDescription | None = Field(
        default=None,
        description="Short human-readable name (e.g. 'A1 northbound, Jct 5 -> Jct 6').",
    )
    long_description: LongDescription | None = Field(
        default=None,
        description="Longer description (context, commissioning notes, etc.).",
    )
    participant_id: ParticipantId | None = Field(
        default=None,
        description="System that owns this transport link definition. Often implicit from context.",
    )
    originating_participant_id: ParticipantId | None = Field(
        default=None,
        description=(
            "Identifier of the upstream system that originally created this object "
            "(comparable to the UML DataSourceTypeId)."
        ),
    )
    originating_object_id: ObjectId | None = Field(
        default=None,
        description=(
            "Identifier of this object on the originating (upstream) system. "
            "Enables cross-system traceability."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


class _TransportLinkConfigurationBase(UTMCBaseModel):
    """Fields shared by every transport link configuration variant.

    Corresponds to the XSD TransportLinkConfigurationGroup, reused
    unchanged across TransportLinkScoot/Gps/AnprConfiguration elements
    in utmc_data_object_transport_link-v1.0.xsd.
    """

    transport_link_id: ObjectId = Field(
        description="Identifier of the transport link this configuration applies to.",
    )
    configuration_date: Timestamp = Field(
        description="When this configuration record became effective.",
    )
    length_metres: float | None = Field(
        default=None,
        ge=0.0,
        description="Distance from detector to stop line in metres.",
    )
    full_length_metres: float | None = Field(
        default=None,
        ge=0.0,
        description="Distance from stop line to stop line in metres.",
    )
    number_of_lanes: int | None = Field(
        default=None,
        ge=1,
        description="Number of lanes on the link.",
    )
    maximum_speed_kph: float | None = Field(
        default=None,
        ge=0.0,
        description="Maximum speed associated with the link in kilometres per hour.",
    )
    capacity_vehicles_per_hour: float | None = Field(
        default=None,
        ge=0.0,
        description="Maximum flow in vehicles per hour.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


class TransportLinkSCOOTConfiguration(_TransportLinkConfigurationBase):
    """Operational configuration for a SCOOT-modelled transport link."""

    kind: Literal["scoot"] = Field(
        default="scoot",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"scoot" selects the TransportLinkSCOOTConfiguration payload; it is not free-text '
            "command content."
        ),
    )


class TransportLinkGPSConfiguration(_TransportLinkConfigurationBase):
    """Operational configuration for a GPS-monitored transport link."""

    kind: Literal["gps"] = Field(
        default="gps",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"gps" selects the TransportLinkGPSConfiguration payload; it is not free-text '
            "command content."
        ),
    )


class TransportLinkANPRConfiguration(_TransportLinkConfigurationBase):
    """Operational configuration for an ANPR-monitored transport link."""

    kind: Literal["anpr"] = Field(
        default="anpr",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"anpr" selects the TransportLinkANPRConfiguration payload; it is not free-text '
            "command content."
        ),
    )


TransportLinkConfiguration = Annotated[
    Union[
        TransportLinkSCOOTConfiguration,
        TransportLinkGPSConfiguration,
        TransportLinkANPRConfiguration,
    ],
    Field(discriminator="kind"),
]
"""Canonical payload type for the `transportLinkConfiguration` AsyncAPI channel.

A discriminated union over the three monitoring-system variants (SCOOT,
GPS, ANPR), keyed by the `kind` property. Variants are structurally
identical except for the literal — the XSD encoded the monitoring-system
identity via element name; we encode it via discriminator.
"""
