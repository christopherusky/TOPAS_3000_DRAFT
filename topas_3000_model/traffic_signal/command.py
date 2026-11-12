# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_command-v1.0.xsd (AbstractDeviceCommandGroup) via
#             UTMC-TS004.0061:2010 Annex D. TS004.0064bis:2017 defers to
#             TS004.0061 for concepts it does not redefine (§1.1.2).
# Changes:    SetPlanCommand.plan_number bounded to 1 to 99, the range PlanLine reports, where it was 0 or more; HoldStageCommand documented as holding a specific stage; correlation id bounded to 2**53 - 1
#
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Traffic Signal commands.

Maps to the `trafficSignalCommand` AsyncAPI channel. A command payload is
one of several variants keyed by the `kind` discriminator, each with its
own parameters.

Field sources: utmc_device_command-v1.0.xsd AbstractDeviceCommandGroup
and DeviceCommandResponseGroup. The legacy XSD uses generic
Command/Response strings; we specialise to a discriminated union with
typed payloads.
"""

from typing import Annotated, Any, Literal, Union

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp


class _TrafficSignalCommandBase(UTMCBaseModel):
    """Shared fields for every traffic signal command variant."""

    traffic_signal_id: ObjectId = Field(
        description="Identifier of the traffic signal receiving the command.",
    )
    command_correlation_id: int = Field(
        ge=0,
        le=9007199254740991,  # 2**53 - 1: largest integer a JSON number carries exactly (I-JSON)
        description=(
            "Caller-assigned identifier correlating a command request with its "
            "response. Not an identifier of the command itself. Named to avoid "
            "collision with the AMQP 1.0 correlation-id message property."
        ),
    )
    request_time: Timestamp = Field(
        description="When the command was issued.",
    )


class SetPlanCommand(_TrafficSignalCommandBase):
    """Set the active plan on a traffic signal controller."""

    kind: Literal["setPlan"] = Field(
        default="setPlan",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"setPlan" selects the SetPlanCommand payload; it is not free-text '
            "command content."
        ),
    )
    plan_number: int = Field(
        ge=1,
        le=99,
        description=(
            "Plan number to activate. Same 1 to 99 range as the plan number a "
            "controller reports (PlanLine.plan_number), so any plan the "
            "controller can be told to run can be reported back."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


class ForceStageCommand(_TrafficSignalCommandBase):
    """Force a specific stage on a traffic signal controller."""

    kind: Literal["forceStage"] = Field(
        default="forceStage",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"forceStage" selects the ForceStageCommand payload; it is not free-text '
            "command content."
        ),
    )
    stage_number: int = Field(
        ge=0,
        description="Stage number to force.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


class HoldStageCommand(_TrafficSignalCommandBase):
    """Hold a specific stage on a traffic signal controller."""

    kind: Literal["holdStage"] = Field(
        default="holdStage",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"holdStage" selects the HoldStageCommand payload; it is not free-text '
            "command content."
        ),
    )
    stage_number: int = Field(
        ge=0,
        description="Stage number to hold.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


class SynchronizeClockCommand(_TrafficSignalCommandBase):
    """Synchronize the traffic signal controller's clock."""

    kind: Literal["synchronizeClock"] = Field(
        default="synchronizeClock",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"synchronizeClock" selects the SynchronizeClockCommand payload; it is not free-text '
            "command content."
        ),
    )
    target_time: Timestamp = Field(
        description="Target time to synchronize the controller clock to.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


class RequestStatusCommand(_TrafficSignalCommandBase):
    """Request the current status of a traffic signal controller."""

    kind: Literal["requestStatus"] = Field(
        default="requestStatus",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"requestStatus" selects the RequestStatusCommand payload; it is not free-text '
            "command content."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


TrafficSignalCommand = Annotated[
    Union[
        SetPlanCommand,
        ForceStageCommand,
        HoldStageCommand,
        SynchronizeClockCommand,
        RequestStatusCommand,
    ],
    Field(discriminator="kind"),
]
"""Canonical payload type for the `trafficSignalCommand` AsyncAPI channel."""
