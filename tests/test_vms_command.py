# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: VMS commands: displayMessage with lines, displayGraphic,
#              setBrightness and clearSign, discriminator dispatch, unknown-
#              kind rejection, and brightness range enforcement.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.vms.command."""

from datetime import datetime, timezone

import pytest
from pydantic import TypeAdapter, ValidationError

from topas_3000_model.vms.command import (
    ClearSignCommand,
    DisplayGraphicCommand,
    DisplayMessageCommand,
    SetBrightnessCommand,
    VmsCommand,
)


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)
_TA = TypeAdapter(VmsCommand)


def test_display_message_command():
    c = DisplayMessageCommand(
        vms_id="vms-001",
        command_correlation_id=1,
        request_time=NOW,
        message="ROAD CLOSED AHEAD",
        display_duration_seconds=60,
    )
    assert c.kind == "displayMessage"
    assert c.message == "ROAD CLOSED AHEAD"
    assert c.display_duration_seconds == 60


def test_display_message_with_lines():
    c = DisplayMessageCommand(
        vms_id="vms-001",
        command_correlation_id=1,
        request_time=NOW,
        message="ROAD CLOSED\nAHEAD",
        message_lines=["ROAD CLOSED", "AHEAD"],
    )
    assert c.message_lines == ["ROAD CLOSED", "AHEAD"]


def test_display_graphic_command():
    c = DisplayGraphicCommand(
        vms_id="vms-001",
        command_correlation_id=2,
        request_time=NOW,
        graphic_url="https://signs.example.com/diversion.png",
        display_duration_seconds=120,
    )
    assert c.kind == "displayGraphic"
    assert str(c.graphic_url) == "https://signs.example.com/diversion.png"


def test_set_brightness_command():
    c = SetBrightnessCommand(
        vms_id="vms-001",
        command_correlation_id=3,
        request_time=NOW,
        brightness_percent=75.0,
    )
    assert c.kind == "setBrightness"
    assert c.brightness_percent == 75.0


def test_clear_sign_command():
    c = ClearSignCommand(
        vms_id="vms-001",
        command_correlation_id=4,
        request_time=NOW,
    )
    assert c.kind == "clearSign"


def test_discriminator_routes_display_message():
    obj = _TA.validate_python({
        "kind": "displayMessage",
        "vmsId": "vms-001",
        "commandCorrelationId": 1,
        "requestTime": NOW,
        "message": "TEST",
    })
    assert isinstance(obj, DisplayMessageCommand)


def test_discriminator_routes_display_graphic():
    obj = _TA.validate_python({
        "kind": "displayGraphic",
        "vmsId": "vms-001",
        "commandCorrelationId": 2,
        "requestTime": NOW,
        "graphicUrl": "https://example.com/img.png",
    })
    assert isinstance(obj, DisplayGraphicCommand)


def test_discriminator_routes_set_brightness():
    obj = _TA.validate_python({
        "kind": "setBrightness",
        "vmsId": "vms-001",
        "commandCorrelationId": 3,
        "requestTime": NOW,
        "brightnessPercent": 50.0,
    })
    assert isinstance(obj, SetBrightnessCommand)


def test_discriminator_routes_clear_sign():
    obj = _TA.validate_python({
        "kind": "clearSign",
        "vmsId": "vms-001",
        "commandCorrelationId": 4,
        "requestTime": NOW,
    })
    assert isinstance(obj, ClearSignCommand)


def test_discriminator_rejects_unknown_kind():
    with pytest.raises(ValidationError):
        _TA.validate_python({
            "kind": "flashWarning",
            "vmsId": "vms-001",
            "commandCorrelationId": 1,
            "requestTime": NOW,
        })


def test_set_brightness_rejects_over_100():
    with pytest.raises(ValidationError):
        SetBrightnessCommand(
            vms_id="vms-001",
            command_correlation_id=1,
            request_time=NOW,
            brightness_percent=101.0,
        )


def test_set_brightness_rejects_negative():
    with pytest.raises(ValidationError):
        SetBrightnessCommand(
            vms_id="vms-001",
            command_correlation_id=1,
            request_time=NOW,
            brightness_percent=-1.0,
        )


def test_display_message_rejects_zero_duration():
    with pytest.raises(ValidationError):
        DisplayMessageCommand(
            vms_id="vms-001",
            command_correlation_id=1,
            request_time=NOW,
            message="TEST",
            display_duration_seconds=0,
        )


def test_rejects_negative_command_correlation_id():
    with pytest.raises(ValidationError):
        ClearSignCommand(
            vms_id="vms-001",
            command_correlation_id=-1,
            request_time=NOW,
        )


def test_display_message_dumps_camel_case():
    c = DisplayMessageCommand(
        vms_id="vms-001",
        command_correlation_id=1,
        request_time=NOW,
        message="ROAD CLOSED",
        display_duration_seconds=60,
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert "vmsId" in dumped
    assert "commandCorrelationId" in dumped
    assert "requestTime" in dumped
    assert "kind" in dumped
    assert dumped["kind"] == "displayMessage"
    assert dumped["message"] == "ROAD CLOSED"
    assert dumped["displayDurationSeconds"] == 60


def test_set_brightness_dumps_camel_case():
    c = SetBrightnessCommand(
        vms_id="vms-001",
        command_correlation_id=3,
        request_time=NOW,
        brightness_percent=75.0,
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["brightnessPercent"] == 75.0
    assert dumped["kind"] == "setBrightness"
