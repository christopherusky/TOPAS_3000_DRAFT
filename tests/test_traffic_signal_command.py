# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.5
# Description: Traffic-signal commands: setPlan, forceStage, holdStage,
#              synchronizeClock and requestStatus, the clock target
#              requirement, discriminator dispatch, and rejection of unknown
#              kinds and negative plan numbers or correlation ids.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.traffic_signal.command."""

from datetime import datetime, timezone

import pytest
from pydantic import TypeAdapter, ValidationError

from topas_3000_model.traffic_signal.command import (
    ForceStageCommand,
    HoldStageCommand,
    RequestStatusCommand,
    SetPlanCommand,
    SynchronizeClockCommand,
    TrafficSignalCommand,
)


NOW = datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc)
_TA = TypeAdapter(TrafficSignalCommand)


def test_set_plan_minimal():
    c = SetPlanCommand(
        traffic_signal_id="ts-001",
        command_correlation_id=1,
        request_time=NOW,
        plan_number=5,
    )
    assert c.kind == "setPlan"
    assert c.plan_number == 5


def test_force_stage_minimal():
    c = ForceStageCommand(
        traffic_signal_id="ts-001",
        command_correlation_id=2,
        request_time=NOW,
        stage_number=3,
    )
    assert c.kind == "forceStage"
    assert c.stage_number == 3


def test_hold_stage_minimal():
    c = HoldStageCommand(
        traffic_signal_id="ts-001",
        command_correlation_id=3,
        request_time=NOW,
        stage_number=2,
    )
    assert c.kind == "holdStage"
    assert c.stage_number == 2


def test_synchronize_clock_with_target():
    target = datetime(2026, 4, 11, 12, 0, tzinfo=timezone.utc)
    c = SynchronizeClockCommand(
        traffic_signal_id="ts-001",
        command_correlation_id=4,
        request_time=NOW,
        target_time=target,
    )
    assert c.kind == "synchronizeClock"
    assert c.target_time == target


def test_synchronize_clock_requires_target():
    with pytest.raises(ValidationError):
        SynchronizeClockCommand(
            traffic_signal_id="ts-001",
            command_correlation_id=4,
            request_time=NOW,
        )


def test_request_status_minimal():
    c = RequestStatusCommand(
        traffic_signal_id="ts-001",
        command_correlation_id=5,
        request_time=NOW,
    )
    assert c.kind == "requestStatus"


def test_discriminator_routes_set_plan():
    obj = _TA.validate_python({
        "kind": "setPlan",
        "traffic_signal_id": "ts-001",
        "command_correlation_id": 1,
        "request_time": NOW,
        "plan_number": 10,
    })
    assert isinstance(obj, SetPlanCommand)
    assert obj.plan_number == 10


def test_discriminator_routes_force_stage():
    obj = _TA.validate_python({
        "kind": "forceStage",
        "traffic_signal_id": "ts-001",
        "command_correlation_id": 2,
        "request_time": NOW,
        "stage_number": 1,
    })
    assert isinstance(obj, ForceStageCommand)


def test_discriminator_rejects_unknown_kind():
    with pytest.raises(ValidationError):
        _TA.validate_python({
            "kind": "teleport",
            "traffic_signal_id": "ts-001",
            "command_correlation_id": 1,
            "request_time": NOW,
        })


def test_set_plan_dumps_camel_case():
    c = SetPlanCommand(
        traffic_signal_id="ts-001",
        command_correlation_id=1,
        request_time=NOW,
        plan_number=5,
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert "trafficSignalId" in dumped
    assert "commandCorrelationId" in dumped
    assert "requestTime" in dumped
    assert "kind" in dumped
    assert "planNumber" in dumped
    assert dumped["kind"] == "setPlan"


def test_rejects_negative_command_correlation_id():
    with pytest.raises(ValidationError):
        SetPlanCommand(
            traffic_signal_id="ts-001",
            command_correlation_id=-1,
            request_time=NOW,
            plan_number=5,
        )


def test_rejects_negative_plan_number():
    with pytest.raises(ValidationError):
        SetPlanCommand(
            traffic_signal_id="ts-001",
            command_correlation_id=1,
            request_time=NOW,
            plan_number=-1,
        )


@pytest.mark.parametrize("plan", [0, 100, 100000])
def test_rejects_plan_number_outside_reportable_range(plan):
    """A plan the controller can be told to run must be reportable (PlanLine 1..99)."""
    with pytest.raises(ValidationError):
        SetPlanCommand(
            traffic_signal_id="ts-001",
            command_correlation_id=1,
            request_time=NOW,
            plan_number=plan,
        )


@pytest.mark.parametrize("plan", [1, 99])
def test_accepts_plan_number_range_limits(plan):
    c = SetPlanCommand(
        traffic_signal_id="ts-001",
        command_correlation_id=1,
        request_time=NOW,
        plan_number=plan,
    )
    assert c.plan_number == plan
