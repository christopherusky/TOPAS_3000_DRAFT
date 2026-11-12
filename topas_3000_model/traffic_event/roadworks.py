# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_traffic_event_roadworks-v1.0.xsd (RoadWorksElementGroup, PlannedEventElementGroup)
# Changes:    Planned/actual date ranges; shared consistency checks; contractor and contraflow flags; string type enum; camelCase JSON output via Pydantic alias_generator; Name/zone/quality references and offset location not carried, ConfirmedDate as a timestamp, one flat external reference
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Roadworks traffic event (planned).

Maps to the `trafficEventRoadworks` AsyncAPI channel.

Roadworks are planned works on the road network. Has planned/actual
date ranges, contractor, traffic signals, and contraflow flags.

Field sources: utmc_traffic_event_roadworks-v1.0.xsd
RoadWorksElementGroup, PlannedEventElementGroup.
"""

from enum import Enum
from typing import Any

from pydantic import Field

from topas_3000_model.traffic_event.checks import TrafficEventChecks
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.location import Point
from topas_3000_model.common.time import Timestamp
from topas_3000_model.traffic_event.common import (
    EventStatus,
    LanesAffectedType,
    SeverityType,
)


class RoadworksType(str, Enum):
    """Classification of roadworks (7 values from XSD)."""

    CENTRAL_RESERVATION = "centralReservation"
    GENERAL = "general"
    OVERHEAD = "overhead"
    SURFACE = "surface"
    VERGE = "verge"
    WATER_MAIN_WORK = "waterMainWork"
    OTHER = "other"


class RoadworksEvent(TrafficEventChecks):
    """A roadworks traffic event."""

    # Identity
    event_id: ObjectId = Field(description="Unique identifier of this event.")
    roadworks_type: RoadworksType = Field(description="Classification of the roadworks.")
    status: EventStatus = Field(description="Event lifecycle state.")

    # Public
    public_description: str | None = Field(
        default=None, description="Public-facing summary for journey planners.",
    )

    # Location
    location_description: str | None = Field(default=None, description="Free-text location description.")
    location: Point | None = Field(default=None, description="WGS84 point location.")

    # Severity & impact
    severity: SeverityType | None = Field(default=None, description="Event severity.")
    phase: str | None = Field(default=None, description="Event phase (unconstrained).")
    lanes_affected: LanesAffectedType | None = Field(
        default=None, description="Lane closure pattern.",
    )
    lanes_restricted: int | None = Field(default=None, ge=0, description="Count of closed/restricted lanes.")
    lanes_operational: int | None = Field(default=None, ge=0, description="Count of operational lanes.")
    diversion_route: str | None = Field(default=None, description="Diversion route description.")
    diversion_in_force: bool | None = Field(default=None, description="Whether diversion is active.")

    # Reporting
    reported_by: str | None = Field(default=None, description="Who reported the event.")
    confirmed_by: str | None = Field(default=None, description="Who confirmed.")
    modified_by: str | None = Field(default=None, description="Who last modified.")

    # Time (planned)
    planned_start: Timestamp | None = Field(default=None, description="Planned start time.")
    planned_end: Timestamp | None = Field(default=None, description="Planned end time.")
    actual_start: Timestamp | None = Field(default=None, description="Actual start time.")
    actual_end: Timestamp | None = Field(default=None, description="Actual end time.")

    # Roadworks-specific
    contractor: str | None = Field(default=None, description="Contractor name.")
    traffic_signals: bool | None = Field(
        default=None, description="Whether temporary traffic signals are in use.",
    )
    contraflow: bool | None = Field(
        default=None, description="Whether contraflow is in operation.",
    )
    confirmed_date: Timestamp | None = Field(
        default=None, description="When the roadworks were confirmed.",
    )

    # External cross-reference
    external_country_code: str | None = Field(
        default=None,
        pattern=r"^[A-Za-z0-9._:-]+$",
        description="Cross-ref country code (an XML NMTOKEN: no whitespace).",
    )
    external_system_code: str | None = Field(default=None, description="Cross-ref system code.")
    external_situation_id: str | None = Field(default=None, description="Cross-ref situation identifier.")
    external_situation_version: int | None = Field(
        default=None,
        ge=0,
        description="Cross-ref situation version. Requires external_situation_id.",
    )

    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")
