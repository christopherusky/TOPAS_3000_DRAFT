# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: VMS dynamic measurements: current message, graphic URL, status
#              enum and brightness range enforcement, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.vms.measurement."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.vms.enums import VmsStatus
from topas_3000_model.vms.measurement import VmsDynamic


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_vms_dynamic_minimal():
    d = VmsDynamic(
        vms_id="vms-001",
        timestamp=NOW,
    )
    assert d.vms_id == "vms-001"
    assert d.current_message is None
    assert d.status is None
    assert d.graphic_url is None


def test_vms_dynamic_with_current_message():
    d = VmsDynamic(
        vms_id="vms-001",
        timestamp=NOW,
        current_message="ROAD CLOSED AHEAD",
        message_lines=["ROAD CLOSED", "AHEAD"],
        brightness_percent=80.0,
        message_set_time=NOW,
        status=VmsStatus.DISPLAYING,
    )
    assert d.current_message == "ROAD CLOSED AHEAD"
    assert d.message_lines == ["ROAD CLOSED", "AHEAD"]
    assert d.brightness_percent == 80.0
    assert d.status == VmsStatus.DISPLAYING


def test_vms_dynamic_with_graphic_url():
    d = VmsDynamic(
        vms_id="vms-001",
        timestamp=NOW,
        graphic_url="https://images.example.com/signs/diversion.png",
        status=VmsStatus.DISPLAYING,
    )
    assert str(d.graphic_url) == "https://images.example.com/signs/diversion.png"


def test_vms_dynamic_with_status_enum():
    for status in VmsStatus:
        d = VmsDynamic(
            vms_id="vms-001",
            timestamp=NOW,
            status=status,
        )
        assert d.status == status


def test_vms_dynamic_rejects_brightness_over_100():
    with pytest.raises(ValidationError):
        VmsDynamic(
            vms_id="vms-001",
            timestamp=NOW,
            brightness_percent=101.0,
        )


def test_vms_dynamic_rejects_negative_brightness():
    with pytest.raises(ValidationError):
        VmsDynamic(
            vms_id="vms-001",
            timestamp=NOW,
            brightness_percent=-1.0,
        )


def test_vms_dynamic_dumps_camel_case():
    d = VmsDynamic(
        vms_id="vms-001",
        timestamp=NOW,
        current_message="CONGESTION",
        brightness_percent=50.0,
        status=VmsStatus.DISPLAYING,
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["vmsId"] == "vms-001"
    assert dumped["currentMessage"] == "CONGESTION"
    assert dumped["brightnessPercent"] == 50.0
    assert dumped["status"] == "displaying"
