# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.3
# Description: UTMCBaseModel, the shared Pydantic base: camelCase aliasing on
#              both parse and dump, snake_case attribute access,
#              extra='forbid' enforcement, and omission of None under
#              exclude_none.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.common.base.UTMCBaseModel."""

import pytest
from pydantic import ValidationError

from topas_3000_model.common.base import UTMCBaseModel


class _ExampleModel(UTMCBaseModel):
    some_field: str
    other_field: int | None = None


def test_camel_case_alias_on_serialize():
    """Fields serialize to camelCase JSON."""
    instance = _ExampleModel(some_field="hi", other_field=42)
    dumped = instance.model_dump(by_alias=True)
    assert dumped == {"someField": "hi", "otherField": 42}


def test_camel_case_alias_on_parse():
    """Fields can be parsed from camelCase JSON."""
    instance = _ExampleModel.model_validate({"someField": "hi", "otherField": 42})
    assert instance.some_field == "hi"
    assert instance.other_field == 42


def test_python_attribute_names_still_work():
    """Populating by Python attribute name is allowed."""
    instance = _ExampleModel(some_field="hi")
    assert instance.some_field == "hi"


def test_extra_fields_forbidden():
    """Unknown fields are rejected."""
    with pytest.raises(ValidationError) as exc:
        _ExampleModel.model_validate({"someField": "hi", "unknownField": 1})
    assert "unknownField" in str(exc.value) or "unknown_field" in str(exc.value)


def test_none_is_omitted_on_dump_with_exclude_none():
    """Dumping with exclude_none skips None fields."""
    instance = _ExampleModel(some_field="hi", other_field=None)
    dumped = instance.model_dump(by_alias=True, exclude_none=True)
    assert dumped == {"someField": "hi"}


def test_model_dump_defaults_to_camel_case():
    """With serialize_by_alias=True in the base config, model_dump() emits camelCase by default."""
    instance = _ExampleModel(some_field="hi", other_field=42)
    dumped = instance.model_dump()  # note: no by_alias=True
    assert dumped == {"someField": "hi", "otherField": 42}
