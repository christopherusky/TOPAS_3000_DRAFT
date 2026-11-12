# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_transport_link-v1.0.xsd (TransportLinkDynamicGroup and its SCOOT, ANPR and GPS sub-groups)
# Changes:    Three-variant discriminated union (SCOOT/ANPR/GPS) with 'kind' field replaces XSD substitution groups; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#             2026-04-17 — Added TransportLinkGPSDynamic as a third variant of the measurement union in response to the colleague completeness review.
#             2026-10-06 — currentFlowVehicles and reservedLaneFlowVehicles are non-negative finite numbers, as v1's VehiclesPerInterval (xsd:decimal), not integers.
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.4
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Transport Link measurement payloads.

Maps to the `transportLinkMeasurement` AsyncAPI channel. A measurement
payload is one of three variants keyed by the `kind` discriminator: SCOOT
(congestion / flow / occupancy from the SCOOT controller), ANPR
(matched number-plate counts from camera pairs) or GPS (journey-time
statistics and reserved-lane flow from GPS probes). All variants share a
common base of transport link id, timestamp, optional quality, average
speed and link travel time — these live in the private _TransportLinkDynamicBase
class.

Field sources: utmc_quantised_transport_link-v1.0.xsd
TransportLinkDynamicGroup (AverageSpeed, LinkTravelTime),
TransportLinkScootDynamicGroup (CongestionPercent, CurrentFlow,
OccupancyPercent) and TransportLinkAnprDynamicGroup (NumberPlatesIn,
NumberPlatesOut, NumberPlatesMatches, NumberPlatesMatchesUsed). The XSD
original wraps these under Prediction/Profile structures; we flatten to
discrete measurement variants suitable as AMQP message payloads.

LinkTravelTime is an XSD duration in the original; we normalise to
seconds-as-float for consistency with the rest of the clean model.
LinkStatusTypeRef (in TransportLinkDynamicGroup) is dropped: no
measurement says what state its link is in. The LinkStatusType catalogue is
still published on the transportLinkType channel, but nothing refers to it.
"""

from typing import Annotated, Any, Literal, Union

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.quality import Quality
from topas_3000_model.common.time import Timestamp


class _TransportLinkDynamicBase(UTMCBaseModel):
    """Fields shared by every transport link measurement variant."""

    transport_link_id: ObjectId = Field(
        description="Identifier of the transport link this reading is about.",
    )
    timestamp: Timestamp = Field(
        description="Time the measurement was taken.",
    )
    quality: Quality | None = Field(
        default=None,
        description="Optional quality statement for this measurement.",
    )
    average_speed_kph: float | None = Field(
        default=None,
        ge=0.0,
        description="Estimated average speed in kilometres per hour.",
    )
    link_travel_time_seconds: float | None = Field(
        default=None,
        ge=0.0,
        description="Current link journey time in seconds.",
    )


class TransportLinkSCOOTDynamic(_TransportLinkDynamicBase):
    """SCOOT-reported live measurement for a transport link."""

    kind: Literal["scoot"] = Field(
        default="scoot",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"scoot" selects the TransportLinkSCOOTDynamic payload; it is not free-text '
            "command content."
        ),
    )
    congestion_percent: float | None = Field(
        default=None,
        ge=0.0,
        le=100.0,
        description="Current percentage congestion as reported by SCOOT.",
    )
    # XSD: CurrentFlow (VehiclesPerInterval, xsd:decimal). A rate per interval,
    # so it may be fractional; narrowed to an integer until this was relaxed.
    current_flow_vehicles: float | None = Field(
        default=None,
        ge=0.0,
        allow_inf_nan=False,
        description=(
            "Current flow in vehicles over the SCOOT measurement interval. A "
            "rate, so it may be fractional (e.g. a smoothed or averaged flow)."
        ),
    )
    occupancy_percent: float | None = Field(
        default=None,
        ge=0.0,
        le=100.0,
        description="Average link occupancy percentage across all detectors on the link.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


class TransportLinkANPRDynamic(_TransportLinkDynamicBase):
    """ANPR-reported live measurement for a transport link."""

    kind: Literal["anpr"] = Field(
        default="anpr",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"anpr" selects the TransportLinkANPRDynamic payload; it is not free-text '
            "command content."
        ),
    )
    number_plates_in: int | None = Field(
        default=None,
        ge=0,
        description="Number of successful plate reads at the start of the link.",
    )
    number_plates_out: int | None = Field(
        default=None,
        ge=0,
        description="Number of successful plate reads at the end of the link.",
    )
    number_plates_matches: int | None = Field(
        default=None,
        ge=0,
        description="Count of number-plate matches in the measurement period.",
    )
    number_plates_matches_used: int | None = Field(
        default=None,
        ge=0,
        description="Number of successful matches actually used in the travel-time calculation.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


class TransportLinkGPSDynamic(_TransportLinkDynamicBase):
    """GPS-probe live measurement for a transport link.

    Field set drawn from the XSD TransportLinkGpsDynamicGroup (Freeflow
    Extension): CurrentJourneyTime, the three travel-time statistics
    (Mean/Median/VarianceTravelTime), NumberOfVehicles and the reserved-
    lane flow count. Shared base fields (transport_link_id, timestamp,
    quality, average_speed_kph, link_travel_time_seconds) are inherited
    from _TransportLinkDynamicBase.
    """

    kind: Literal["gps"] = Field(
        default="gps",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"gps" selects the TransportLinkGPSDynamic payload; it is not free-text '
            "command content."
        ),
    )
    current_journey_time_seconds: float | None = Field(
        default=None,
        ge=0.0,
        description="Current journey time in seconds.",
    )
    mean_travel_time_seconds: float | None = Field(
        default=None,
        ge=0.0,
        description="Mean travel time along the link in the measured interval, in seconds.",
    )
    median_travel_time_seconds: float | None = Field(
        default=None,
        ge=0.0,
        description="Median travel time along the link in the measured interval, in seconds.",
    )
    variance_travel_time_seconds: float | None = Field(
        default=None,
        ge=0.0,
        description=(
            "Variance of travel times along the link in the measured interval. "
            "The unit is seconds squared (s²), as for any variance of times; "
            "the field name keeps 'Seconds' for compatibility with v1."
        ),
    )
    number_of_vehicles: int | None = Field(
        default=None,
        ge=0,
        description="Number of vehicles in the measured interval.",
    )
    # XSD: ReservedLane (VehiclesPerInterval, xsd:decimal); narrowed to an
    # integer until this was relaxed.
    reserved_lane_flow_vehicles: float | None = Field(
        default=None,
        ge=0.0,
        allow_inf_nan=False,
        description=(
            "Flow in the reserved lane in the measured interval, in vehicles. "
            "A rate, so it may be fractional."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


TransportLinkMeasurement = Annotated[
    Union[
        TransportLinkSCOOTDynamic,
        TransportLinkANPRDynamic,
        TransportLinkGPSDynamic,
    ],
    Field(discriminator="kind"),
]
"""Canonical payload type for the `transportLinkMeasurement` AsyncAPI channel.

A discriminated union over the three measurement variants (SCOOT, ANPR
and GPS), keyed by the `kind` property. This is the file-level $ref
target when emit.py produces transport_link_measurement-v1.0.json.
"""
