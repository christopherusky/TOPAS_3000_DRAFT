# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.2
# Description: Detector definition and configuration: minimal construction,
#              location, thresholds, empty-id rejection, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.detector.definition."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.common.location import Coordinates, Point
from topas_3000_model.detector.definition import DetectorConfiguration, DetectorDefinition


def test_detector_definition_minimal():
    d = DetectorDefinition(
        detector_id="det-001",
        type_id="1",
        creation_date=datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc),
    )
    assert d.detector_id == "det-001"
    assert d.type_id == "1"
    assert d.location is None


def test_detector_definition_with_location():
    d = DetectorDefinition(
        detector_id="det-001",
        type_id="1",
        creation_date=datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc),
        location=Point(coordinates=Coordinates(latitude=51.5, longitude=-0.1)),
        short_description="A1 southbound",
    )
    assert d.location.coordinates.latitude == 51.5


def test_detector_definition_rejects_empty_id():
    with pytest.raises(ValidationError):
        DetectorDefinition(
            detector_id="",
            type_id="1",
            creation_date=datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc),
        )


def test_detector_configuration_minimal():
    c = DetectorConfiguration(
        detector_id="det-001",
        configuration_date=datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc),
    )
    assert c.threshold_up is None
    assert c.threshold_down is None


def test_detector_configuration_with_thresholds():
    c = DetectorConfiguration(
        detector_id="det-001",
        configuration_date=datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc),
        threshold_up=80,
        threshold_down=20,
    )
    assert c.threshold_up == 80
    assert c.threshold_down == 20


def test_detector_definition_dumps_camel_case():
    d = DetectorDefinition(
        detector_id="det-001",
        type_id="1",
        creation_date=datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc),
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert "detectorId" in dumped
    assert "typeId" in dumped
    assert "creationDate" in dumped
