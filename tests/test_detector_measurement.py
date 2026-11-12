# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.2
# Description: The detector measurement discriminated union: flow with vehicle
#              classes, occupancy, queue, speed and headway variants,
#              discriminator dispatch, unknown-kind rejection, range
#              enforcement, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.detector.measurement."""

from datetime import datetime, timezone

import pytest
from pydantic import TypeAdapter, ValidationError

from topas_3000_model.detector.measurement import (
    DetectorMeasurement,
    FlowDynamic,
    HeadwayDynamic,
    OccupancyDynamic,
    QueueDynamic,
    SpeedDynamic,
)


NOW = datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc)


def test_flow_dynamic_minimal():
    f = FlowDynamic(
        detector_id="det-001",
        timestamp=NOW,
        total_flow=42,
        period_seconds=300,
    )
    assert f.kind == "flow"
    assert f.total_flow == 42
    assert f.period_seconds == 300
    assert f.class_counts == []


def test_flow_dynamic_with_classes():
    f = FlowDynamic(
        detector_id="det-001",
        timestamp=NOW,
        total_flow=42,
        period_seconds=300,
        class_counts=[10, 20, 5, 7],
    )
    assert f.class_counts == [10, 20, 5, 7]


def test_flow_dynamic_rejects_negative_flow():
    with pytest.raises(ValidationError):
        FlowDynamic(
            detector_id="det-001",
            timestamp=NOW,
            total_flow=-1,
            period_seconds=300,
        )


def test_occupancy_dynamic_valid():
    o = OccupancyDynamic(
        detector_id="det-001",
        timestamp=NOW,
        occupancy_percent=47.5,
        period_seconds=300,
    )
    assert o.kind == "occupancy"
    assert o.occupancy_percent == 47.5


def test_occupancy_dynamic_rejects_over_100():
    with pytest.raises(ValidationError):
        OccupancyDynamic(
            detector_id="det-001",
            timestamp=NOW,
            occupancy_percent=150.0,
            period_seconds=300,
        )


def test_queue_dynamic_minimal():
    q = QueueDynamic(detector_id="det-001", timestamp=NOW, queue_present=True)
    assert q.kind == "queue"
    assert q.queue_present is True
    assert q.severity is None


def test_queue_dynamic_with_severity():
    q = QueueDynamic(
        detector_id="det-001",
        timestamp=NOW,
        queue_present=True,
        severity="severe",
    )
    assert q.severity == "severe"


def test_queue_dynamic_rejects_unknown_severity():
    with pytest.raises(ValidationError):
        QueueDynamic(
            detector_id="det-001",
            timestamp=NOW,
            queue_present=True,
            severity="very bad",
        )


def test_speed_dynamic_valid():
    s = SpeedDynamic(
        detector_id="det-001",
        timestamp=NOW,
        mean_speed_kph=60.0,
        period_seconds=300,
    )
    assert s.kind == "speed"
    assert s.mean_speed_kph == 60.0


def test_headway_dynamic_valid():
    h = HeadwayDynamic(
        detector_id="det-001",
        timestamp=NOW,
        mean_headway_seconds=3.2,
        period_seconds=300,
    )
    assert h.kind == "headway"
    assert h.mean_headway_seconds == 3.2


def test_discriminated_union_routes_flow():
    adapter = TypeAdapter(DetectorMeasurement)
    instance = adapter.validate_python({
        "kind": "flow",
        "detectorId": "det-001",
        "timestamp": "2026-04-11T10:00:00Z",
        "totalFlow": 42,
        "periodSeconds": 300,
    })
    assert isinstance(instance, FlowDynamic)


def test_discriminated_union_routes_queue():
    adapter = TypeAdapter(DetectorMeasurement)
    instance = adapter.validate_python({
        "kind": "queue",
        "detectorId": "det-001",
        "timestamp": "2026-04-11T10:00:00Z",
        "queuePresent": False,
    })
    assert isinstance(instance, QueueDynamic)


def test_discriminated_union_rejects_unknown_kind():
    adapter = TypeAdapter(DetectorMeasurement)
    with pytest.raises(ValidationError):
        adapter.validate_python({
            "kind": "bogus",
            "detectorId": "det-001",
            "timestamp": "2026-04-11T10:00:00Z",
        })


