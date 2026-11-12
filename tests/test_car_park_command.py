# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.4
# Description: Car-park commands: setOccupancy, setCapacity, setThresholds and
#              requestStatus construction, rejection of a partial threshold
#              set, discriminator dispatch, and rejection of unknown kinds and
#              negative occupancy or correlation ids.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.car_park.command."""

from datetime import datetime, timezone

import pytest
from pydantic import TypeAdapter, ValidationError

from topas_3000_model.car_park.command import (
    CarParkCommand,
    RequestCarParkStatusCommand,
    SetCapacityCommand,
    SetOccupancyCommand,
    SetThresholdsCommand,
)


NOW = datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc)
_TA = TypeAdapter(CarParkCommand)


def test_set_occupancy_minimal():
    c = SetOccupancyCommand(
        car_park_id="cp-001",
        command_correlation_id=1,
        request_time=NOW,
        occupancy=320,
    )
    assert c.kind == "setOccupancy"
    assert c.occupancy == 320


def test_set_capacity_full():
    c = SetCapacityCommand(
        car_park_id="cp-001",
        command_correlation_id=2,
        request_time=NOW,
        total_capacity=500,
        disabled_capacity=20,
    )
    assert c.kind == "setCapacity"
    assert c.total_capacity == 500
    assert c.disabled_capacity == 20


def test_set_capacity_without_disabled():
    c = SetCapacityCommand(
        car_park_id="cp-001",
        command_correlation_id=2,
        request_time=NOW,
        total_capacity=500,
    )
    assert c.disabled_capacity is None


def test_set_thresholds_full():
    c = SetThresholdsCommand(
        car_park_id="cp-001",
        command_correlation_id=3,
        request_time=NOW,
        almost_full_increasing=400,
        almost_full_decreasing=380,
        full_increasing=490,
        full_decreasing=470,
    )
    assert c.kind == "setThresholds"
    assert c.almost_full_increasing == 400
    assert c.full_decreasing == 470


def test_set_thresholds_rejects_partial():
    with pytest.raises(ValidationError):
        SetThresholdsCommand(
            car_park_id="cp-001",
            command_correlation_id=3,
            request_time=NOW,
            almost_full_increasing=400,
        )


def test_request_status_minimal():
    c = RequestCarParkStatusCommand(
        car_park_id="cp-001",
        command_correlation_id=4,
        request_time=NOW,
    )
    assert c.kind == "requestStatus"


def test_discriminator_routes_set_occupancy():
    obj = _TA.validate_python({
        "kind": "setOccupancy",
        "car_park_id": "cp-001",
        "command_correlation_id": 1,
        "request_time": NOW,
        "occupancy": 100,
    })
    assert isinstance(obj, SetOccupancyCommand)
    assert obj.occupancy == 100


def test_discriminator_routes_set_capacity():
    obj = _TA.validate_python({
        "kind": "setCapacity",
        "car_park_id": "cp-001",
        "command_correlation_id": 2,
        "request_time": NOW,
        "total_capacity": 500,
    })
    assert isinstance(obj, SetCapacityCommand)


def test_discriminator_routes_set_thresholds():
    obj = _TA.validate_python({
        "kind": "setThresholds",
        "car_park_id": "cp-001",
        "command_correlation_id": 3,
        "request_time": NOW,
        "almost_full_increasing": 400,
        "almost_full_decreasing": 380,
        "full_increasing": 490,
        "full_decreasing": 470,
    })
    assert isinstance(obj, SetThresholdsCommand)


def test_discriminator_rejects_unknown_kind():
    with pytest.raises(ValidationError):
        _TA.validate_python({
            "kind": "teleport",
            "car_park_id": "cp-001",
            "command_correlation_id": 1,
            "request_time": NOW,
        })


def test_set_occupancy_dumps_camel_case():
    c = SetOccupancyCommand(
        car_park_id="cp-001",
        command_correlation_id=1,
        request_time=NOW,
        occupancy=320,
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert "carParkId" in dumped
    assert "commandCorrelationId" in dumped
    assert "requestTime" in dumped
    assert "kind" in dumped
    assert "occupancy" in dumped
    assert dumped["kind"] == "setOccupancy"


def test_rejects_negative_command_correlation_id():
    with pytest.raises(ValidationError):
        SetOccupancyCommand(
            car_park_id="cp-001",
            command_correlation_id=-1,
            request_time=NOW,
            occupancy=100,
        )


def test_rejects_negative_occupancy():
    with pytest.raises(ValidationError):
        SetOccupancyCommand(
            car_park_id="cp-001",
            command_correlation_id=1,
            request_time=NOW,
            occupancy=-1,
        )
