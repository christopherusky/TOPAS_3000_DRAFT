# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.2
# Description: Car-park definition, configuration and detector models: minimal
#              and populated construction, rejection of empty ids and negative
#              entrances, exits and capacity, thresholds, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.car_park.definition."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.car_park.definition import (
    CarParkConfiguration,
    CarParkDefinition,
    CarParkDetectorsConfiguration,
)
from topas_3000_model.common.location import Coordinates, Point


NOW = datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc)


def test_car_park_definition_minimal():
    d = CarParkDefinition(
        car_park_id="cp-001",
        type_id="1",
        creation_date=NOW,
    )
    assert d.car_park_id == "cp-001"
    assert d.type_id == "1"
    assert d.location is None
    assert d.owner_operator is None
    assert d.number_of_entrances is None


def test_car_park_definition_with_basic_data():
    d = CarParkDefinition(
        car_park_id="cp-001",
        type_id="1",
        creation_date=NOW,
        location=Point(coordinates=Coordinates(latitude=51.5, longitude=-0.1)),
        short_description="City Centre Multi-Storey",
        owner_operator="Metro Council",
        car_park_user_type="pay on foot",
        number_of_entrances=2,
        number_of_exits=2,
        toilets_available=True,
        disabled_toilets_available=True,
    )
    assert d.location.coordinates.latitude == 51.5
    assert d.owner_operator == "Metro Council"
    assert d.car_park_user_type == "pay on foot"
    assert d.number_of_entrances == 2
    assert d.number_of_exits == 2
    assert d.toilets_available is True
    assert d.disabled_toilets_available is True


def test_car_park_definition_rejects_empty_id():
    with pytest.raises(ValidationError):
        CarParkDefinition(
            car_park_id="",
            type_id="1",
            creation_date=NOW,
        )


def test_car_park_definition_rejects_negative_entrances():
    with pytest.raises(ValidationError):
        CarParkDefinition(
            car_park_id="cp-001",
            type_id="1",
            creation_date=NOW,
            number_of_entrances=-1,
        )


def test_car_park_definition_rejects_negative_exits():
    with pytest.raises(ValidationError):
        CarParkDefinition(
            car_park_id="cp-001",
            type_id="1",
            creation_date=NOW,
            number_of_exits=-1,
        )


def test_car_park_definition_dumps_camel_case():
    d = CarParkDefinition(
        car_park_id="cp-001",
        type_id="1",
        creation_date=NOW,
        owner_operator="Metro Council",
        car_park_user_type="pay on foot",
        number_of_entrances=2,
        number_of_exits=2,
        toilets_available=True,
        disabled_toilets_available=False,
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["carParkId"] == "cp-001"
    assert dumped["typeId"] == "1"
    assert dumped["creationDate"] == "2026-04-11T10:00:00Z"
    assert dumped["ownerOperator"] == "Metro Council"
    assert dumped["carParkUserType"] == "pay on foot"
    assert dumped["numberOfEntrances"] == 2
    assert dumped["numberOfExits"] == 2
    assert dumped["toiletsAvailable"] is True
    assert dumped["disabledToiletsAvailable"] is False


def test_car_park_configuration_minimal():
    c = CarParkConfiguration(
        car_park_id="cp-001",
        configuration_date=NOW,
    )
    assert c.car_park_id == "cp-001"
    assert c.total_capacity is None
    assert c.disabled_capacity is None
    assert c.almost_full_increasing_threshold is None
    assert c.almost_full_decreasing_threshold is None
    assert c.full_increasing_threshold is None
    assert c.full_decreasing_threshold is None
    assert c.entrance_full_threshold is None


def test_car_park_configuration_with_thresholds():
    c = CarParkConfiguration(
        car_park_id="cp-001",
        configuration_date=NOW,
        total_capacity=500,
        disabled_capacity=20,
        almost_full_increasing_threshold=450,
        almost_full_decreasing_threshold=420,
        full_increasing_threshold=495,
        full_decreasing_threshold=480,
        entrance_full_threshold=500,
    )
    assert c.total_capacity == 500
    assert c.disabled_capacity == 20
    assert c.almost_full_increasing_threshold == 450
    assert c.full_increasing_threshold == 495


def test_car_park_configuration_rejects_negative_capacity():
    with pytest.raises(ValidationError):
        CarParkConfiguration(
            car_park_id="cp-001",
            configuration_date=NOW,
            total_capacity=-1,
        )


def test_car_park_configuration_dumps_camel_case():
    c = CarParkConfiguration(
        car_park_id="cp-001",
        configuration_date=NOW,
        total_capacity=500,
        disabled_capacity=20,
        almost_full_increasing_threshold=450,
        full_increasing_threshold=495,
        entrance_full_threshold=500,
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["carParkId"] == "cp-001"
    assert dumped["configurationDate"] == "2026-04-11T10:00:00Z"
    assert dumped["totalCapacity"] == 500
    assert dumped["disabledCapacity"] == 20
    assert dumped["almostFullIncreasingThreshold"] == 450
    assert dumped["fullIncreasingThreshold"] == 495
    assert dumped["entranceFullThreshold"] == 500


def test_car_park_detectors_configuration_single_detector():
    c = CarParkDetectorsConfiguration(
        car_park_id="cp-001",
        configuration_date=NOW,
        detector_ids=["det-001"],
    )
    assert c.car_park_id == "cp-001"
    assert c.detector_ids == ["det-001"]


def test_car_park_detectors_configuration_multiple_detectors():
    c = CarParkDetectorsConfiguration(
        car_park_id="cp-001",
        configuration_date=NOW,
        detector_ids=["det-001", "det-002", "det-003"],
    )
    assert len(c.detector_ids) == 3


def test_car_park_detectors_configuration_rejects_empty_list():
    with pytest.raises(ValidationError):
        CarParkDetectorsConfiguration(
            car_park_id="cp-001",
            configuration_date=NOW,
            detector_ids=[],
        )


def test_car_park_detectors_configuration_rejects_missing_detector_ids():
    with pytest.raises(ValidationError):
        CarParkDetectorsConfiguration(
            car_park_id="cp-001",
            configuration_date=NOW,
        )


def test_car_park_detectors_configuration_dumps_camel_case():
    c = CarParkDetectorsConfiguration(
        car_park_id="cp-001",
        configuration_date=NOW,
        detector_ids=["det-001", "det-002"],
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["carParkId"] == "cp-001"
    assert dumped["detectorIds"] == ["det-001", "det-002"]
