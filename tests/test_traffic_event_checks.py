# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.0
# Description: Traffic event restorations and shared consistency checks:
#              optional accident and incident times, end not before start,
#              external version needs an id, NMTOKEN country code, and lane
#              counts that fit the lane pattern, across all four subtypes.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.traffic_event.checks and the time restoration."""

from datetime import datetime, timedelta, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.traffic_event.accident import AccidentEvent, AccidentType
from topas_3000_model.traffic_event.common import EventStatus, LanesAffectedType
from topas_3000_model.traffic_event.event import PlannedEvent, PlannedEventType
from topas_3000_model.traffic_event.incident import IncidentEvent, IncidentType
from topas_3000_model.traffic_event.roadworks import RoadworksEvent, RoadworksType

T0 = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)
T1 = T0 + timedelta(hours=2)

# Each factory builds the minimal valid event of one subtype.
_FACTORIES = {
    "accident": lambda **kw: AccidentEvent(
        event_id="e", accident_type=AccidentType.COLLISION, status=EventStatus.ACTIVE, **kw
    ),
    "incident": lambda **kw: IncidentEvent(
        event_id="e", incident_type=IncidentType.CONGESTION, status=EventStatus.ACTIVE, **kw
    ),
    "roadworks": lambda **kw: RoadworksEvent(
        event_id="e", roadworks_type=RoadworksType.SURFACE, status=EventStatus.PLANNED, **kw
    ),
    "event": lambda **kw: PlannedEvent(
        event_id="e", planned_event_type=PlannedEventType.FOOTBALL, status=EventStatus.PLANNED, **kw
    ),
}
ALL = list(_FACTORIES)

# (subtype, start field, end field)
_TIME_CASES = [
    ("accident", "accident_time", "end_time"),
    ("incident", "incident_time", "end_time"),
    ("roadworks", "planned_start", "planned_end"),
    ("roadworks", "actual_start", "actual_end"),
    ("event", "planned_start", "planned_end"),
    ("event", "actual_start", "actual_end"),
]


# ── report times are optional ──


def test_accident_without_a_report_time_is_valid():
    e = _FACTORIES["accident"]()
    assert e.accident_time is None


def test_incident_without_a_report_time_is_valid():
    e = _FACTORIES["incident"]()
    assert e.incident_time is None


def test_accident_end_time_without_a_start_is_valid():
    """The ordering rule only fires when both ends are present."""
    assert _FACTORIES["accident"](end_time=T1).accident_time is None


# ── time ordering ──


@pytest.mark.parametrize("subtype,start,end", _TIME_CASES)
def test_end_before_start_is_rejected(subtype, start, end):
    with pytest.raises(ValidationError):
        _FACTORIES[subtype](**{start: T1, end: T0})


@pytest.mark.parametrize("subtype,start,end", _TIME_CASES)
def test_end_equal_to_or_after_start_is_accepted(subtype, start, end):
    _FACTORIES[subtype](**{start: T0, end: T0})
    _FACTORIES[subtype](**{start: T0, end: T1})


# ── external cross-reference ──


@pytest.mark.parametrize("subtype", ALL)
def test_external_version_requires_an_id(subtype):
    with pytest.raises(ValidationError):
        _FACTORIES[subtype](external_situation_version=3)
    e = _FACTORIES[subtype](external_situation_id="S-1", external_situation_version=3)
    assert e.external_situation_version == 3


@pytest.mark.parametrize("subtype", ALL)
def test_external_version_cannot_be_negative(subtype):
    with pytest.raises(ValidationError):
        _FACTORIES[subtype](external_situation_id="S-1", external_situation_version=-3)


@pytest.mark.parametrize("subtype", ALL)
def test_external_country_code_is_an_nmtoken(subtype):
    assert _FACTORIES[subtype](external_country_code="GB").external_country_code == "GB"
    for bad in ("not a code!!", "", "G B"):
        with pytest.raises(ValidationError):
            _FACTORIES[subtype](external_country_code=bad)


