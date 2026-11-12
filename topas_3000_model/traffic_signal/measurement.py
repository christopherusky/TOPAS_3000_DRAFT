# ──────────────────────────────────────────────────────────────────────
# Origin:     UTMC-TS004.0064bis:2017 §3.5 (TrafficSignalDynamicState) + §3.6
#             (PlanLine, SignalGroupTimings worked example)
# XSD:        UTMCv1_xml/UTMC-TS004.0064bis-2017-traffic-signal-interface.xsd
#             complexTypes: TrafficSignalDynamicState, PlanLine, SignalGroupTimings
# Changes:    DATEX II publication envelope dropped (F1); SignalGroup class named
#             plural per XSD authority; cycle_time optional in dict;
#             offset cross-field validator (§3.5 "0..cycleTime-1");
#             offset-required-if-SGT cross-field validator (§3.5);
#             extensions slot preserved on all three classes (F9);
#             §5.1 mode/subMode combinations NOT enforced.
#             Ranges written as "XSD:" in the field comments come from the PDF, not the XSD.
#             cycle time, offset and primary-group duration are whole seconds where
#             the XSD type is Float.
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""TrafficSignalDynamicState and the PlanLine / SignalGroupTimings composition tree.

The 2017 spec's PlanLine / SignalGroupTimings structure pairs a named
signal group (`primarySignalGroup`) with its duration
(`primarySignalGroupDuration`), fixing the 2010 model's ambiguous
single-integer stage_sequence + unpaired stage_durations_seconds array.
The §3.6 worked example demonstrates how the structure renders as an
operator-facing plan line with start times.
"""

from typing import Any

from pydantic import Field, model_validator

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp
from topas_3000_model.traffic_signal.enums import (
    TrafficSignalModeEnum,
    TrafficSignalSubModeEnum,
)


class SignalGroupTimings(UTMCBaseModel):
    """Signal-group timing row within a PlanLine (§3.5, §3.6)."""

    # XSD: SignalGroupTimings/primarySignalGroup (String, minOccurs=1); the range 1..20 is from PDF §3.5, the XSD states none
    primary_signal_group: str = Field(
        min_length=1,
        max_length=20,
        description="A text representation of the primary signal group.",
    )
    # XSD: SignalGroupTimings/primarySignalGroupDuration (Seconds, minOccurs=1); the range 2..998 is from PDF §3.5, the XSD states none
    primary_signal_group_duration_seconds: int = Field(
        alias="primarySignalGroupDuration",
        ge=2,
        le=998,
        description=(
            "The length of the primary signal group. Note that there is no "
            "timing data associated with the alternate signal group."
        ),
    )
    # XSD: SignalGroupTimings/alternateSignalGroup (String, minOccurs=0); the range 1..20 is from PDF §3.5, the XSD states none
    alternate_signal_group: str | None = Field(
        default=None,
        min_length=1,
        max_length=20,
        description="A text representation of the alternatives to the primary signal group.",
    )
    # XSD: SignalGroupTimings/signalGroupTimingsExtension (_ExtensionType, minOccurs=0) per F9
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or forward-compatibility extensions. Open-content object.",
    )


class PlanLine(UTMCBaseModel):
    """A signal plan definition carried in the dynamic state (§3.5, §3.6)."""

    # XSD: PlanLine/planNumber (NonNegativeInteger, minOccurs=1); the range 1..99 is from PDF §3.5, the XSD states none
    plan_number: int = Field(
        ge=1,
        le=99,
        description="The currently active plan.",
    )
    # XSD: PlanLine/planStartTime (DateTime, minOccurs=1); PDF §3.5
    # Relaxed to 0..1. A publisher emitting
    # dynamic state on a fixed interval (rather than only on change) may not
    # know when the running plan started.
    plan_start_time: Timestamp | None = Field(
        default=None,
        description=(
            "The time the plan was started. Optional: a publisher emitting on "
            "a fixed interval may not know the start time of the running plan."
        ),
    )
    # XSD: PlanLine/cycleTime (Seconds, minOccurs=1 in XSD, 0..1 in dict); the range 1..1000 is from PDF §3.5, the XSD states none
    cycle_time_seconds: int | None = Field(
        alias="cycleTime",
        default=None,
        ge=1,
        le=1000,
        description="The current cycle time.",
    )
    # XSD: PlanLine/planDescription (String, minOccurs=0); the range 1..50 is from PDF §3.5, the XSD states none
    plan_description: str | None = Field(
        default=None,
        min_length=1,
        max_length=50,
        description="A user entered description of the active plan.",
    )
    # XSD: PlanLine/offset (Seconds, minOccurs=0); the bound 0..cycleTime-1 is from PDF §3.5, the XSD states none
    offset_seconds: int | None = Field(
        alias="offset",
        default=None,
        ge=0,
        description=(
            "The offset to the first SignalGroupTiming in the list. "
            "Must be present if SignalGroupTimingList is present."
        ),
    )
    # XSD: PlanLine/signalGroupTimings (SignalGroupTimings 0..*, minOccurs=0); PDF §3.5, §3.6
    signal_group_timings: list[SignalGroupTimings] | None = Field(
        default=None,
        description="The signal group data for the plan.",
    )
    # XSD: PlanLine/planLineExtension (_ExtensionType, minOccurs=0) per F9
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or forward-compatibility extensions. Open-content object.",
    )

    # Python-only: this rule is not in the emitted schema, so other SDKs do not enforce it.
    @model_validator(mode="after")
    def _validate_offset(self) -> "PlanLine":
        # §3.5 definition: "Must be present if SignalGroupTimingList is present"
        if self.signal_group_timings is not None and self.offset_seconds is None:
            raise ValueError("offset must be present if signal_group_timings is present")
        # §3.5 type: "Seconds (0..cycleTime-1)"
        if (
            self.offset_seconds is not None
            and self.cycle_time_seconds is not None
            and self.offset_seconds >= self.cycle_time_seconds
        ):
            raise ValueError("offset must be less than cycle_time")
        return self


class TrafficSignalDynamicState(UTMCBaseModel):
    """Dynamic operational state for a traffic signal (§3.5)."""

    # Payload-level binder; PDF §3.5 (implied by association from TrafficSignalStatus)
    traffic_signal_id: ObjectId = Field(
        description="Identifier of the traffic signal this reading is about.",
    )
    # Payload-level observation time; project convention
    timestamp: Timestamp = Field(
        description="Time the state was observed.",
    )
    # XSD: TrafficSignalDynamicState/trafficSignalMode (enum, minOccurs=1); PDF §3.5, §4.3
    traffic_signal_mode: TrafficSignalModeEnum = Field(
        description="The high level mode of the traffic signal.",
    )
    # XSD: TrafficSignalDynamicState/trafficSignalSubMode (enum, minOccurs=0); PDF §3.5, §4.4
    traffic_signal_sub_mode: TrafficSignalSubModeEnum | None = Field(
        default=None,
        description="The detailed sub-mode of the traffic signal.",
    )
    # XSD: TrafficSignalDynamicState/subArea (String, minOccurs=0); PDF §3.5
    sub_area: str | None = Field(
        default=None,
        description=(
            "Name for the sub-area that identifies the group of signals to "
            "which the associated traffic signal equipment belongs."
        ),
    )
    # XSD: TrafficSignalDynamicState/planLine (PlanLine, minOccurs=0); PDF §3.5
    plan_line: PlanLine | None = Field(
        default=None,
        description=(
            "A plan line for this traffic signal. This is intended to provide "
            "information allowing the plan data to be displayed to an operator. "
            "It contains the current plan data and would be updated when the "
            "plan is changed. For adaptive central control, such as SCOOT, the "
            "information should be available periodically and contain the plan "
            "at that instant."
        ),
    )
    # XSD: TrafficSignalDynamicState/trafficSignalDynamicStateExtension (_ExtensionType, minOccurs=0) per F9
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or forward-compatibility extensions. Open-content object.",
    )
