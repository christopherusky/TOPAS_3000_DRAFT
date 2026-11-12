# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Meteorological definition: minimal construction, empty-id
#              rejection, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.meteorological.definition."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.meteorological.definition import MeteorologicalDefinition


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_meteorological_definition_minimal():
    d = MeteorologicalDefinition(meteorological_id="met-001", type_id="1", creation_date=NOW)
    assert d.meteorological_id == "met-001"


def test_meteorological_definition_rejects_empty_id():
    with pytest.raises(ValidationError):
        MeteorologicalDefinition(meteorological_id="", type_id="1", creation_date=NOW)


def test_meteorological_definition_dumps_camel_case():
    d = MeteorologicalDefinition(meteorological_id="met-001", type_id="1", creation_date=NOW)
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["meteorologicalId"] == "met-001"
