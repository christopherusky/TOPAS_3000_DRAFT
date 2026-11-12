# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Strategy definition and enums: minimal and full construction,
#              rejection of an empty id or name, camelCase serialisation, and
#              the StrategyStatus, TriggerState and TriggerUpdateError value
#              sets.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.strategy.definition and enums."""

import pytest
from pydantic import ValidationError

from topas_3000_model.common.geometry import GeoJsonPoint
from topas_3000_model.strategy.definition import StrategyDefinition
from topas_3000_model.strategy.enums import (
    StrategyStatusEnum,
    TriggerStateEnum,
    TriggerUpdateErrorEnum,
)


def test_strategy_definition_minimal():
    d = StrategyDefinition(
        strategy_id="STR00000001",
        strategy_name="Peak Hour Congestion",
    )
    assert d.strategy_id == "STR00000001"
    assert d.strategy_name == "Peak Hour Congestion"
    assert d.strategy_description is None
    assert d.location is None


def test_strategy_definition_with_all_fields():
    d = StrategyDefinition(
        strategy_id="STR00000001",
        strategy_name="Peak Hour Congestion",
        strategy_description="Activates gating and signal coordination on A52 corridor",
        location=GeoJsonPoint(coordinates=(-1.5, 52.9)),
        extensions={"area": "city-centre"},
    )
    assert d.strategy_description.startswith("Activates")
    assert d.location.coordinates == (-1.5, 52.9)
    assert d.extensions == {"area": "city-centre"}


def test_strategy_definition_rejects_empty_id():
    with pytest.raises(ValidationError):
        StrategyDefinition(strategy_id="", strategy_name="Test")


def test_strategy_definition_rejects_empty_name():
    with pytest.raises(ValidationError):
        StrategyDefinition(strategy_id="STR001", strategy_name="")


def test_strategy_definition_dumps_camel_case():
    d = StrategyDefinition(
        strategy_id="STR00000001",
        strategy_name="Peak Hour Congestion",
        strategy_description="A52 corridor strategy",
        location=GeoJsonPoint(coordinates=(-1.5, 52.9)),
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["strategyId"] == "STR00000001"
    assert dumped["strategyName"] == "Peak Hour Congestion"
    assert dumped["strategyDescription"] == "A52 corridor strategy"
    assert dumped["location"]["type"] == "Point"
    assert dumped["location"]["coordinates"] == [-1.5, 52.9]


def test_strategy_status_enum_values():
    assert StrategyStatusEnum.ACTIVE.value == "active"
    assert StrategyStatusEnum.INACTIVE.value == "inactive"
    assert len(StrategyStatusEnum) == 2


def test_trigger_state_enum_values():
    assert TriggerStateEnum.ENABLED.value == "enabled"
    assert TriggerStateEnum.DISABLED.value == "disabled"
    assert len(TriggerStateEnum) == 2


def test_trigger_update_error_enum_values():
    assert TriggerUpdateErrorEnum.ACCESS_DENIED.value == "accessDenied"
    assert TriggerUpdateErrorEnum.NOT_AUTHENTICATED.value == "notAuthenticated"
    assert TriggerUpdateErrorEnum.STRATEGY_ID_DOES_NOT_EXIST.value == "strategyIdDoesNotExist"
    assert TriggerUpdateErrorEnum.OTHER.value == "other"
    assert len(TriggerUpdateErrorEnum) == 4
