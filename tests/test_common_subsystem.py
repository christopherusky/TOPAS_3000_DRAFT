# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: SubsystemDefinition: minimal and full construction, rejection
#              of a missing description, extra fields and an invalid URL, and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.common.subsystem."""

import pytest
from pydantic import ValidationError

from topas_3000_model.common.subsystem import SubsystemDefinition


def test_subsystem_minimal():
    s = SubsystemDefinition(type_id="1000", description="MOTTS Traffic Controller")
    assert s.type_id == "1000"
    assert s.description == "MOTTS Traffic Controller"
    assert s.notes is None
    assert s.specification_link is None
    assert s.extensions is None


def test_subsystem_full():
    s = SubsystemDefinition(
        type_id="1100",
        description="PEEK Signal Controller",
        notes="Supports UTC protocol",
        specification_link="https://example.invalid/peek-spec",
        extensions={"vendor": "PEEK"},
    )
    assert s.notes == "Supports UTC protocol"
    assert str(s.specification_link) == "https://example.invalid/peek-spec"
    assert s.extensions == {"vendor": "PEEK"}


def test_subsystem_rejects_missing_description():
    with pytest.raises(ValidationError):
        SubsystemDefinition(type_id="1000")


def test_subsystem_rejects_extra_fields():
    with pytest.raises(ValidationError):
        SubsystemDefinition(type_id="1000", description="Test", unknown="bad")


def test_subsystem_invalid_url():
    with pytest.raises(ValidationError):
        SubsystemDefinition(type_id="1000", description="Test", specification_link="not a url")


def test_subsystem_dumps_camel_case():
    s = SubsystemDefinition(type_id="1200", description="TENET Controller")
    dumped = s.model_dump(by_alias=True, exclude_none=True)
    assert dumped == {"typeId": "1200", "description": "TENET Controller"}
