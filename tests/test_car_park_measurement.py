# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Car-park dynamic measurements: occupancy, percentage, fill rate
#              and queue time, with rejection of a missing timestamp, negative
#              values and occupancy over 100 percent, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.car_park.measurement."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.car_park.measurement import CarParkDynamic
from topas_3000_model.common.quality import Quality


NOW = datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc)


def test_car_park_dynamic_minimal():
    d = CarParkDynamic(
        car_park_id="cp-001",
        timestamp=NOW,
    )
    assert d.car_park_id == "cp-001"
    assert d.timestamp == NOW
    assert d.quality is None
    assert d.occupancy is None
    assert d.occupancy_percent is None
    assert d.fill_rate_vehicles_per_minute is None
    assert d.exit_rate_vehicles_per_minute is None
    assert d.queue_time_seconds is None


def test_car_park_dynamic_full():
    d = CarParkDynamic(
        car_park_id="cp-001",
        timestamp=NOW,
        quality=Quality(quality_id=1, source_name="carpark-ctrl"),
        occupancy=320,
        occupancy_percent=64.0,
        fill_rate_vehicles_per_minute=5.5,
        exit_rate_vehicles_per_minute=2.1,
        queue_time_seconds=90.0,
    )
    assert d.occupancy == 320
    assert d.occupancy_percent == 64.0
    assert d.fill_rate_vehicles_per_minute == 5.5
    assert d.exit_rate_vehicles_per_minute == 2.1
    assert d.queue_time_seconds == 90.0
    assert d.quality is not None


def test_car_park_dynamic_rejects_missing_timestamp():
    with pytest.raises(ValidationError):
        CarParkDynamic(car_park_id="cp-001")


def test_car_park_dynamic_rejects_negative_occupancy():
    with pytest.raises(ValidationError):
        CarParkDynamic(
            car_park_id="cp-001",
            timestamp=NOW,
            occupancy=-1,
        )


def test_car_park_dynamic_rejects_occupancy_percent_over_100():
    with pytest.raises(ValidationError):
        CarParkDynamic(
            car_park_id="cp-001",
            timestamp=NOW,
            occupancy_percent=101.0,
        )


def test_car_park_dynamic_rejects_negative_occupancy_percent():
    with pytest.raises(ValidationError):
        CarParkDynamic(
            car_park_id="cp-001",
            timestamp=NOW,
            occupancy_percent=-1.0,
        )


def test_car_park_dynamic_rejects_negative_fill_rate():
    with pytest.raises(ValidationError):
        CarParkDynamic(
            car_park_id="cp-001",
            timestamp=NOW,
            fill_rate_vehicles_per_minute=-0.1,
        )


def test_car_park_dynamic_rejects_negative_queue_time():
    with pytest.raises(ValidationError):
        CarParkDynamic(
            car_park_id="cp-001",
            timestamp=NOW,
            queue_time_seconds=-5.0,
        )


def test_car_park_dynamic_dumps_camel_case():
    d = CarParkDynamic(
        car_park_id="cp-001",
        timestamp=NOW,
        occupancy=320,
        occupancy_percent=64.0,
        fill_rate_vehicles_per_minute=5.5,
        exit_rate_vehicles_per_minute=2.1,
        queue_time_seconds=90.0,
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["carParkId"] == "cp-001"
    assert dumped["timestamp"] == "2026-04-11T10:00:00Z"
    assert dumped["occupancy"] == 320
    assert dumped["occupancyPercent"] == 64.0
    assert dumped["fillRateVehiclesPerMinute"] == 5.5
    assert dumped["exitRateVehiclesPerMinute"] == 2.1
    assert dumped["queueTimeSeconds"] == 90.0
