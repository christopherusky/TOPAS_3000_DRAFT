# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-27
# Version:     1.0
# Description: The fault-type catalogue extensions: unchanged minimal
#              entries, templated entries carrying parameters, code and id as
#              distinct fields, the category value set, rejection of unknown
#              fields and empty collections, and that every fault payload
#              carries parameters.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for the fault-type catalogue extensions.

The catalogue is what lets a fault whose meaning depends on runtime values
— "minimum green not met on stage 3" — be expressed at all. A closed enum
cannot, which is what PR #2 thread #33 reported.
"""
import pytest
from pydantic import ValidationError

from topas_3000_model.fault_support.enums import FaultCategory
from topas_3000_model.fault_support.fault import FaultTypeDefinition


def test_minimal_entry_unchanged():
    """The earlier shape still validates — the new fields are all optional."""
    e = FaultTypeDefinition(fault_type_id="1", description="Lamp failure")
    assert e.code is None
    assert e.category is None
    assert e.message_template is None
    assert e.parameter_names is None


def test_templated_entry():
    e = FaultTypeDefinition(
        fault_type_id="F3",
        description="Minimum green not met",
        code="F3",
        category=FaultCategory.SIGNAL_TIMINGS,
        message_template=(
            "Minimum green not met: stage {0} ran {1}s against a minimum of {2}s"
        ),
        parameter_names=["stageNumber", "actualSeconds", "minimumSeconds"],
    )
    dumped = e.model_dump(mode="json", exclude_none=True)
    assert dumped["code"] == "F3"
    assert dumped["category"] == "signalTimings"
    assert dumped["parameterNames"][0] == "stageNumber"


def test_code_and_id_are_distinct():
    """Two vendors may publish the same code; the id is what disambiguates."""
    a = FaultTypeDefinition(fault_type_id="yunex.A1", description="Detector fault", code="A1")
    b = FaultTypeDefinition(fault_type_id="siemens.A1", description="Detector fault", code="A1")
    assert a.code == b.code
    assert a.fault_type_id != b.fault_type_id


def test_dumps_camel_case():
    e = FaultTypeDefinition(
        fault_type_id="A1", description="x", message_template="y {0}",
        parameter_names=["z"],
    )
    dumped = e.model_dump()
    assert "messageTemplate" in dumped
    assert "parameterNames" in dumped
    assert "message_template" not in dumped


def test_rejects_unknown_field():
    with pytest.raises(ValidationError):
        FaultTypeDefinition(fault_type_id="A1", description="x", nonsense=1)


def test_rejects_empty_collections_and_strings():
    with pytest.raises(ValidationError):
        FaultTypeDefinition(fault_type_id="A1", description="x", parameter_names=[])
    with pytest.raises(ValidationError):
        FaultTypeDefinition(fault_type_id="A1", description="x", message_template="")
    with pytest.raises(ValidationError):
        FaultTypeDefinition(fault_type_id="A1", description="x", code="")


def test_category_values():
    assert {c.value for c in FaultCategory} == {
        "hardware",
        "communications",
        "signalTimings",
        "environmental",
        "other",
    }


@pytest.mark.parametrize("domain", [
    "access_control", "air_quality", "car_park", "cctv", "detector",
    "meteorological", "traffic_signal", "transport_link", "transport_route",
    "vms",
])
def test_every_fault_payload_carries_parameters(domain):
    """Templating is a property of the shared catalogue, so every domain that
    references it can carry the values."""
    import importlib
    mod = importlib.import_module(f"topas_3000_model.{domain}.fault")
    fault_cls = next(
        obj for name, obj in vars(mod).items()
        if name.endswith("Fault") and hasattr(obj, "model_fields")
        and "fault_type_id" in getattr(obj, "model_fields", {})
    )
    assert "parameters" in fault_cls.model_fields