def test_flow_dynamic_dumps_camel_case():
    f = FlowDynamic(
        detector_id="det-001",
        timestamp=NOW,
        total_flow=42,
        period_seconds=300,
    )
    dumped = f.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped == {
        "kind": "flow",
        "detectorId": "det-001",
        "timestamp": "2026-04-11T10:00:00Z",
        "totalFlow": 42,
        "periodSeconds": 300,
        "classCounts": [],
    }


def test_flow_dynamic_dumps_camel_case_by_default():
    """The base model's serialize_by_alias=True means model_dump() without
    explicit by_alias=True should still emit camelCase. This locks that
    contract so a future config change doesn't silently break producers."""
    f = FlowDynamic(
        detector_id="det-001",
        timestamp=NOW,
        total_flow=42,
        period_seconds=300,
    )
    dumped = f.model_dump(mode="json", exclude_none=True)
    assert "detectorId" in dumped
    assert "detector_id" not in dumped
    assert "totalFlow" in dumped
    assert "total_flow" not in dumped
    assert dumped["kind"] == "flow"


# ── Restored v1 optionality ──


def test_speed_reading_without_an_interval_is_valid():
    s = SpeedDynamic(detector_id="det-001", timestamp=NOW, mean_speed_kph=60.0)
    assert s.period_seconds is None


def test_class_counts_only_flow_reading_is_valid():
    f = FlowDynamic(detector_id="det-001", timestamp=NOW, class_counts=[4, 9])
    assert f.total_flow is None
    assert f.period_seconds is None


def test_class_counts_can_be_sparse():
    """Class 3 absent while class 4 is present."""
    f = FlowDynamic(detector_id="det-001", timestamp=NOW, class_counts=[4, 9, None, 2])
    assert f.class_counts == [4, 9, None, 2]
    dumped = f.model_dump(by_alias=True, mode="json")
    assert dumped["classCounts"] == [4, 9, None, 2]


def test_class_count_zero_is_distinct_from_absent():
    f = FlowDynamic(detector_id="det-001", timestamp=NOW, class_counts=[0, None])
    assert f.class_counts[0] == 0 and f.class_counts[1] is None


def test_class_counts_reject_negative_entries():
    with pytest.raises(ValidationError):
        FlowDynamic(detector_id="det-001", timestamp=NOW, class_counts=[3, -1])


def test_flow_total_is_not_checked_against_class_counts():
    """Partial class reporting is legitimate, so no sum check is applied."""
    f = FlowDynamic(detector_id="det-001", timestamp=NOW, total_flow=5, class_counts=[9, 9])
    assert f.total_flow == 5


@pytest.mark.parametrize(
    "model",
    [FlowDynamic, OccupancyDynamic, SpeedDynamic, HeadwayDynamic],
)
def test_a_reading_with_no_value_is_rejected(model):
    with pytest.raises(ValidationError):
        model(detector_id="det-001", timestamp=NOW)


def test_flow_with_only_null_class_counts_is_rejected():
    with pytest.raises(ValidationError):
        FlowDynamic(detector_id="det-001", timestamp=NOW, class_counts=[None, None])


def test_a_period_alone_is_not_a_reading():
    with pytest.raises(ValidationError):
        SpeedDynamic(detector_id="det-001", timestamp=NOW, period_seconds=300)


@pytest.mark.parametrize("bad", [float("nan"), float("inf")])
def test_non_finite_values_are_rejected(bad):
    with pytest.raises(ValidationError):
        SpeedDynamic(detector_id="det-001", timestamp=NOW, mean_speed_kph=bad)
    with pytest.raises(ValidationError):
        OccupancyDynamic(detector_id="det-001", timestamp=NOW, occupancy_percent=bad)
    with pytest.raises(ValidationError):
        HeadwayDynamic(detector_id="det-001", timestamp=NOW, mean_headway_seconds=bad)


def test_class_counts_hold_at_most_eight_classes():
    FlowDynamic(detector_id="det-001", timestamp=NOW, class_counts=[1] * 8)
    with pytest.raises(ValidationError):
        FlowDynamic(detector_id="det-001", timestamp=NOW, class_counts=[1] * 9)
