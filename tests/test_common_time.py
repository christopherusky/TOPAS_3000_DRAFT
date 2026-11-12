# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.2
# Description: The Timestamp type: ISO-8601 string acceptance and naive-
#              datetime rejection.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.common.time."""

from datetime import datetime

import pytest
from pydantic import TypeAdapter, ValidationError

from topas_3000_model.common.time import Timestamp


def test_timestamp_accepts_iso_string():
    adapter = TypeAdapter(Timestamp)
    value = adapter.validate_python("2026-04-11T10:00:00Z")
    assert isinstance(value, datetime)
    assert value.tzinfo is not None


def test_timestamp_rejects_naive_datetime():
    adapter = TypeAdapter(Timestamp)
    with pytest.raises(ValidationError):
        adapter.validate_python("2026-04-11T10:00:00")
