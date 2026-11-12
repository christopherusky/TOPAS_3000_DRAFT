# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_traffic_event_event-v1.0.xsd (EventElementGroup, PlannedEventElementGroup)
# Changes:    Planned/actual date ranges; shared consistency checks; organiser/venue fields; string type enum; camelCase JSON output via Pydantic alias_generator; Name/zone/quality references and offset location not carried, one flat external reference
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Planned Event traffic event.

Maps to the `trafficEventEvent` AsyncAPI channel.

A planned event is a scheduled occurrence (sports, concerts, markets,
demonstrations) that may affect traffic. Has planned/actual date ranges and organiser/venue fields.

Field sources: utmc_traffic_event_event-v1.0.xsd EventElementGroup,
PlannedEventElementGroup.
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


class PlannedEventType(str, Enum):
    """Classification of planned event (32 values from the UTMC specification).

    The XSD carries only a catalogue reference, so the value set is not in it.
    """

    BALL_GAME = "ballGame"
    BOXING = "boxing"
    ATHLETICS = "athletics"
    CYCLE_RACE = "cycleRace"
    DEMONSTRATION = "demonstration"
    EXHIBITION = "exhibition"
    FAIR = "fair"
    FESTIVAL = "festival"
    FOOTBALL = "football"
    FUNFAIR = "funfair"
    GOLF = "golf"
    INTERNATIONAL_SPORTS = "internationalSports"
    MARATHON = "marathon"
    MARCH = "march"
    MARKET = "market"
    MATCH = "match"
    PARADE = "parade"
    PROCESSION = "procession"
    RACE = "race"
    RUGBY = "rugby"
    SHOW = "show"
    SHOW_JUMPING = "showJumping"
    SPORTS_EVENT = "sportsEvent"
    SPORTS_MEETING = "sportsMeeting"
    STATE_OCCASION = "stateOccasion"
    STRIKE = "strike"
    TENNIS = "tennis"
    TOURNAMENT = "tournament"
    TRADE_FAIR = "tradeFair"
    WATER_SPORTS = "waterSports"
    WINTER_SPORTS = "winterSports"
    OTHER = "other"


class PlannedEvent(TrafficEventChecks):
    """A planned event that may affect traffic."""

    # Identity
    event_id: ObjectId = Field(description="Unique identifier of this event.")
    planned_event_type: PlannedEventType = Field(description="Classification of the event.")
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

    # Event-specific
    organiser: str | None = Field(default=None, description="Event organiser.")
    venue_name: str | None = Field(default=None, description="Venue name.")

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
