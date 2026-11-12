# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_transport_route-v1.0.xsd
# Changes:    Segment configuration as sibling type; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation; ShortDescription (1 to 32) and LongDescription (1 to 2000) limits restored
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.3
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Transport Route Definition and Configuration.

TransportRouteSegmentConfiguration is a sibling type in the
configuration file, following the car park detectors config
precedent.

Field sources: utmc_data_object_transport_route-v1.0.xsd.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId, ParticipantId, TypeId
from topas_3000_model.common.location import Point
from topas_3000_model.common.time import Timestamp
from topas_3000_model.common.text import LongDescription, ShortDescription


class TransportRouteDefinition(UTMCBaseModel):
    """Static metadata for a transport route."""

    transport_route_id: ObjectId = Field(description="Unique identifier of this route.")
    type_id: TypeId = Field(description="Unique identifier for a transport route type (road, bus, rail, ...).")
    creation_date: Timestamp = Field(description="When this definition was created.")
    location: Point | None = Field(default=None, description="Start point location.")
    end_point: Point | None = Field(default=None, description="Route endpoint location.")
    short_description: ShortDescription | None = Field(default=None, description="Short name.")
    long_description: LongDescription | None = Field(default=None, description="Longer description.")
    participant_id: ParticipantId | None = Field(default=None, description="Owning system.")
    owner_operator: str | None = Field(default=None, description="Owner or operator.")
    originating_participant_id: ParticipantId | None = Field(default=None, description="Upstream system.")
    originating_object_id: ObjectId | None = Field(default=None, description="ID on originating system.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")


class TransportRouteConfiguration(UTMCBaseModel):
    """Operational configuration for a transport route."""

    transport_route_id: ObjectId = Field(description="Route this configuration applies to.")
    configuration_date: Timestamp = Field(description="When this configuration became effective.")
    route_name: str | None = Field(default=None, description="Human-readable route name.")
    route_start: str | None = Field(default=None, description="Route start description.")
    route_via: str | None = Field(default=None, description="Via point description.")
    route_end: str | None = Field(default=None, description="Route end description.")
    route_length_meters: float | None = Field(default=None, ge=0.0, description="Route length in meters.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")


class TransportRouteSegmentConfiguration(UTMCBaseModel):
    """Segment configuration — links a route segment to a transport link.

    Sibling type in the configuration file.
    """

    transport_route_id: ObjectId = Field(description="Route this segment belongs to.")
    configuration_date: Timestamp = Field(description="When effective.")
    route_segment_id: int = Field(ge=0, description="Ordered segment identifier within the route.")
    transport_link_id: ObjectId = Field(description="Cross-domain reference to a transport link.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")
