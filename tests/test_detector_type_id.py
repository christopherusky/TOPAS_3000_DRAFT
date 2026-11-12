# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.2
# Description: Detector type identifier: minimal and full construction,
#              rejection of a missing description and an invalid URL, and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.detector.type_id."""

import pytest
from pydantic import ValidationError

from topas_3000_model.detector.type_id import DetectorType


def test_detector_type_id_minimal():
    t = DetectorType(type_id="1", description="Inductive loop")
    assert t.type_id == "1"
    assert t.description == "Inductive loop"
    assert t.notes is None
    assert t.specification_link is None


def test_detector_type_id_full():
    t = DetectorType(
        type_id="2",
        description="Radar",
        notes="Side-fire",
        specification_link="https://example.invalid/radar-spec",
    )
    assert t.notes == "Side-fire"


def test_detector_type_id_rejects_missing_description():
    with pytest.raises(ValidationError):
        DetectorType(type_id="1")


def test_detector_type_id_invalid_url():
    with pytest.raises(ValidationError):
        DetectorType(type_id="1", description="Radar", specification_link="not a url")


def test_detector_type_id_dumps_camel_case():
    t = DetectorType(type_id="1", description="Inductive loop")
    dumped = t.model_dump(by_alias=True, exclude_none=True)
    assert dumped == {"typeId": "1", "description": "Inductive loop"}
