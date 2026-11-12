# ──────────────────────────────────────────────────────────────────────
# Origin:     DS003 Strategy Interface Specification v1.2 (TriggerUpdateFeedback schema)
# Changes:    Separate command + response files; strategy-specific error_code from TriggerUpdateFeedback, now only permitted on a failure outcome; commandCorrelationId and generic code added to match every other command response; service_requester made optional and audit-only; camelCase JSON output via Pydantic alias_generator; correlation id bounded to 2**53 - 1
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.3
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Strategy Command Response.

Response to a strategy trigger command. Uses CommandResponseStatus
from common for the overall outcome, plus a strategy-specific error_code
for failure categorisation.

The reply to a `strategy.command` request. It has a schema and a message but no channel: it is
sent to the request's `reply-to` address (A11).

Field sources: DS003 v1.2, TriggerUpdateFeedback schema (mapped to
standard command response pattern).
"""

from typing import Any

from pydantic import Field, model_validator

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.command import CommandResponseStatus
from topas_3000_model.common.time import Timestamp
from topas_3000_model.strategy.enums import TriggerUpdateErrorEnum


class StrategyCommandResponse(UTMCBaseModel):
    """Response to a strategy trigger command."""

    strategy_id: str = Field(
        min_length=1,
        description="Identifier of the strategy the command targeted.",
    )
    command_correlation_id: int = Field(
        ge=0,
        le=9007199254740991,  # 2**53 - 1: largest integer a JSON number carries exactly (I-JSON)
        description=(
            "Correlation identifier copied verbatim from the originating command "
            "request. Not an identifier of the command itself."
        ),
    )
    service_requester: str | None = Field(
        default=None,
        min_length=1,
        description=(
            "Audit label copied from the command, if it carried one. "
            "Never used for authorisation or idempotency."
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
    error_code: TriggerUpdateErrorEnum | None = Field(
        default=None,
        description=(
            "Strategy-specific rejection category. Only permitted when the "
            "outcome is not success or pending."
        ),
    )
    message: str | None = Field(
        default=None,
        description="Human-readable detail.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )

    # Python-only: this rule is not in the emitted schema, so other SDKs do not enforce it.
    @model_validator(mode="after")
    def _error_code_requires_failure_outcome(self) -> "StrategyCommandResponse":
        """An error category on a success or pending outcome is a contradiction."""
        if self.error_code is not None and self.status in (
            CommandResponseStatus.SUCCESS,
            CommandResponseStatus.PENDING,
        ):
            raise ValueError(
                f"error_code is not permitted when status is {self.status.value!r}"
            )
        return self
