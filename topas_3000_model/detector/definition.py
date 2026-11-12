# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_detector-v1.0.xsd (DetectorDefinitionStructure, DetectorConfigurationGroup)
# Changes:    Flattened from XSD inheritance chains (AbstractDeviceStructure -> concrete type); camelCase JSON output via Pydantic alias_generator (XSD used PascalCase element names); extra='forbid' enforces strict validation (XSD allowed extensions via xs:any); ThreshholdUp/Down reduced to one integer each; OffsetObjectGroup and PointOnPath link/path references restored; ShortDescription (1 to 32) and LongDescription (1 to 2000) limits restored
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.3
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Detector Definition and Configuration.

DetectorDefinition is the static metadata for a physical detector: its
identifier, type, location, and human-readable description. A
DetectorConfiguration is the (changeable) operational configuration:
thresholds used for alerting.

Both map to the `detectorDefinition` AsyncAPI channel (which carries both
definition and configuration payloads — the channel pattern is
`{object}Definition` for both per the spec §1).

Field sources: utmc_data_object_detector-v1.0.xsd DetectorDefinitionStructure
and DetectorConfigurationStructure. Freeflow extensions (Colour, ImageLink,
HistoricDate on the definition) are dropped per the spec prohibited-patterns
list.
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


class DetectorDefinition(UTMCBaseModel):
    """Static metadata for a physical detector."""

    detector_id: ObjectId = Field(
        description="Unique identifier of this detector within its owning system.",
    )
    type_id: TypeId = Field(
        description="Unique identifier for a detector type (loop, radar, ANPR, ...).",
    )
    creation_date: Timestamp = Field(
        description="When this definition record was created.",
    )
    location: Point | None = Field(
        default=None,
        description="Physical location of the detector. Optional.",
    )
    transport_link_id: TransportLinkRef | None = Field(
        default=None,
        description="Transport link this detector monitors or sits on. Optional.",
    )
    link_distance_metres: LinkDistanceMetres | None = Field(
        default=None,
        description="Distance along that transport link, in metres. Optional.",
    )
    network_link_id: NetworkLinkRef | None = Field(
        default=None,
        description="Network link the detector stands on. At most one of this and networkPathId. Optional.",
    )
    network_path_id: NetworkPathRef | None = Field(
        default=None,
        description="Network path the detector stands on. At most one of this and networkLinkId. Optional.",
    )
    short_description: ShortDescription | None = Field(
        default=None,
        description="Short human-readable name (e.g. 'A1 southbound').",
    )
    long_description: LongDescription | None = Field(
        default=None,
        description="Longer description (context, installation notes, etc.).",
    )
    participant_id: ParticipantId | None = Field(
        default=None,
        description="System that owns this detector definition. Often implicit from context.",
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

    @model_validator(mode="after")
    def _link_or_path(self) -> "DetectorDefinition":
        check_network_location(self.network_link_id, self.network_path_id)
        return self


class DetectorConfiguration(UTMCBaseModel):
    """Operational configuration for a detector."""

    detector_id: ObjectId = Field(
        description="Identifier of the detector this configuration applies to.",
    )
    configuration_date: Timestamp = Field(
        description="When this configuration record became effective.",
    )
    threshold_up: int | None = Field(
        default=None,
        ge=0,
        description="Upward threshold for alerting (e.g. occupancy threshold).",
    )
    threshold_down: int | None = Field(
        default=None,
        ge=0,
        description="Downward threshold for alerting.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )
