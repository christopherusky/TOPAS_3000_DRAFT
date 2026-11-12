# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_car_park-v1.0.xsd (CarParkDefinitionStructure, CarParkConfigurationGroup)
# Changes:    Flattened from XSD inheritance chains (AbstractDeviceStructure -> concrete type); camelCase JSON output via Pydantic alias_generator (XSD used PascalCase element names); extra='forbid' enforces strict validation (XSD allowed extensions via xs:any)
#             2026-04-17 — Added CarParkTariffConfiguration, CarParkOpeningTimesConfiguration,
#                          CarParkAccessLocation, CarParkZoneListEntry and CarParkEntranceType
#                          enum in response to the 2026-04-17 colleague completeness
#                          review.
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-08-29
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Car Park Definition and Configuration.

CarParkDefinition is the static metadata for a physical car park: its
identifier, type, location, human-readable descriptions, and basic
facts (owner/operator, user-type classification, entrance/exit counts,
toilet availability). CarParkConfiguration is the (changeable) capacity
and threshold configuration (total/disabled capacity, AFI/AFD/FI/FD and
entrance-full thresholds). CarParkDetectorsConfiguration is a separate
configuration object that enumerates which detectors feed this car
park's occupancy counts — each element is an ObjectId reference into
the Detector package.

All three map to the `carParkDefinition` AsyncAPI channel (which
carries both definition and configuration payloads — the channel
pattern is `{object}Definition` for both per the spec \u00a71).

