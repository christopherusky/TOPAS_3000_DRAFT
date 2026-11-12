# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-06
# Version:     1.0
# Description: The v1 fault fields restored (B7): supplierFaultNumber,
#              subsystemTypeId, equipmentFault and communicationsFault on the
#              live device fault payloads, and subsystemTypeId on the fault
#              type catalogue entry.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for the optional fault fields restored from utmc_device_fault-v1.0.xsd."""

import json
from datetime import datetime, timezone
from pathlib import Path

import pytest
from pydantic import ValidationError

from topas_3000_model.detector.fault import DetectorFault
from topas_3000_model.fault_support.fault import FaultTypeDefinition
from topas_3000_model.transport_link.fault import TransportLinkFault
from topas_3000_model.transport_route.fault import TransportRouteFault
from topas_3000_model.vms.fault import VmsFault

NOW = datetime(2026, 10, 6, 9, 0, tzinfo=timezone.utc)
SCHEMAS = Path(__file__).resolve().parent.parent / "Schemas"

# model, the object id field, and the emitted schema file
FAULTS = [
    (DetectorFault, "detector_id", "detector/detector_fault-v1.0.json"),
    (TransportLinkFault, "transport_link_id", "transport_link/transport_link_fault-v1.0.json"),
    (TransportRouteFault, "transport_route_id", "transport_route/transport_route_fault-v1.0.json"),
    (VmsFault, "vms_id", "vms/vms_fault-v1.0.json"),
]
NEW_FIELDS = {
    "supplierFaultNumber": "S-2291/b",
    "subsystemTypeId": "1400",
    "equipmentFault": True,
    "communicationsFault": False,
}


def _base(id_field):
    return {id_field: "OBJ-1", "fault_id": 7, "fault_type_id": "5", "creation_date": NOW}


@pytest.mark.parametrize(("model", "id_field", "_file"), FAULTS, ids=lambda v: getattr(v, "__name__", None))
class TestDeviceFault:
    def test_valid_without_the_restored_fields(self, model, id_field, _file):
        """Additive: a payload from before the change is still valid."""
        f = model(**_base(id_field))
        assert f.supplier_fault_number is None
        assert f.subsystem_type_id is None
        assert f.equipment_fault is None
        assert f.communications_fault is None

    def test_accepts_the_restored_fields_and_round_trips_in_camel_case(self, model, id_field, _file):
        f = model(**_base(id_field), **NEW_FIELDS)
        dumped = json.loads(f.model_dump_json(exclude_none=True))
        for key, value in NEW_FIELDS.items():
            assert dumped[key] == value, key
        assert model.model_validate_json(f.model_dump_json()) == f

    def test_a_false_flag_is_kept_not_dropped(self, model, id_field, _file):
        """False is a statement ('not a communications fault'), not an absence."""
        f = model(**_base(id_field), communications_fault=False)
        assert json.loads(f.model_dump_json(exclude_none=True))["communicationsFault"] is False

    def test_the_supplier_number_is_free_text(self, model, id_field, _file):
        """v1 types it xsd:string with no constraint; an id with punctuation is fine."""
        assert model(**_base(id_field), supplier_fault_number="A 12/3 (rev 2)").supplier_fault_number

    def test_subsystem_type_id_is_a_type_id(self, model, id_field, _file):
        with pytest.raises(ValidationError):
            model(**_base(id_field), subsystem_type_id="")

    def test_extra_fields_are_still_rejected(self, model, id_field, _file):
        with pytest.raises(ValidationError):
            model(**_base(id_field), subsystem_number=3)

    def test_the_emitted_schema_carries_the_fields_as_optional(self, model, id_field, _file):
        schema = json.loads((SCHEMAS / _file).read_text(encoding="utf-8"))
        definition = schema["$defs"][model.__name__]
        for key in NEW_FIELDS:
            assert key in definition["properties"], key
            assert key not in definition["required"], key


def test_fault_type_definition_carries_an_optional_subsystem():
    plain = FaultTypeDefinition(fault_type_id="F3", description="Minimum green not met")
    assert plain.subsystem_type_id is None
    with_sub = FaultTypeDefinition(fault_type_id="F3", description="Minimum green not met", subsystem_type_id="1400")
    assert json.loads(with_sub.model_dump_json(exclude_none=True))["subsystemTypeId"] == "1400"
    with pytest.raises(ValidationError):
        FaultTypeDefinition(fault_type_id="F3", description="x", subsystem_type_id="")


def test_traffic_signal_fault_is_unchanged():
    """It follows the 2017 profile, where trafficSignalManufacturerFaultCode is the supplier's own number."""
    from topas_3000_model.traffic_signal.fault import TrafficSignalFault

    assert "supplier_fault_number" not in TrafficSignalFault.model_fields
    assert "traffic_signal_manufacturer_fault_code" in TrafficSignalFault.model_fields
