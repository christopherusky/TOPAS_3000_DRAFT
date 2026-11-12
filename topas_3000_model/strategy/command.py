# ──────────────────────────────────────────────────────────────────────
# Origin:     DS003 Strategy Interface Specification v1.2 (StrategyTrigger schema)
# Changes:    commandCorrelationId and requestTime added to match every other command; service_requester made optional and audit-only; AMQP payloads; camelCase JSON output via Pydantic alias_generator; correlation id bounded to 2**53 - 1
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.3
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Strategy Trigger (command).

Command to enable or disable a strategy remotely. Like every other command
it carries a caller-assigned ``command_correlation_id`` and a ``request_time``, so the response can be matched at application level and the Data
Publisher can execute a retried command once (A11.2).

``service_requester`` is an audit label only. It is sender-supplied, so it
MUST NOT be used for authorisation or idempotency; the requester's identity
comes from the authenticated messaging context (AC5-3).

Maps to the `strategyCommand` AsyncAPI channel.

Field sources: DS003 v1.2, StrategyTrigger schema.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.time import Timestamp
from topas_3000_model.strategy.enums import TriggerStateEnum


class StrategyTrigger(UTMCBaseModel):
    """Command to enable or disable a strategy remotely."""

    strategy_id: str = Field(
        min_length=1,
        description="Identifier of the strategy to trigger.",
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
    service_requester: str | None = Field(
        default=None,
        min_length=1,
        description=(
            "Audit label naming the requesting system. Sender-supplied, so "
            "it is never used for authorisation or idempotency."
        ),
    )
    trigger_state: TriggerStateEnum = Field(
        description="Desired trigger state (enabled/disabled).",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )
