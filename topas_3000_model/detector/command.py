# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_command-v1.0.xsd (AbstractDeviceCommandGroup)
# Changes:    Separated command and response into distinct schemas; discriminated union with 'kind' field replaces XSD generic command string; camelCase JSON output via Pydantic alias_generator; correlation id bounded to 2**53 - 1
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Detector commands.

Maps to the `detectorCommand` AsyncAPI channel. A command payload is one
of several variants keyed by the `kind` discriminator, each with its own
parameters.

Field sources: utmc_device_command-v1.0.xsd. The XSD used generic text
commands; we specialise to a discriminated union with typed payloads.
"""

from typing import Annotated, Any, Literal, Union

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp


class _DetectorCommandBase(UTMCBaseModel):
    """Shared fields for every detector command variant."""

    detector_id: ObjectId = Field(
        description="Identifier of the detector receiving the command.",
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


class ResetDetectorCommand(_DetectorCommandBase):
    """Reset the detector to its default state."""

    kind: Literal["reset"] = Field(
        default="reset",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"reset" selects the ResetDetectorCommand payload; it is not free-text '
            "command content."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


class CalibrateDetectorCommand(_DetectorCommandBase):
    """Trigger a calibration cycle on the detector."""

    kind: Literal["calibrate"] = Field(
        default="calibrate",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"calibrate" selects the CalibrateDetectorCommand payload; it is not free-text '
            "command content."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


class SetThresholdDetectorCommand(_DetectorCommandBase):
    """Set occupancy/flow threshold values on the detector."""

    kind: Literal["setThreshold"] = Field(
        default="setThreshold",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"setThreshold" selects the SetThresholdDetectorCommand payload; it is not free-text '
            "command content."
        ),
    )
    threshold_up: int = Field(
        ge=0,
        description="Upward threshold value to set.",
    )
    threshold_down: int = Field(
        ge=0,
        description="Downward threshold value to set.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


DetectorCommand = Annotated[
    Union[ResetDetectorCommand, CalibrateDetectorCommand, SetThresholdDetectorCommand],
    Field(discriminator="kind"),
]
"""Canonical payload type for the `detectorCommand` AsyncAPI channel."""
