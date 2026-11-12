# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_traffic_event_accident-v1.0.xsd (AccidentElementGroup, AccidentStatisticsGroup)
# Changes:    Flattened accident statistics; unplanned time fields, accident_time optional as in the XSD; shared consistency checks; string type enum; camelCase JSON output via Pydantic alias_generator; accident enum differs from the XSD, StatisticDate as a timestamp, Name/zone/quality references and offset location not carried, one flat external reference
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Accident traffic event (unplanned).

Maps to the `trafficEventAccident` AsyncAPI channel.

An accident is an unplanned occurrence involving vehicles that has a
direct effect on traffic flow. Has flattened accident statistics
and unplanned time fields: accident_time + end_time.

Field sources: utmc_traffic_event_accident-v1.0.xsd
AccidentElementGroup, AccidentStatisticsGroup.
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


class AccidentType(str, Enum):
    """Classification of accident (20 values).

    18 of the 20 named XSD values, plus ``other``, plus ``overturnedTrailer``,
    which has no XSD code. XSD codes 5 ("Accident History") and 20 ("Vehicle
    spun around") are not carried.
    """

    ACCIDENT = "accident"
    BUS_ACCIDENT = "busAccident"
    COLLISION = "collision"
    INVESTIGATION_WORK = "investigationWork"
    CHEMICAL_SPILLAGE = "chemicalSpillage"
    FUEL_SPILLAGE = "fuelSpillage"
    HAZARDOUS_MATERIALS = "hazardousMaterials"
    HEAVY_LORRY = "heavyLorry"
    JACK_KNIFED_ARTICULATED_LORRY = "jackKnifedArticulatedLorry"
    JACK_KNIFED_CARAVAN = "jackKnifedCaravan"
    JACK_KNIFED_TRAILER = "jackKnifedTrailer"
    MULTI_VEHICLE = "multiVehicle"
    OIL_SPILLAGE = "oilSpillage"
    OVERTURNED_HEAVY_LORRY = "overturnedHeavyLorry"
    OVERTURNED_TRAILER = "overturnedTrailer"
    OVERTURNED_VEHICLE = "overturnedVehicle"
    SECONDARY_ACCIDENT = "secondaryAccident"
    SERIOUS_ACCIDENT = "seriousAccident"
    SHED_LOAD = "shedLoad"
    OTHER = "other"


class AccidentEvent(TrafficEventChecks):
    """An accident traffic event."""

    # Identity
    event_id: ObjectId = Field(description="Unique identifier of this event.")
    accident_type: AccidentType = Field(description="Classification of the accident.")
    status: EventStatus = Field(description="Event lifecycle state.")

    # Public
    public_description: str | None = Field(
        default=None, description="Public-facing summary for journey planners.",
    )

    # Location
    location_description: str | None = Field(
        default=None, description="Free-text location description.",
    )
    location: Point | None = Field(
        default=None, description="WGS84 point location.",
    )

    # Severity & impact
    severity: SeverityType | None = Field(default=None, description="Event severity.")
    phase: str | None = Field(default=None, description="Event phase (unconstrained).")
    lanes_affected: LanesAffectedType | None = Field(
        default=None, description="Lane closure pattern (I=open, T=closed).",
    )
    lanes_restricted: int | None = Field(
        default=None, ge=0, description="Count of closed/restricted lanes.",
    )
    lanes_operational: int | None = Field(
        default=None, ge=0, description="Count of operational lanes.",
    )
    diversion_route: str | None = Field(default=None, description="Diversion route description.")
    diversion_in_force: bool | None = Field(default=None, description="Whether diversion is active.")

    # Reporting
    reported_by: str | None = Field(default=None, description="Who reported the event.")
    confirmed_by: str | None = Field(default=None, description="Who confirmed.")
    modified_by: str | None = Field(default=None, description="Who last modified.")

    # Time (unplanned)
    accident_time: Timestamp | None = Field(
        default=None, description="When the accident was reported. Optional: unknown when first reported.",
    )
    end_time: Timestamp | None = Field(default=None, description="When cleared.")

    # Flattened statistics
    vehicle_count: int | None = Field(
        default=None, ge=0, description="Number of vehicles involved.",
    )
    light_conditions: str | None = Field(default=None, description="Light conditions at time.")
    road_conditions: str | None = Field(default=None, description="Road surface conditions.")
    weather_conditions: str | None = Field(default=None, description="Weather conditions.")
    statistic_date: Timestamp | None = Field(
        default=None, description="When statistics were collected.",
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
