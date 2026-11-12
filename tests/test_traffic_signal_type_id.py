# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-27
# Version:     1.3
# Description: Traffic-signal type identifier: minimal and full construction,
#              rejection of a missing description and an invalid URL, and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.traffic_signal.type_id."""

import pytest
from pydantic import ValidationError

from topas_3000_model.traffic_signal.type_id import TrafficSignalType


def test_traffic_signal_type_id_minimal():
    t = TrafficSignalType(type_id="1", description="MOVA")
    assert t.type_id == "1"
    assert t.description == "MOVA"
    assert t.notes is None
    assert t.specification_link is None


def test_traffic_signal_type_id_full():
    t = TrafficSignalType(
        type_id="2",
        description="SCOOT",
        notes="Adaptive urban traffic control",
        specification_link="https://example.invalid/scoot-spec",
    )
    assert t.notes == "Adaptive urban traffic control"


def test_traffic_signal_type_id_rejects_missing_description():
    with pytest.raises(ValidationError):
        TrafficSignalType(type_id="1")


def test_traffic_signal_type_id_invalid_url():
    with pytest.raises(ValidationError):
        TrafficSignalType(
            type_id="1",
            description="SCOOT",
            specification_link="not a url",
        )


def test_traffic_signal_type_id_dumps_camel_case():
    t = TrafficSignalType(type_id="1", description="MOVA")
    dumped = t.model_dump(by_alias=True, exclude_none=True)
    assert dumped == {"typeId": "1", "description": "MOVA"}
