# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.1
# Description: The Quality model: minimal and full construction, required-
#              field enforcement, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.common.quality."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.common.quality import Quality


def test_quality_minimal():
    q = Quality(quality_id=1, source_name="SCOOT")
    assert q.quality_id == 1
    assert q.source_name == "SCOOT"
    assert q.last_updated is None


def test_quality_full():
    q = Quality(
        quality_id=42,
        source_name="MIDAS",
        source_type="inductive-loop",
        last_updated=datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc),
        participant_id="MyCity",
    )
    assert q.source_type == "inductive-loop"
    assert q.participant_id == "MyCity"


def test_quality_rejects_missing_required():
    with pytest.raises(ValidationError):
        Quality()


def test_quality_dumps_camel_case():
    q = Quality(quality_id=1, source_name="SCOOT")
    dumped = q.model_dump(by_alias=True, exclude_none=True)
    assert dumped == {"qualityId": 1, "sourceName": "SCOOT"}


def test_quality_rejects_an_empty_source_name():
    """A statement naming no source cannot be interpreted."""
    with pytest.raises(ValidationError):
        Quality(quality_id=1, source_name="")
