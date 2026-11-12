# ──────────────────────────────────────────────────────────────────────
# Origin:     DS003 Strategy Interface Specification v1.2 (StrategyStatus schema)
# Changes:    Separate from definition, referenced by strategy_id; camelCase JSON output via Pydantic alias_generator
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Strategy Status.

Current status of a traffic management strategy, published when
the strategy changes state.

Maps to the `strategyStatus` AsyncAPI channel.

Field sources: DS003 v1.2, StrategyStatus schema. Definition is
NOT embedded — referenced by strategy_id only.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.time import Timestamp
from topas_3000_model.strategy.enums import StrategyStatusEnum


class StrategyStatus(UTMCBaseModel):
    """Current status of a traffic management strategy."""

    strategy_id: str = Field(
        min_length=1,
        description="Identifier of the strategy this status is about.",
    )
    strategy_status: StrategyStatusEnum = Field(
        description="Current status (active/inactive).",
    )
    status_message: str | None = Field(
        default=None,
        description="Non-error status information (e.g. 'Strategy applied 6 of 10 measures').",
    )
    error_message: str | None = Field(
        default=None,
        description="Error information (e.g. 'Strategy will never become active').",
    )
    strategy_change_state_time: Timestamp = Field(
        description="When the strategy status changed.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )
