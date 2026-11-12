# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-06
# Version:     1.1
# Description: The emitted JSON Schemas against the Python models: what Python
#              emits validates against the schema (no stray nulls), the
#              single-value detector fields are required in both, a version
#              without its id is invalid in both, and the rules that exist only
#              in Python are pinned so the gap is stated rather than assumed.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Schema and model must agree wherever the schema claims to speak."""

import functools
import json
from datetime import datetime, timezone
from pathlib import Path

import pytest
from jsonschema import Draft202012Validator
from pydantic import ValidationError
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT202012

from topas_3000_model.common.command import CommandResponseStatus
from topas_3000_model.detector.measurement import (
    FlowDynamic,
    HeadwayDynamic,
    OccupancyDynamic,
    QueueDynamic,
    SpeedDynamic,
)
from topas_3000_model.strategy.command import StrategyTrigger
from topas_3000_model.strategy.enums import TriggerStateEnum
from topas_3000_model.strategy.response import StrategyCommandResponse
from topas_3000_model.traffic_event.accident import AccidentEvent, AccidentType
from topas_3000_model.traffic_event.common import EventStatus, LanesAffectedType
from topas_3000_model.traffic_signal.fault import TrafficSignalFault

ROOT = Path(__file__).resolve().parent.parent
NOW = datetime(2026, 10, 3, 12, 0, tzinfo=timezone.utc)


@functools.cache
def _docs() -> dict[Path, dict]:
    """Every emitted schema, read on first use so collection does not need Schemas/."""
    paths = sorted((ROOT / "Schemas").rglob("*.json"))
    assert paths, "Schemas/ is empty: run `make schemas` first"
    return {p: json.loads(p.read_text()) for p in paths}


@functools.cache
def _registry() -> Registry:
    return Registry().with_resources(
        (d["$id"], Resource.from_contents(d, default_specification=DRAFT202012))
        for d in _docs().values()
    )


def _validator(filename: str) -> Draft202012Validator:
    doc = next(d for p, d in _docs().items() if p.name == filename)
    return Draft202012Validator(doc, registry=_registry())


def _valid(filename: str, payload: dict) -> bool:
    return _validator(filename).is_valid(payload)


def _emitted(model) -> dict:
    """What a Python publisher sends: the default JSON, with no special options."""
    return json.loads(model.model_dump_json())


# ── nothing Python emits breaks the schema ──


@pytest.mark.parametrize(
    "model",
    [
        FlowDynamic(detector_id="d", timestamp=NOW, total_flow=3),
        FlowDynamic(detector_id="d", timestamp=NOW, class_counts=[None, 2]),
        OccupancyDynamic(detector_id="d", timestamp=NOW, occupancy_percent=4.5),
        SpeedDynamic(detector_id="d", timestamp=NOW, mean_speed_kph=50.0),
        HeadwayDynamic(detector_id="d", timestamp=NOW, mean_headway_seconds=2.1),
        QueueDynamic(detector_id="d", timestamp=NOW, queue_present=False),
    ],
    ids=lambda m: m.kind,
)
def test_default_detector_json_validates_against_its_schema(model):
    """Unset optionals used to be emitted as null, which the schema rejects."""
    payload = _emitted(model)
    assert None not in [v for k, v in payload.items() if k != "classCounts"]
    assert _valid("detector_measurement-v1.0.json", payload), payload


def test_default_strategy_json_validates_against_its_schema():
    trigger = StrategyTrigger(
        strategy_id="S1", command_correlation_id=1, request_time=NOW,
        trigger_state=TriggerStateEnum.ENABLED,
    )
    assert _valid("strategy_command-v1.0.json", _emitted(trigger))
    response = StrategyCommandResponse(
        strategy_id="S1", command_correlation_id=1,
        status=CommandResponseStatus.SUCCESS, response_time=NOW,
    )
    assert _valid("strategy_command_response-v1.0.json", _emitted(response))


def test_default_traffic_event_json_validates_against_its_schema():
    event = AccidentEvent(
        event_id="e", accident_type=AccidentType.COLLISION, status=EventStatus.ACTIVE
    )
    payload = _emitted(event)
    assert "accidentTime" not in payload
    assert _valid("traffic_event_accident-v1.0.json", payload)


def test_null_class_count_entries_survive_serialisation():
    """Only a model's own unset fields are dropped; a null entry is data."""
    assert _emitted(FlowDynamic(detector_id="d", timestamp=NOW, class_counts=[None, 2]))[
        "classCounts"
    ] == [None, 2]


