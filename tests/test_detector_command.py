# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.4
# Description: Detector commands: reset, calibrate and setThreshold
#              construction, enforcement that both thresholds are present and
#              non-negative, discriminator dispatch, and rejection of unknown
#              kinds and negative correlation ids.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.detector.command."""

from datetime import datetime, timezone

import pytest
from pydantic import TypeAdapter, ValidationError

from topas_3000_model.detector.command import (
    CalibrateDetectorCommand,
    DetectorCommand,
    ResetDetectorCommand,
    SetThresholdDetectorCommand,
)


NOW = datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc)
_TA = TypeAdapter(DetectorCommand)


def test_reset_minimal():
    c = ResetDetectorCommand(
        detector_id="det-001",
        command_correlation_id=1,
        request_time=NOW,
    )
    assert c.kind == "reset"
    assert c.detector_id == "det-001"


def test_calibrate_minimal():
    c = CalibrateDetectorCommand(
        detector_id="det-002",
        command_correlation_id=2,
        request_time=NOW,
    )
    assert c.kind == "calibrate"


def test_set_threshold_with_values():
    c = SetThresholdDetectorCommand(
        detector_id="det-003",
        command_correlation_id=3,
        request_time=NOW,
        threshold_up=80,
        threshold_down=20,
    )
    assert c.kind == "setThreshold"
    assert c.threshold_up == 80
    assert c.threshold_down == 20


def test_set_threshold_requires_both_thresholds():
    """threshold_up and threshold_down are required — omitting them must fail."""
    with pytest.raises(ValidationError):
        SetThresholdDetectorCommand(
            detector_id="det-003",
            command_correlation_id=3,
            request_time=NOW,
        )


def test_set_threshold_rejects_negative():
    with pytest.raises(ValidationError):
        SetThresholdDetectorCommand(
            detector_id="det-003",
            command_correlation_id=3,
            request_time=NOW,
            threshold_up=-1,
        )


def test_discriminator_routes_reset():
    obj = _TA.validate_python({
        "kind": "reset",
        "detector_id": "det-001",
        "command_correlation_id": 1,
        "request_time": NOW,
    })
    assert isinstance(obj, ResetDetectorCommand)


def test_discriminator_routes_calibrate():
    obj = _TA.validate_python({
        "kind": "calibrate",
        "detector_id": "det-001",
        "command_correlation_id": 2,
        "request_time": NOW,
    })
    assert isinstance(obj, CalibrateDetectorCommand)


def test_discriminator_routes_set_threshold():
    obj = _TA.validate_python({
        "kind": "setThreshold",
        "detector_id": "det-001",
        "command_correlation_id": 3,
        "request_time": NOW,
        "threshold_up": 50,
        "threshold_down": 10,
    })
    assert isinstance(obj, SetThresholdDetectorCommand)
    assert obj.threshold_up == 50
    assert obj.threshold_down == 10


def test_discriminator_rejects_unknown_kind():
    with pytest.raises(ValidationError):
        _TA.validate_python({
            "kind": "teleport",
            "detector_id": "det-001",
            "command_correlation_id": 1,
            "request_time": NOW,
        })


def test_reset_dumps_camel_case():
    c = ResetDetectorCommand(
        detector_id="det-001",
        command_correlation_id=1,
        request_time=NOW,
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert "detectorId" in dumped
    assert "commandCorrelationId" in dumped
    assert "requestTime" in dumped
    assert "kind" in dumped
    assert dumped["kind"] == "reset"


def test_rejects_negative_command_correlation_id():
    with pytest.raises(ValidationError):
        ResetDetectorCommand(
            detector_id="det-001",
            command_correlation_id=-1,
            request_time=NOW,
        )
