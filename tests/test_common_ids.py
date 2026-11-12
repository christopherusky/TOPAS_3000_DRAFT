# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.1
# Description: Shared identifier types: ObjectId length and empty-string
#              constraints, ParticipantId, the integer QualityId, and the
#              constrained-string TypeId.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.common.ids."""

import pytest
from pydantic import TypeAdapter, ValidationError

from topas_3000_model.common.ids import ObjectId, ParticipantId, QualityId, TypeId


def test_object_id_valid():
    adapter = TypeAdapter(ObjectId)
    assert adapter.validate_python("detector-001") == "detector-001"


def test_object_id_too_long():
    adapter = TypeAdapter(ObjectId)
    with pytest.raises(ValidationError):
        adapter.validate_python("x" * 33)


def test_object_id_empty_rejected():
    adapter = TypeAdapter(ObjectId)
    with pytest.raises(ValidationError):
        adapter.validate_python("")


def test_participant_id_valid():
    adapter = TypeAdapter(ParticipantId)
    assert adapter.validate_python("MyCity") == "MyCity"


def test_quality_id_is_integer():
    adapter = TypeAdapter(QualityId)
    assert adapter.validate_python(42) == 42


def test_type_id_is_a_constrained_string():
    """TypeId became a string; the format
    is deliberately left open."""
    adapter = TypeAdapter(TypeId)
    # Legacy numeric identifiers still round-trip, as their string form.
    assert adapter.validate_python("1") == "1"
    # Alphanumeric and namespaced forms are the point of the change.
    for value in ("A1", "MOVA", "QMISS", "yunex.lampFault", "TOPAS 2503A"):
        assert adapter.validate_python(value) == value
    # An integer is no longer a TypeId.
    with pytest.raises(ValidationError):
        adapter.validate_python(1)
    # Empty, whitespace-only, padded and over-long values are rejected.
    for bad in ("", "   ", " x", "x ", "y" * 65):
        with pytest.raises(ValidationError):
            adapter.validate_python(bad)


def test_type_id_pattern_matches_its_comment():
    """Only leading and trailing whitespace is rejected; interior characters,
    including a newline, are allowed."""
    adapter = TypeAdapter(TypeId)
    for ok in ("a b", "a\nb", "a\tb"):
        assert adapter.validate_python(ok) == ok
    for bad in ("\na", "a\n", " a", "a "):
        with pytest.raises(ValidationError):
            adapter.validate_python(bad)
