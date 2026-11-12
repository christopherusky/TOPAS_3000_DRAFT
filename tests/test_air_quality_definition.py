# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Air-quality definition and configuration: minimal and full
#              construction, empty-id and negative-threshold rejection, and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.air_quality.definition."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.air_quality.definition import (
    AirQualityConfiguration,
    AirQualityDefinition,
)
from topas_3000_model.common.location import Coordinates, Point


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_air_quality_definition_minimal():
    d = AirQualityDefinition(
        air_quality_id="aq-001",
        type_id="1",
        creation_date=NOW,
    )
    assert d.air_quality_id == "aq-001"
    assert d.measurement_units is None


def test_air_quality_definition_with_all_fields():
    d = AirQualityDefinition(
        air_quality_id="aq-001",
        type_id="1",
        creation_date=NOW,
        location=Point(coordinates=Coordinates(latitude=51.5, longitude=-0.1)),
        short_description="City Centre AQ Monitor",
        long_description="DEFRA reference-grade monitor on High Street",
        measurement_units="ug/m3",
        participant_id="sys-001",
        owner_operator="Environment Agency",
    )
    assert d.measurement_units == "ug/m3"
    assert d.owner_operator == "Environment Agency"


def test_air_quality_definition_rejects_empty_id():
    with pytest.raises(ValidationError):
        AirQualityDefinition(air_quality_id="", type_id="1", creation_date=NOW)


def test_air_quality_definition_dumps_camel_case():
    d = AirQualityDefinition(
        air_quality_id="aq-001",
        type_id="1",
        creation_date=NOW,
        measurement_units="ug/m3",
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["airQualityId"] == "aq-001"
    assert dumped["measurementUnits"] == "ug/m3"


def test_air_quality_configuration_minimal():
    c = AirQualityConfiguration(
        air_quality_id="aq-001",
        configuration_date=NOW,
    )
    assert c.co_threshold is None
    assert c.pm10_threshold is None


def test_air_quality_configuration_with_thresholds():
    c = AirQualityConfiguration(
        air_quality_id="aq-001",
        configuration_date=NOW,
        no2_threshold=40.0,
        pm10_threshold=50.0,
        ozone_threshold=100.0,
    )
    assert c.no2_threshold == 40.0
    assert c.pm10_threshold == 50.0


def test_air_quality_configuration_rejects_negative_threshold():
    with pytest.raises(ValidationError):
        AirQualityConfiguration(
            air_quality_id="aq-001",
            configuration_date=NOW,
            no2_threshold=-1.0,
        )


def test_air_quality_configuration_dumps_camel_case():
    c = AirQualityConfiguration(
        air_quality_id="aq-001",
        configuration_date=NOW,
        no2_threshold=40.0,
        pm10_threshold=50.0,
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["airQualityId"] == "aq-001"
    assert dumped["no2Threshold"] == 40.0
    assert dumped["pm10Threshold"] == 50.0