# ── rules stated in the schema agree with the model ──


@pytest.mark.parametrize("kind,field", [
    ("occupancy", "occupancyPercent"), ("speed", "meanSpeedKph"), ("headway", "meanHeadwaySeconds"),
])
def test_single_value_detector_fields_are_required_in_both(kind, field):
    base = {"kind": kind, "detectorId": "d", "timestamp": "2026-10-03T12:00:00Z"}
    assert not _valid("detector_measurement-v1.0.json", base)
    assert _valid("detector_measurement-v1.0.json", {**base, field: 1.0})
    with pytest.raises(ValidationError):
        {"occupancy": OccupancyDynamic, "speed": SpeedDynamic, "headway": HeadwayDynamic}[kind](
            detector_id="d", timestamp=NOW
        )


def _accident(**extra) -> dict:
    return {
        "eventId": "e", "accidentType": "collision", "status": "active", **extra,
    }


def test_version_without_its_id_is_invalid_in_both():
    payload = _accident(externalSituationVersion=3)
    assert not _valid("traffic_event_accident-v1.0.json", payload)
    with pytest.raises(ValidationError):
        AccidentEvent.model_validate(payload)
    ok = _accident(externalSituationVersion=3, externalSituationId="S-1")
    assert _valid("traffic_event_accident-v1.0.json", ok)
    AccidentEvent.model_validate(ok)


# ── the gaps: rules that exist only in Python, pinned so they are stated ──


def _signal_fault(**extra) -> dict:
    return {"trafficSignalId": "TS-1", "faultLastUpdateTime": "2026-10-06T09:00:00+01:00", **extra}


@pytest.mark.parametrize(
    ("extra", "accepted"),
    [
        ({"faultTypeId": "F3"}, True),
        ({"trafficSignalFaultType": "lampFault"}, True),
        ({"faultTypeId": "F3", "trafficSignalFaultType": "lampFault"}, True),
        ({}, False),
    ],
    ids=["catalogue-only", "2017-enum-only", "both", "neither"],
)
def test_signal_fault_needs_a_classification_in_both(extra, accepted):
    """At least one of faultTypeId and trafficSignalFaultType: the schema
    states it as anyOf, so this is not a Python-only rule."""
    payload = _signal_fault(**extra)
    assert _valid("traffic_signal_fault-v1.0.json", payload) is accepted
    try:
        TrafficSignalFault.model_validate(payload)
        python_accepts = True
    except ValidationError:
        python_accepts = False
    assert python_accepts is accepted


def test_gap_empty_flow_reading_is_schema_valid_but_python_invalid():
    """cargo-typify turns each anyOf branch into a type, so the rule is not emitted."""
    payload = {"kind": "flow", "detectorId": "d", "timestamp": "2026-10-03T12:00:00Z"}
    assert _valid("detector_measurement-v1.0.json", payload)
    with pytest.raises(ValidationError):
        FlowDynamic(detector_id="d", timestamp=NOW)


def test_gap_network_link_and_path_together_are_schema_valid_but_python_invalid():
    """The XSD's link-or-path choice is a Python-only rule."""
    from topas_3000_model.detector.definition import DetectorDefinition

    payload = {
        "detectorId": "d", "typeId": "1", "creationDate": "2026-10-03T12:00:00Z",
        "networkLinkId": "a", "networkPathId": "b",
    }
    assert _valid("detector_definition-v1.0.json", payload)
    with pytest.raises(ValidationError):
        DetectorDefinition.model_validate(payload)


def test_gap_backwards_time_is_schema_valid_but_python_invalid():
    payload = _accident(accidentTime="2026-10-03T12:00:00Z", endTime="2026-10-03T11:00:00Z")
    assert _valid("traffic_event_accident-v1.0.json", payload)
    with pytest.raises(ValidationError):
        AccidentEvent.model_validate(payload)


def test_gap_lane_counts_beyond_the_pattern_are_schema_valid_but_python_invalid():
    """cargo-typify 0.7.0 panics on if/then, so the lane rules are not emitted."""
    payload = _accident(lanesAffected=LanesAffectedType.TT.value, lanesRestricted=5)
    assert _valid("traffic_event_accident-v1.0.json", payload)
    with pytest.raises(ValidationError):
        AccidentEvent.model_validate(payload)
