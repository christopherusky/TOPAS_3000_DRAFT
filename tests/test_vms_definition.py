# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: VMS definition and configuration: sign hardware, rejection of
#              an empty id and non-positive height, lines or characters per
#              line, brightness and timing configuration with range
#              enforcement, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.vms.definition."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.common.location import Coordinates, Point
from topas_3000_model.vms.definition import VmsConfiguration, VmsDefinition
from topas_3000_model.vms.enums import VmsColourCapability


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_vms_definition_minimal():
    d = VmsDefinition(
        vms_id="vms-001",
        type_id="1",
        creation_date=NOW,
    )
    assert d.vms_id == "vms-001"
    assert d.type_id == "1"
    assert d.location is None
    assert d.sign_height is None
    assert d.colour_capability is None


def test_vms_definition_with_sign_hardware():
    d = VmsDefinition(
        vms_id="vms-001",
        type_id="1",
        creation_date=NOW,
        location=Point(coordinates=Coordinates(latitude=51.5, longitude=-0.1)),
        short_description="M1 J25 Southbound",
        long_description="Variable message sign on M1 southbound approach to J25",
        sign_height=64,
        sign_width=192,
        number_of_lines=4,
        characters_per_line=20,
        supports_graphics=True,
        colour_capability=VmsColourCapability.FULL_COLOUR,
        owner_operator="National Highways",
        extensions={"firmware": "v3.2"},
    )
    assert d.sign_height == 64
    assert d.sign_width == 192
    assert d.number_of_lines == 4
    assert d.characters_per_line == 20
    assert d.supports_graphics is True
    assert d.colour_capability == VmsColourCapability.FULL_COLOUR
    assert d.owner_operator == "National Highways"


def test_vms_definition_rejects_empty_id():
    with pytest.raises(ValidationError):
        VmsDefinition(
            vms_id="",
            type_id="1",
            creation_date=NOW,
        )


def test_vms_definition_rejects_negative_height():
    with pytest.raises(ValidationError):
        VmsDefinition(
            vms_id="vms-001",
            type_id="1",
            creation_date=NOW,
            sign_height=-1,
        )


def test_vms_definition_rejects_zero_lines():
    with pytest.raises(ValidationError):
        VmsDefinition(
            vms_id="vms-001",
            type_id="1",
            creation_date=NOW,
            number_of_lines=0,
        )


def test_vms_definition_rejects_zero_characters_per_line():
    with pytest.raises(ValidationError):
        VmsDefinition(
            vms_id="vms-001",
            type_id="1",
            creation_date=NOW,
            characters_per_line=0,
        )


def test_vms_definition_dumps_camel_case():
    d = VmsDefinition(
        vms_id="vms-001",
        type_id="1",
        creation_date=NOW,
        sign_height=64,
        sign_width=192,
        colour_capability=VmsColourCapability.AMBER,
        owner_operator="Metro Council",
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["vmsId"] == "vms-001"
    assert dumped["typeId"] == "1"
    assert dumped["creationDate"] == "2026-04-16T10:00:00Z"
    assert dumped["signHeight"] == 64
    assert dumped["signWidth"] == 192
    assert dumped["colourCapability"] == "amber"
    assert dumped["ownerOperator"] == "Metro Council"


def test_vms_configuration_minimal():
    c = VmsConfiguration(
        vms_id="vms-001",
        configuration_date=NOW,
    )
    assert c.vms_id == "vms-001"
    assert c.default_brightness_percent is None
    assert c.auto_brightness is None


def test_vms_configuration_with_brightness_and_timing():
    c = VmsConfiguration(
        vms_id="vms-001",
        configuration_date=NOW,
        default_brightness_percent=75.0,
        auto_brightness=True,
        default_display_time_seconds=30,
        scroll_speed=5,
    )
    assert c.default_brightness_percent == 75.0
    assert c.auto_brightness is True
    assert c.default_display_time_seconds == 30
    assert c.scroll_speed == 5


def test_vms_configuration_rejects_brightness_over_100():
    with pytest.raises(ValidationError):
        VmsConfiguration(
            vms_id="vms-001",
            configuration_date=NOW,
            default_brightness_percent=101.0,
        )


def test_vms_configuration_rejects_negative_brightness():
    with pytest.raises(ValidationError):
        VmsConfiguration(
            vms_id="vms-001",
            configuration_date=NOW,
            default_brightness_percent=-1.0,
        )


def test_vms_configuration_rejects_zero_display_time():
    with pytest.raises(ValidationError):
        VmsConfiguration(
            vms_id="vms-001",
            configuration_date=NOW,
            default_display_time_seconds=0,
        )


def test_vms_configuration_dumps_camel_case():
    c = VmsConfiguration(
        vms_id="vms-001",
        configuration_date=NOW,
        default_brightness_percent=80.0,
        auto_brightness=False,
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["vmsId"] == "vms-001"
    assert dumped["configurationDate"] == "2026-04-16T10:00:00Z"
    assert dumped["defaultBrightnessPercent"] == 80.0
    assert dumped["autoBrightness"] is False
