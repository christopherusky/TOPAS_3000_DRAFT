# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_command-v1.0.xsd (AbstractDeviceCommandGroup)
# Changes:    Separated command and response into distinct schemas; discriminated union with 'kind' field replaces XSD generic command string; camelCase JSON output via Pydantic alias_generator; correlation id bounded to 2**53 - 1
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Meteorological commands.

Maps to the `meteorologicalCommand` AsyncAPI channel.
"""

from typing import Annotated, Any, Literal, Union

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp


class _MeteorologicalCommandBase(UTMCBaseModel):
    """Shared fields for every meteorological command variant."""

    meteorological_id: ObjectId = Field(
        description="Identifier of the met station receiving the command.",
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


class RequestMeteorologicalStatusCommand(_MeteorologicalCommandBase):
    """Request the current status of a meteorological station."""

    kind: Literal["requestStatus"] = Field(
        default="requestStatus",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"requestStatus" selects the RequestMeteorologicalStatusCommand payload; it is not free-text '
            "command content."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


MeteorologicalCommand = Annotated[
    Union[RequestMeteorologicalStatusCommand],
    Field(discriminator="kind"),
]
"""Canonical payload type for the `meteorologicalCommand` AsyncAPI channel."""
