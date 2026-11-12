# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: CCTV definition and configuration: minimal and full
#              construction, rejection of an empty id and negative height, and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.cctv.definition."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.cctv.definition import CctvConfiguration, CctvDefinition
from topas_3000_model.common.location import Coordinates, Point


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_cctv_definition_minimal():
    d = CctvDefinition(
        cctv_id="cam-001",
        type_id="1",
        creation_date=NOW,
    )
    assert d.cctv_id == "cam-001"
    assert d.location is None


def test_cctv_definition_with_all_fields():
    d = CctvDefinition(
        cctv_id="cam-001",
        type_id="1",
        creation_date=NOW,
        location=Point(coordinates=Coordinates(latitude=51.5, longitude=-0.1)),
        short_description="High Street Camera",
        long_description="PTZ camera on High Street junction",
        participant_id="sys-001",
        owner_operator="Council",
    )
    assert d.short_description == "High Street Camera"
    assert d.owner_operator == "Council"


def test_cctv_definition_rejects_empty_id():
    with pytest.raises(ValidationError):
        CctvDefinition(cctv_id="", type_id="1", creation_date=NOW)


def test_cctv_definition_dumps_camel_case():
    d = CctvDefinition(
        cctv_id="cam-001",
        type_id="1",
        creation_date=NOW,
        short_description="Test Camera",
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["cctvId"] == "cam-001"
    assert dumped["shortDescription"] == "Test Camera"


def test_cctv_configuration_minimal():
    c = CctvConfiguration(
        cctv_id="cam-001",
        configuration_date=NOW,
    )
    assert c.serial_number is None
    assert c.fixed is None


def test_cctv_configuration_with_all_fields():
    c = CctvConfiguration(
        cctv_id="cam-001",
        configuration_date=NOW,
        serial_number="SN-12345",
        camera_details="Axis P3245-V",
        fixed=True,
        camera_height_meters=5,
        camera_sub_site_id="sub-001",
    )
    assert c.serial_number == "SN-12345"
    assert c.fixed is True
    assert c.camera_height_meters == 5


def test_cctv_configuration_rejects_negative_height():
    with pytest.raises(ValidationError):
        CctvConfiguration(
            cctv_id="cam-001",
            configuration_date=NOW,
            camera_height_meters=-1,
        )


def test_cctv_configuration_dumps_camel_case():
    c = CctvConfiguration(
        cctv_id="cam-001",
        configuration_date=NOW,
        serial_number="SN-12345",
        camera_height_meters=5,
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["cctvId"] == "cam-001"
    assert dumped["serialNumber"] == "SN-12345"
    assert dumped["cameraHeightMeters"] == 5
