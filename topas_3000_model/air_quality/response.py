# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_command-v1.0.xsd (DeviceCommandResponseGroup)
# Changes:    Separated command response into its own schema; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation; correlation id bounded to 2**53 - 1
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Air Quality command response.

Maps to the `airQualityCommandResponse` AsyncAPI channel.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.command import CommandResponseStatus
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp


class AirQualityCommandResponse(UTMCBaseModel):
    """Response to an air quality command."""

    air_quality_id: ObjectId = Field(
        description="Identifier of the monitor the command targeted.",
    )
    command_correlation_id: int = Field(
        ge=0,
        le=9007199254740991,  # 2**53 - 1: largest integer a JSON number carries exactly (I-JSON)
        description=(
            "Correlation identifier copied verbatim from the originating command "
            "request. Not an identifier of the command itself."
        ),
    )
    status: CommandResponseStatus = Field(
        description="Outcome of the command execution.",
    )
    response_time: Timestamp = Field(
        description="When the response was generated.",
    )
    code: str | None = Field(default=None, description="Machine-readable response code.")
    message: str | None = Field(default=None, description="Human-readable response message.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")
