# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: DayTypeDefinition and DateTypeDefinition: minimal, full and
#              custom construction, id bounds, rejection of missing and extra
#              fields, abnormal-day handling, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.common.day_type."""

import datetime

import pytest
from pydantic import ValidationError

from topas_3000_model.common.day_type import DateTypeDefinition, DayTypeDefinition


# -- DayTypeDefinition --------------------------------------------------------


def test_day_type_minimal():
    d = DayTypeDefinition(day_type_id=1, description="Monday")
    assert d.day_type_id == 1
    assert d.description == "Monday"
    assert d.short_name is None
    assert d.pre_defined is None
    assert d.extensions is None


def test_day_type_full():
    d = DayTypeDefinition(
        day_type_id=6,
        description="Saturday",
        short_name="Sat",
        pre_defined=True,
        extensions={"vendor": "example"},
    )
    assert d.short_name == "Sat"
    assert d.pre_defined is True
    assert d.extensions == {"vendor": "example"}


def test_day_type_custom():
    d = DayTypeDefinition(
        day_type_id=10,
        description="Race Day",
        pre_defined=False,
    )
    assert d.day_type_id == 10
    assert d.pre_defined is False


def test_day_type_rejects_missing_description():
    with pytest.raises(ValidationError):
        DayTypeDefinition(day_type_id=1)


def test_day_type_rejects_id_below_minimum():
    with pytest.raises(ValidationError):
        DayTypeDefinition(day_type_id=0, description="Invalid")


def test_day_type_rejects_id_above_maximum():
    with pytest.raises(ValidationError):
        DayTypeDefinition(day_type_id=101, description="Invalid")


def test_day_type_rejects_extra_fields():
    with pytest.raises(ValidationError):
        DayTypeDefinition(day_type_id=1, description="Monday", unknown="bad")


def test_day_type_dumps_camel_case():
    d = DayTypeDefinition(day_type_id=1, description="Monday", short_name="Mon")
    dumped = d.model_dump(by_alias=True, exclude_none=True)
    assert dumped == {
        "dayTypeId": 1,
        "description": "Monday",
        "shortName": "Mon",
    }


# -- DateTypeDefinition -------------------------------------------------------


def test_date_type_minimal():
    d = DateTypeDefinition(
        day_type_id=8,
        date=datetime.date(2025, 4, 18),
        abnormal=True,
    )
    assert d.day_type_id == 8
    assert d.date == datetime.date(2025, 4, 18)
    assert d.abnormal is True
    assert d.reason is None
    assert d.extensions is None


def test_date_type_full():
    d = DateTypeDefinition(
        day_type_id=8,
        date=datetime.date(2025, 4, 18),
        abnormal=True,
        reason="Easter Bank Holiday",
        extensions={"source": "national"},
    )
    assert d.reason == "Easter Bank Holiday"
    assert d.extensions == {"source": "national"}


def test_date_type_normal_day():
    d = DateTypeDefinition(
        day_type_id=1,
        date=datetime.date(2025, 4, 14),
        abnormal=False,
    )
    assert d.abnormal is False


def test_date_type_rejects_missing_abnormal():
    with pytest.raises(ValidationError):
        DateTypeDefinition(day_type_id=1, date=datetime.date(2025, 1, 1))


def test_date_type_rejects_missing_date():
    with pytest.raises(ValidationError):
        DateTypeDefinition(day_type_id=1, abnormal=False)


def test_date_type_rejects_extra_fields():
    with pytest.raises(ValidationError):
        DateTypeDefinition(
            day_type_id=1,
            date=datetime.date(2025, 1, 1),
            abnormal=False,
            unknown="bad",
        )


def test_date_type_dumps_camel_case():
    d = DateTypeDefinition(
        day_type_id=8,
        date=datetime.date(2025, 4, 18),
        abnormal=True,
        reason="Easter Bank Holiday",
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True)
    assert dumped == {
        "dayTypeId": 8,
        "date": datetime.date(2025, 4, 18),
        "abnormal": True,
        "reason": "Easter Bank Holiday",
    }