Field sources: utmc_data_object_car_park-v1.0.xsd
CarParkDefinitionStructure (plus BaseDefinitionGroup and
DefinitionDescriptionGroup), CarParkBasicDataGroup,
CarParkCapacityConfigurationGroup (which composes
CarParkCapacityThresholdsGroup), CarParkDetectorConfigurationGroup,
CarParkTariffConfigurationStructure,
CarParkOpeningTimesConfigurationStructure,
CarParkAccessLocationStructure and CarParkZoneListEntryStructure.
The XSD's access-location and zone-list groups are now modelled as
standalone sibling classes (`CarParkAccessLocation` and
`CarParkZoneListEntry`) in this same module — they describe a specific
car park even though they hold back-references into the Network
package.
"""

from datetime import time
from enum import Enum
from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.geometry import GeoJsonPoint
from topas_3000_model.common.ids import ObjectId, ParticipantId, TypeId
from topas_3000_model.common.location import Point
from topas_3000_model.common.time import Timestamp


class CarParkDefinition(UTMCBaseModel):
    """Static metadata for a physical car park."""

    car_park_id: ObjectId = Field(
        description="Unique identifier of this car park within its owning system.",
    )
    type_id: TypeId = Field(
        description="Unique identifier for a car park type (multi-storey, surface, park-and-ride, ...).",
    )
    creation_date: Timestamp = Field(
        description="When this definition record was created.",
    )
    location: Point | None = Field(
        default=None,
        description="Physical location of the car park as a WGS84 point. Optional.",
    )
    short_description: str | None = Field(
        default=None,
        description="Short human-readable name (e.g. 'City Centre Multi-Storey').",
    )
    long_description: str | None = Field(
        default=None,
        description="Longer description (context, commissioning notes, etc.).",
    )
    participant_id: ParticipantId | None = Field(
        default=None,
        description="System that owns this car park definition. Often implicit from context.",
    )
    owner_operator: str | None = Field(
        default=None,
        description="Name of the car park's owner or operator.",
    )
    car_park_user_type: str | None = Field(
        default=None,
        description="Public representation of car park type, e.g. 'pay on foot'.",
    )
    number_of_entrances: int | None = Field(
        default=None,
        ge=0,
        description="Number of entrances to the car park.",
    )
    number_of_exits: int | None = Field(
        default=None,
        ge=0,
        description="Number of exits from the car park.",
    )
    toilets_available: bool | None = Field(
        default=None,
        description="Whether toilets are available.",
    )
    disabled_toilets_available: bool | None = Field(
        default=None,
        description="Whether disabled toilets are available.",
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


class CarParkConfiguration(UTMCBaseModel):
    """Capacity and threshold configuration for a car park.

    Maps to CarParkCapacityConfigurationGroup + CarParkCapacityThresholdsGroup
    in the XSD. Thresholds follow the AFI/AFD/FI/FD state-machine:
    the car park is 'almost full' when occupancy rises past AFI and
    recovers below AFD; it is 'full' past FI and recovers below FD.
    """

    car_park_id: ObjectId = Field(
        description="Identifier of the car park this configuration applies to.",
    )
    configuration_date: Timestamp = Field(
        description="When this configuration record became effective.",
    )
    total_capacity: int | None = Field(
        default=None,
        ge=0,
        description="Total number of spaces in the car park (was CarParkCapacity).",
    )
    disabled_capacity: int | None = Field(
        default=None,
        ge=0,
        description="Number of spaces reserved for disabled users.",
    )
    almost_full_increasing_threshold: int | None = Field(
        default=None,
        ge=0,
        description=(
            "Occupancy above which the car park enters the 'almost full' "
            "state (AFI)."
        ),
    )
    almost_full_decreasing_threshold: int | None = Field(
        default=None,
        ge=0,
        description=(
            "Occupancy below which the car park leaves the 'almost full' "
            "state (AFD). Must be less than AFI."
        ),
    )
    full_increasing_threshold: int | None = Field(
        default=None,
        ge=0,
        description=(
            "Occupancy above which the car park enters the 'full' state "
            "(FI). Must be greater than AFI."
        ),
    )
    full_decreasing_threshold: int | None = Field(
        default=None,
        ge=0,
        description=(
            "Occupancy below which the car park returns from 'full' to "
            "'almost full' (FD). Must be less than FI and greater than AFI."
        ),
    )
    entrance_full_threshold: int | None = Field(
        default=None,
        ge=0,
        description=(
            "Occupancy above which the car park's entrance is considered "
            "full."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


class CarParkDetectorsConfiguration(UTMCBaseModel):
    """Configures which detectors feed this car park's occupancy counts.

    Maps to the `Car_Park_Detectors` object in the Stage 1 scope list
    and to CarParkDetectorConfigurationGroup in the XSD (which carries
    a single DetectorRef). The XSD group allows one reference per
    configuration message; we promote to a non-empty list so a single
    message can enumerate the full detector set for a car park.
    """

    car_park_id: ObjectId = Field(
        description="Identifier of the car park this detector configuration applies to.",
    )
    configuration_date: Timestamp = Field(
        description="When this configuration record became effective.",
    )
    detector_ids: list[ObjectId] = Field(
        min_length=1,
        description=(
            "One or more detector identifiers that feed this car park's "
            "occupancy counts. Each entry references a detector in the "
            "Detector package by its ObjectId."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


class CarParkTariffConfiguration(UTMCBaseModel):
    """Per-day-type, per-time-band car park pricing entry."""

    car_park_id: ObjectId = Field(
        description="Identifier of the car park this tariff applies to.",
    )
    day_type_id: ObjectId | None = Field(
        default=None,
        description="Reference to a DayTypeDefinition this tariff applies to.",
    )
    time_period: str | None = Field(
        default=None,
        description=(
            "Operator-specific time-period string (e.g. '08:00-18:00'). "
            "XSD leaves this unconstrained; kept as a raw string."
        ),
    )
    cost: float | None = Field(
        default=None,
        ge=0.0,
        description="Tariff cost for this day-type + time-period band.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


class CarParkOpeningTimesConfiguration(UTMCBaseModel):
    """Per-day-type car park opening / closing time pair."""

    car_park_id: ObjectId = Field(
        description="Identifier of the car park these opening times apply to.",
    )
    day_type_id: ObjectId | None = Field(
        default=None,
        description="Reference to a DayTypeDefinition these times apply to.",
    )
    opening_time: time | None = Field(
        default=None,
        description="Time the car park opens on this day type (local 24-hour).",
    )
    closing_time: time | None = Field(
        default=None,
        description="Time the car park closes on this day type (local 24-hour).",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


class CarParkEntranceType(str, Enum):
    """Direction an access location supports."""

    ENTRY = "entry"
    EXIT = "exit"
    ENTRY_AND_EXIT = "entryAndExit"


class CarParkAccessLocation(UTMCBaseModel):
    """Car park entry/exit point with optional coordinates and network path
    back-reference."""

    car_park_id: ObjectId = Field(
        description="Identifier of the car park this entry/exit belongs to.",
    )
    entry_or_exit: CarParkEntranceType = Field(
        description="Whether this point is an entry, exit, or both.",
    )
    entrance_location: GeoJsonPoint | None = Field(
        default=None,
        description="GeoJSON point for the physical entrance / exit.",
    )
    network_path_id: ObjectId | None = Field(
        default=None,
        description="Back-reference to a NetworkPath the entrance joins.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


class CarParkZoneListEntry(UTMCBaseModel):
    """Membership of a car park in a network zone — for example so a VMS
    can advertise "car parks in Zone X have Y spaces free"."""

    car_park_id: ObjectId = Field(
        description="Identifier of the car park.",
    )
    network_zone_id: ObjectId = Field(
        description="Identifier of the network zone this car park is in.",
    )
    participant_id: ParticipantId | None = Field(
        default=None,
        description="Participant (system) that owns this zone-membership record.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )
