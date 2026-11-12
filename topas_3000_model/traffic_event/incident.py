# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_traffic_event_incident-v1.0.xsd (IncidentElementGroup)
# Changes:    78 type enum values preserved; unplanned time fields, incident_time optional as in the XSD; shared consistency checks; string type enum; camelCase JSON output via Pydantic alias_generator; Name/zone/quality references and offset location not carried, one flat external reference
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Incident traffic event (unplanned).

Maps to the `trafficEventIncident` AsyncAPI channel.

An incident is an unplanned occurrence that may affect traffic flow
or needs recording. Lighter than an accident -- no vehicle statistics.
Has 78 type enum values.

Field sources: utmc_traffic_event_incident-v1.0.xsd IncidentElementGroup.
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


class IncidentType(str, Enum):
    """Classification of incident (78 values from the UTMC specification).

    The XSD carries only a catalogue reference, so the value set is not in it.
    """

    ABNORMAL_LOAD = "abnormalLoad"
    AIR_CRASH = "airCrash"
    AIR_RAID = "airRaid"
    ANIMALS_ON_ROAD = "animalsOnRoad"
    ATTACK_ON_VEHICLE = "attackOnVehicle"
    AVALANCHE = "avalanche"
    BLASTING = "blasting"
    BRIDGE_BLOCKED = "bridgeBlocked"
    BRIDGE_CLOSED = "bridgeClosed"
    BRIDGE_DEMOLITION = "bridgeDemolition"
    BRIDGE_MAINTENANCE = "bridgeMaintenance"
    BRIDGE_OPENING = "bridgeOpening"
    BROKEN_DOWN_BUS = "brokenDownBus"
    BROKEN_DOWN_LORRY = "brokenDownLorry"
    BROKEN_DOWN_VEHICLE = "brokenDownVehicle"
    BURST_PIPE = "burstPipe"
    BURST_WATER_MAIN = "burstWaterMain"
    BUS_DISRUPTION = "busDisruption"
    BUS_LANE_BLOCKED = "busLaneBlocked"
    BUS_LANE_CLOSED = "busLaneClosed"
    CENTRE_LANE_BLOCKED = "centreLaneBlocked"
    CENTRE_LANE_CLOSED = "centreLaneClosed"
    CHEMICAL_SPILLAGE = "chemicalSpillage"
    CHILDREN_ON_ROAD = "childrenOnRoad"
    CIVIL_EMERGENCY = "civilEmergency"
    CIVIL_EMERGENCY_CANCELLED = "civilEmergencyCancelled"
    SMOG_ALERT = "smogAlert"
    CONGESTION = "congestion"
    CONTRAFLOW = "contraflow"
    CONVOY = "convoy"
    CROWD = "crowd"
    CYCLISTS = "cyclists"
    DIVERSION = "diversion"
    EMERGENCY_ALERT = "emergencyAlert"
    FACILITIES_CLOSED = "facilitiesClosed"
    FAULT = "fault"
    FIRE = "fire"
    FLOOD = "flood"
    FLOW = "flow"
    FOG = "fog"
    GAWKING_TRAFFIC = "gawkingTraffic"
    GUNFIRE = "gunfire"
    HAIL = "hail"
    HEAVY_FLOW = "heavyFlow"
    ICE = "ice"
    LANE_CLOSURE = "laneClosure"
    LIGHT_FAULT = "lightFault"
    OBSTRUCTION = "obstruction"
    PEOPLE_ON_ROAD = "peopleOnRoad"
    POLLUTION = "pollution"
    POWER_FAILURE = "powerFailure"
    PUBLIC_DISTURBANCE = "publicDisturbance"
    QUEUE = "queue"
    RADIOACTIVE_LEAK = "radioactiveLeak"
    RAIN = "rain"
    RIOT = "riot"
    ROAD_CLOSURE = "roadClosure"
    ROADSIDE_FIRE = "roadsideFire"
    SANDSTORMS = "sandstorms"
    SECURITY_ALERT = "securityAlert"
    SECURITY_INCIDENT = "securityIncident"
    SIGHTSEERS = "sightseers"
    SNOW = "snow"
    SPEED_RESTRICTION = "speedRestriction"
    SPILLAGE = "spillage"
    SPRAY_HAZARD = "sprayHazard"
    STRUCTURAL_DAMAGE = "structuralDamage"
    TELEPHONE_FAULT = "telephoneFault"
    TEMPORARY_SIGNAL = "temporarySignal"
    TERRORIST_INCIDENT = "terroristIncident"
    TORNADOES = "tornadoes"
    TRAFFIC_SIGNAL_FAULT = "trafficSignalFault"
    TRAIN_DISRUPTION = "trainDisruption"
    UNEXPLAINED_DAMAGE = "unexplainedDamage"
    UNEXPLAINED_EVENT = "unexplainedEvent"
    WASHOUT = "washout"
    WIND = "wind"
    OTHER = "other"


class IncidentEvent(TrafficEventChecks):
    """An incident traffic event."""

    # Identity
    event_id: ObjectId = Field(description="Unique identifier of this event.")
    incident_type: IncidentType = Field(description="Classification of the incident.")
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

    # Time (unplanned)
    incident_time: Timestamp | None = Field(
        default=None, description="When the incident was reported. Optional: unknown when first reported.",
    )
    end_time: Timestamp | None = Field(default=None, description="When cleared.")

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
