# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_command-v1.0.xsd (DeviceCommandResponseGroup) via
#             UTMC-TS004.0061:2010 Annex D. TS004.0064bis:2017 defers to
#             TS004.0061 for concepts it does not redefine (§1.1.2).
# Changes:    Standalone copy — the 2017 profile previously re-exported from
#             topas_3000_model.traffic_signal_2010.response but now duplicates the
#             class locally so the package is self-contained.
#             2026-10-03: correlation id bounded to 2**53 - 1.
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Traffic signal command response.

The reply to a `traffic-signal.command` request. It has a schema and a message but no channel: it is sent
to the request's `reply-to` address (A11). Correlated to the original command by
`commandCorrelationId`, self-describing via `trafficSignalId`.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.command import CommandResponseStatus
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp


class TrafficSignalCommandResponse(UTMCBaseModel):
    """Response to a traffic signal command."""

    traffic_signal_id: ObjectId = Field(
        description="Identifier of the traffic signal the command targeted.",
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
    code: str | None = Field(
        default=None,
        description="Machine-readable response code for programmatic handling.",
    )
    message: str | None = Field(
        default=None,
        description="Human-readable response message with details.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )
