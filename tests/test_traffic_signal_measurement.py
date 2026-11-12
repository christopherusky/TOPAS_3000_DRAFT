# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-04-23
# Version:     1.1
# Description: TrafficSignalDynamicState with PlanLine and SignalGroupTimings:
#              duration and primary/alternate name bounds, extensions round-
#              trip, plan-number and cycle-time bounds, description length,
#              and the dependency of offset on signal-group timings.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for TrafficSignalDynamicState, PlanLine, SignalGroupTimings
(TS004.0064bis:2017 §3.5, §3.6)."""

import pytest
from pydantic import ValidationError

from topas_3000_model.traffic_signal.enums import (
    TrafficSignalModeEnum,
    TrafficSignalSubModeEnum,
)
from topas_3000_model.traffic_signal.measurement import (
    PlanLine,
    SignalGroupTimings,
    TrafficSignalDynamicState,
)


# ── SignalGroupTimings ──────────────────────────────────────────────


def test_sgt_minimal_valid():
    sgt = SignalGroupTimings.model_validate(
        {"primarySignalGroup": "A", "primarySignalGroupDuration": 40}
    )
    assert sgt.primary_signal_group == "A"
    assert sgt.primary_signal_group_duration_seconds == 40
    assert sgt.alternate_signal_group is None
    assert sgt.extensions is None


def test_sgt_with_alternate():
    sgt = SignalGroupTimings.model_validate(
        {
            "primarySignalGroup": "B",
            "primarySignalGroupDuration": 20,
            "alternateSignalGroup": "A",
        }
    )
    assert sgt.alternate_signal_group == "A"


def test_sgt_duration_below_min_rejected():
    with pytest.raises(ValidationError):
        SignalGroupTimings.model_validate(
            {"primarySignalGroup": "A", "primarySignalGroupDuration": 1}
        )


def test_sgt_duration_above_max_rejected():
    with pytest.raises(ValidationError):
        SignalGroupTimings.model_validate(
            {"primarySignalGroup": "A", "primarySignalGroupDuration": 999}
        )


def test_sgt_primary_too_long_rejected():
    with pytest.raises(ValidationError):
        SignalGroupTimings.model_validate(
            {"primarySignalGroup": "A" * 21, "primarySignalGroupDuration": 40}
        )


def test_sgt_primary_empty_rejected():
    with pytest.raises(ValidationError):
        SignalGroupTimings.model_validate(
            {"primarySignalGroup": "", "primarySignalGroupDuration": 40}
        )


def test_sgt_alternate_too_long_rejected():
    with pytest.raises(ValidationError):
        SignalGroupTimings.model_validate(
            {
                "primarySignalGroup": "A",
                "primarySignalGroupDuration": 40,
                "alternateSignalGroup": "B" * 21,
            }
        )


def test_sgt_extensions_round_trip():
    sgt = SignalGroupTimings.model_validate(
        {
            "primarySignalGroup": "A",
            "primarySignalGroupDuration": 40,
            "extensions": {"vendorField": 1},
        }
    )
    assert sgt.extensions == {"vendorField": 1}


# ── PlanLine ────────────────────────────────────────────────────────


def test_planline_minimal_valid():
    pl = PlanLine.model_validate(
        {"planNumber": 1, "planStartTime": "2026-04-22T10:00:00+00:00"}
    )
    assert pl.plan_number == 1
    assert pl.cycle_time_seconds is None
    assert pl.signal_group_timings is None


def test_planline_plan_number_bounds():
    with pytest.raises(ValidationError):
        PlanLine.model_validate({"planNumber": 0, "planStartTime": "2026-04-22T10:00:00+00:00"})
    with pytest.raises(ValidationError):
        PlanLine.model_validate({"planNumber": 100, "planStartTime": "2026-04-22T10:00:00+00:00"})


def test_planline_cycle_time_bounds():
    with pytest.raises(ValidationError):
        PlanLine.model_validate(
            {"planNumber": 1, "planStartTime": "2026-04-22T10:00:00+00:00", "cycleTime": 0}
        )
    with pytest.raises(ValidationError):
        PlanLine.model_validate(
            {"planNumber": 1, "planStartTime": "2026-04-22T10:00:00+00:00", "cycleTime": 1001}
        )


def test_planline_description_length_bounds():
    with pytest.raises(ValidationError):
        PlanLine.model_validate(
            {
                "planNumber": 1,
                "planStartTime": "2026-04-22T10:00:00+00:00",
                "planDescription": "",
            }
        )
    with pytest.raises(ValidationError):
        PlanLine.model_validate(
            {
                "planNumber": 1,
                "planStartTime": "2026-04-22T10:00:00+00:00",
                "planDescription": "X" * 51,
            }
        )


def test_planline_offset_requires_signal_group_timings_if_present_raises():
    """offset is required if signal_group_timings is present (§3.5)."""
    with pytest.raises(ValidationError, match="offset must be present"):
        PlanLine.model_validate(
            {
                "planNumber": 1,
                "planStartTime": "2026-04-22T10:00:00+00:00",
                "cycleTime": 120,
                "signalGroupTimings": [
                    {"primarySignalGroup": "A", "primarySignalGroupDuration": 40}
                ],
            }
        )


def test_planline_offset_must_be_less_than_cycle_time():
    """offset in Seconds (0..cycleTime-1) per §3.5."""
    with pytest.raises(ValidationError, match="offset must be less than"):
        PlanLine.model_validate(
            {
                "planNumber": 1,
                "planStartTime": "2026-04-22T10:00:00+00:00",
                "cycleTime": 120,
                "offset": 120,
            }
        )


def test_planline_pdf_worked_example():
    """§3.6 worked example: cycleTime 120, offset 7, four SGT rows A/BA/C/DC."""
    pl = PlanLine.model_validate(
        {
            "planNumber": 1,
            "planStartTime": "2026-04-22T10:00:00+00:00",
            "cycleTime": 120,
            "offset": 7,
            "signalGroupTimings": [
                {"primarySignalGroup": "A", "primarySignalGroupDuration": 40},
                {
                    "primarySignalGroup": "B",
                    "alternateSignalGroup": "A",
                    "primarySignalGroupDuration": 20,
                },
                {"primarySignalGroup": "C", "primarySignalGroupDuration": 30},
                {
                    "primarySignalGroup": "D",
                    "alternateSignalGroup": "C",
                    "primarySignalGroupDuration": 30,
                },
            ],
        }
    )
    assert pl.cycle_time_seconds == 120
    assert pl.offset_seconds == 7
    assert len(pl.signal_group_timings) == 4
    assert pl.signal_group_timings[1].alternate_signal_group == "A"


def test_planline_extensions_round_trip():
    pl = PlanLine.model_validate(
        {
            "planNumber": 1,
            "planStartTime": "2026-04-22T10:00:00+00:00",
            "extensions": {"origin": "SCOOT"},
        }
    )
    assert pl.extensions == {"origin": "SCOOT"}


# ── TrafficSignalDynamicState ───────────────────────────────────────


def test_dynamic_state_minimal_valid():
    ds = TrafficSignalDynamicState.model_validate(
        {
            "trafficSignalId": "TS-001",
            "timestamp": "2026-04-22T10:00:00+00:00",
            "trafficSignalMode": "centralMode",
        }
    )
    assert ds.traffic_signal_id == "TS-001"
    assert ds.traffic_signal_mode is TrafficSignalModeEnum.CENTRAL_MODE
    assert ds.traffic_signal_sub_mode is None
    assert ds.plan_line is None
    assert ds.sub_area is None


def test_dynamic_state_mode_required():
    with pytest.raises(ValidationError):
        TrafficSignalDynamicState.model_validate(
            {"trafficSignalId": "TS-001", "timestamp": "2026-04-22T10:00:00+00:00"}
        )


def test_dynamic_state_full_payload():
    ds = TrafficSignalDynamicState.model_validate(
        {
            "trafficSignalId": "TS-001",
            "timestamp": "2026-04-22T10:00:00+00:00",
            "subArea": "Ring A",
            "trafficSignalMode": "centralMode",
            "trafficSignalSubMode": "utcScoot",
            "planLine": {
                "planNumber": 3,
                "planStartTime": "2026-04-22T09:30:00+00:00",
                "cycleTime": 120,
                "offset": 7,
                "signalGroupTimings": [
                    {"primarySignalGroup": "A", "primarySignalGroupDuration": 40},
                ],
            },
            "extensions": {"live": True},
        }
    )
    assert ds.traffic_signal_sub_mode is TrafficSignalSubModeEnum.UTC_SCOOT
    assert ds.plan_line.plan_number == 3
    assert ds.extensions == {"live": True}


def test_dynamic_state_extra_field_rejected():
    with pytest.raises(ValidationError):
        TrafficSignalDynamicState.model_validate(
            {
                "trafficSignalId": "TS-001",
                "timestamp": "2026-04-22T10:00:00+00:00",
                "trafficSignalMode": "centralMode",
                "stray": 1,
            }
        )
