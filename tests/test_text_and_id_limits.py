# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-06
# Version:     1.0
# Description: The description length limits and the NMTOKEN subset pattern
# that v1 had and the models had lost, restored (B5, B6).
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.common.text and the ObjectId / ParticipantId pattern."""

import json
from pathlib import Path

import pytest
from pydantic import TypeAdapter, ValidationError

from topas_3000_model.common.ids import ObjectId, ParticipantId
from topas_3000_model.common.text import LongDescription, ShortDescription
from topas_3000_model.detector.definition import DetectorDefinition
from topas_3000_model.traffic_signal.definition import TrafficSignalDefinition
from topas_3000_model.transport_link.definition import TransportLinkDefinition
from topas_3000_model.transport_route.definition import TransportRouteDefinition
from topas_3000_model.vms.definition import VmsDefinition

DEFINITIONS = [DetectorDefinition, VmsDefinition, TransportLinkDefinition, TransportRouteDefinition, TrafficSignalDefinition]
SCHEMAS = Path(__file__).resolve().parent.parent / "Schemas"


@pytest.mark.parametrize("adapter_type", [ObjectId, ParticipantId], ids=["ObjectId", "ParticipantId"])
class TestIdPattern:
    @pytest.mark.parametrize("value", ["DET-0001", "a.b_c:d", "X", "0" * 32, "Mixed-Case.1"])
    def test_accepts_the_ascii_nmtoken_subset(self, adapter_type, value):
        assert TypeAdapter(adapter_type).validate_python(value) == value

    @pytest.mark.parametrize("value", ["has space", "tab\there", "new\nline", "slash/x", "comma,x", "é", "id#1", "", "x" * 33])
    def test_rejects_everything_else(self, adapter_type, value):
        with pytest.raises(ValidationError):
            TypeAdapter(adapter_type).validate_python(value)


@pytest.mark.parametrize(("adapter_type", "limit"), [(ShortDescription, 32), (LongDescription, 2000)])
class TestDescriptionLimits:
    def test_accepts_one_to_the_limit(self, adapter_type, limit):
        adapter = TypeAdapter(adapter_type)
        assert adapter.validate_python("x") == "x"
        assert adapter.validate_python("x" * limit) == "x" * limit

    def test_rejects_empty_and_over_the_limit(self, adapter_type, limit):
        adapter = TypeAdapter(adapter_type)
        for value in ("", "x" * (limit + 1)):
            with pytest.raises(ValidationError):
                adapter.validate_python(value)


@pytest.mark.parametrize("model", DEFINITIONS, ids=lambda m: m.__name__)
def test_every_definition_uses_the_shared_description_types(model):
    short = TypeAdapter(model.model_fields["short_description"].annotation)
    long = TypeAdapter(model.model_fields["long_description"].annotation)
    assert short.validate_python(None) is None and long.validate_python(None) is None
    with pytest.raises(ValidationError):
        short.validate_python("x" * 33)
    with pytest.raises(ValidationError):
        long.validate_python("x" * 2001)
    with pytest.raises(ValidationError):
        short.validate_python("")


def _definition_properties(domain: str, file: str, cls: str) -> dict:
    schema = json.loads((SCHEMAS / domain / file).read_text(encoding="utf-8"))
    return schema["$defs"][cls]["properties"]


def test_the_emitted_schemas_carry_the_limits():
    """The limits reach the JSON Schema, not only the Python models."""
    cases = [
        ("detector", "detector_definition-v1.0.json", "DetectorDefinition", "detectorId"),
        ("vms", "vms_definition-v1.0.json", "VmsDefinition", "vmsId"),
    ]
    for domain, file, cls, id_field in cases:
        props = _definition_properties(domain, file, cls)
        assert props[id_field]["pattern"] == r"^[A-Za-z0-9._:-]+$", domain
        assert (props["shortDescription"]["minLength"], props["shortDescription"]["maxLength"]) == (1, 32), domain
        assert (props["longDescription"]["minLength"], props["longDescription"]["maxLength"]) == (1, 2000), domain
    common = json.loads((SCHEMAS / "common" / "common-v1.0.json").read_text(encoding="utf-8"))
    assert r"^[A-Za-z0-9._:-]+$" in json.dumps(common), "ParticipantId pattern missing from common"
