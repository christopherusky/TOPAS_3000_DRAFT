# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_command-v1.0.xsd (AbstractDeviceCommandGroup)
# Changes:    Separated command and response into distinct schemas; discriminated union with 'kind' field replaces XSD generic command string; camelCase JSON output via Pydantic alias_generator; correlation id bounded to 2**53 - 1
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Car Park commands.

Maps to the `carParkCommand` AsyncAPI channel. A command payload is one
of several variants keyed by the `kind` discriminator, each with its own
parameters.

Field sources: utmc_device_command-v1.0.xsd AbstractDeviceCommandGroup
and DeviceCommandResponseGroup. The legacy XSD uses generic
Command/Response strings; we specialise to a discriminated union with
typed payloads for occupancy, capacity, threshold, and status commands.
"""

from typing import Annotated, Any, Literal, Union

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp


class _CarParkCommandBase(UTMCBaseModel):
    """Shared fields for every car park command variant."""

    car_park_id: ObjectId = Field(
        description="Identifier of the car park receiving the command.",
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


class SetOccupancyCommand(_CarParkCommandBase):
    """Set the current occupancy count on a car park."""

    kind: Literal["setOccupancy"] = Field(
        default="setOccupancy",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"setOccupancy" selects the SetOccupancyCommand payload; it is not free-text '
            "command content."
        ),
    )
    occupancy: int = Field(
        ge=0,
        description="Occupancy count to set.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


class SetCapacityCommand(_CarParkCommandBase):
    """Set the capacity values on a car park."""

    kind: Literal["setCapacity"] = Field(
        default="setCapacity",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"setCapacity" selects the SetCapacityCommand payload; it is not free-text '
            "command content."
        ),
    )
    total_capacity: int = Field(
        ge=0,
        description="Total capacity to set.",
    )
    disabled_capacity: int | None = Field(
        default=None,
        ge=0,
        description="Disabled capacity to set.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


class SetThresholdsCommand(_CarParkCommandBase):
    """Set the occupancy threshold values on a car park."""

    kind: Literal["setThresholds"] = Field(
        default="setThresholds",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"setThresholds" selects the SetThresholdsCommand payload; it is not free-text '
            "command content."
        ),
    )
    almost_full_increasing: int = Field(
        ge=0,
        description="AFI threshold.",
    )
    almost_full_decreasing: int = Field(
        ge=0,
        description="AFD threshold.",
    )
    full_increasing: int = Field(
        ge=0,
        description="FI threshold.",
    )
    full_decreasing: int = Field(
        ge=0,
        description="FD threshold.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


class RequestCarParkStatusCommand(_CarParkCommandBase):
    """Request the current status of a car park."""

    kind: Literal["requestStatus"] = Field(
        default="requestStatus",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"requestStatus" selects the RequestCarParkStatusCommand payload; it is not free-text '
            "command content."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


CarParkCommand = Annotated[
    Union[
        SetOccupancyCommand,
        SetCapacityCommand,
        SetThresholdsCommand,
        RequestCarParkStatusCommand,
    ],
    Field(discriminator="kind"),
]
"""Canonical payload type for the `carParkCommand` AsyncAPI channel."""
