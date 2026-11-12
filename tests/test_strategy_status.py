# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Strategy status: active and inactive states, messages,
#              progress, empty-id rejection, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.strategy.status."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.strategy.enums import StrategyStatusEnum
from topas_3000_model.strategy.status import StrategyStatus


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_strategy_status_active():
    s = StrategyStatus(
        strategy_id="STR00000001",
        strategy_status=StrategyStatusEnum.ACTIVE,
        strategy_change_state_time=NOW,
    )
    assert s.strategy_status == StrategyStatusEnum.ACTIVE
    assert s.status_message is None
    assert s.error_message is None


def test_strategy_status_inactive_with_messages():
    s = StrategyStatus(
        strategy_id="STR00000001",
        strategy_status=StrategyStatusEnum.INACTIVE,
        status_message="Strategy overridden by operator action",
        error_message="Conflicting strategy STR00000002 is active",
        strategy_change_state_time=NOW,
    )
    assert s.strategy_status == StrategyStatusEnum.INACTIVE
    assert s.status_message == "Strategy overridden by operator action"
    assert s.error_message == "Conflicting strategy STR00000002 is active"


def test_strategy_status_active_with_progress():
    s = StrategyStatus(
        strategy_id="STR00000001",
        strategy_status=StrategyStatusEnum.ACTIVE,
        status_message="Strategy applied 6 of 10 measures",
        strategy_change_state_time=NOW,
    )
    assert s.status_message == "Strategy applied 6 of 10 measures"


def test_strategy_status_rejects_empty_id():
    with pytest.raises(ValidationError):
        StrategyStatus(
            strategy_id="",
            strategy_status=StrategyStatusEnum.ACTIVE,
            strategy_change_state_time=NOW,
        )


def test_strategy_status_dumps_camel_case():
    s = StrategyStatus(
        strategy_id="STR00000001",
        strategy_status=StrategyStatusEnum.ACTIVE,
        status_message="All measures applied",
        strategy_change_state_time=NOW,
    )
    dumped = s.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["strategyId"] == "STR00000001"
    assert dumped["strategyStatus"] == "active"
    assert dumped["statusMessage"] == "All measures applied"
    assert dumped["strategyChangeStateTime"] == "2026-04-16T10:00:00Z"