# ── lane counts ──


@pytest.mark.parametrize("subtype", ALL)
def test_lane_counts_cannot_exceed_the_lane_pattern(subtype):
    with pytest.raises(ValidationError):
        _FACTORIES[subtype](
            lanes_affected=LanesAffectedType.TT, lanes_restricted=5, lanes_operational=7
        )
    with pytest.raises(ValidationError):
        _FACTORIES[subtype](lanes_affected=LanesAffectedType.TT, lanes_operational=3)


@pytest.mark.parametrize("subtype", ALL)
def test_lane_counts_within_the_pattern_are_accepted(subtype):
    e = _FACTORIES[subtype](
        lanes_affected=LanesAffectedType.ITT, lanes_restricted=2, lanes_operational=1
    )
    assert e.lanes_restricted == 2


@pytest.mark.parametrize("subtype", ALL)
def test_all_open_cannot_have_restricted_lanes(subtype):
    with pytest.raises(ValidationError):
        _FACTORIES[subtype](lanes_affected=LanesAffectedType.ALL_OPEN, lanes_restricted=1)
    _FACTORIES[subtype](lanes_affected=LanesAffectedType.ALL_OPEN, lanes_restricted=0)


@pytest.mark.parametrize("subtype", ALL)
def test_lane_counts_without_a_pattern_are_not_checked(subtype):
    """Nothing to compare against, so any non-negative count stands."""
    assert _FACTORIES[subtype](lanes_restricted=9).lanes_restricted == 9


# ── deliberately not checked ──


@pytest.mark.parametrize("subtype", ["accident", "incident"])
def test_cleared_event_without_an_end_time_is_accepted(subtype):
    """The clearance time is often unknown; requiring it would reject that report."""
    e = _FACTORIES[subtype]()
    cleared = type(e)(**{**e.model_dump(), "status": EventStatus.CLEARED})
    assert cleared.end_time is None


@pytest.mark.parametrize("subtype", ALL)
def test_external_version_needs_a_non_empty_id(subtype):
    """An empty id satisfied the old presence check without naming any situation."""
    with pytest.raises(ValidationError):
        _FACTORIES[subtype](external_situation_id="", external_situation_version=3)


# ── boundaries (mutation checks: `>` must not become `>=`) ──


@pytest.mark.parametrize("subtype", ALL)
@pytest.mark.parametrize(
    "pattern,lanes",
    [(LanesAffectedType.I, 1), (LanesAffectedType.TT, 2), (LanesAffectedType.ITT, 3), (LanesAffectedType.IITT, 4)],
)
def test_lane_counts_equal_to_the_lane_count_are_accepted_and_one_more_is_not(subtype, pattern, lanes):
    for field in ("lanes_restricted", "lanes_operational"):
        assert getattr(_FACTORIES[subtype](lanes_affected=pattern, **{field: lanes}), field) == lanes
        with pytest.raises(ValidationError):
            _FACTORIES[subtype](lanes_affected=pattern, **{field: lanes + 1})


# ── mixed UTC offsets ──


def test_end_before_start_is_judged_on_the_instant_not_the_wall_clock():
    """12:00+01:00 is 11:00Z, which is earlier than 11:30Z; wall-clock digits mislead."""
    plus_one = timezone(timedelta(hours=1))
    start = datetime(2026, 4, 16, 11, 30, tzinfo=timezone.utc)
    ok_end = datetime(2026, 4, 16, 13, 0, tzinfo=plus_one)  # 12:00Z, after the start
    bad_end = datetime(2026, 4, 16, 12, 0, tzinfo=plus_one)  # 11:00Z, before the start
    _FACTORIES["accident"](accident_time=start, end_time=ok_end)
    with pytest.raises(ValidationError):
        _FACTORIES["accident"](accident_time=start, end_time=bad_end)
