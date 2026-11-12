# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-05
# Version:     1.0
# Description: Device location on the transport and network model and
#              the transport link end point: the fields round-trip,
#              default to absent, reject both a network link and a network
#              path, are emitted in the schema, and match the wire names.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for device location and the transport link end point."""

import json
from pathlib import Path

import pytest
from pydantic import ValidationError

from topas_3000_model.common.location import Coordinates, Point
from topas_3000_model.detector.definition import DetectorDefinition
from topas_3000_model.transport_link.definition import TransportLinkDefinition
from topas_3000_model.vms.definition import VmsDefinition

SCHEMAS = Path(__file__).resolve().parent.parent / "Schemas"
BASE = {"type_id": "1", "creation_date": "2026-10-05T10:00:00Z"}

DEVICES = [
    (DetectorDefinition, "detector_id", "detector/detector_definition-v1.0.json"),
    (VmsDefinition, "vms_id", "vms/vms_definition-v1.0.json"),
]


@pytest.mark.parametrize("cls,id_field,_schema", DEVICES)
def test_location_fields_round_trip_with_wire_names(cls, id_field, _schema):
    m = cls(**{id_field: "x1"}, **BASE, transport_link_id="L1", link_distance_metres=12.5,
            network_path_id="P1")
    wire = json.loads(m.model_dump_json())
    assert wire["transportLinkId"] == "L1"
    assert wire["linkDistanceMetres"] == 12.5
    assert wire["networkPathId"] == "P1"
    assert "networkLinkId" not in wire
    assert cls.model_validate(wire) == m


@pytest.mark.parametrize("cls,id_field,_schema", DEVICES)
def test_location_fields_are_absent_by_default(cls, id_field, _schema):
    wire = json.loads(cls(**{id_field: "x1"}, **BASE).model_dump_json())
    assert not {"transportLinkId", "linkDistanceMetres", "networkLinkId", "networkPathId"} & set(wire)


@pytest.mark.parametrize("cls,id_field,_schema", DEVICES)
def test_a_network_link_and_a_network_path_together_are_rejected(cls, id_field, _schema):
    with pytest.raises(ValidationError, match="either networkLinkId or networkPathId"):
        cls(**{id_field: "x1"}, **BASE, network_link_id="a", network_path_id="b")
    cls(**{id_field: "x1"}, **BASE, network_link_id="a")
    cls(**{id_field: "x1"}, **BASE, network_path_id="b")


@pytest.mark.parametrize("cls,id_field,_schema", DEVICES)
@pytest.mark.parametrize("bad", [float("inf"), float("nan")])
def test_link_distance_must_be_finite(cls, id_field, _schema, bad):
    with pytest.raises(ValidationError):
        cls(**{id_field: "x1"}, **BASE, link_distance_metres=bad)


@pytest.mark.parametrize("cls,id_field,_schema", DEVICES)
def test_link_ids_keep_the_object_id_limits(cls, id_field, _schema):
    with pytest.raises(ValidationError):
        cls(**{id_field: "x1"}, **BASE, transport_link_id="")
    with pytest.raises(ValidationError):
        cls(**{id_field: "x1"}, **BASE, network_link_id="x" * 33)


@pytest.mark.parametrize("cls,id_field,schema", DEVICES)
def test_the_schema_carries_the_fields(cls, id_field, schema):
    doc = json.loads((SCHEMAS / schema).read_text())
    root = doc["$ref"].rsplit("/", 1)[-1]
    props = doc["$defs"][root]["properties"]
    for name in ("transportLinkId", "linkDistanceMetres", "networkLinkId", "networkPathId"):
        assert name in props, name
    assert "transportLinkId" not in doc["$defs"][root].get("required", [])


def test_traffic_signal_does_not_gain_link_references():
    """It derives from the 2017 profile, which carries only a point."""
    from topas_3000_model.traffic_signal.definition import TrafficSignalDefinition

    assert "transport_link_id" not in TrafficSignalDefinition.model_fields


def test_transport_link_end_point_round_trips_and_is_optional():
    start = Point(coordinates=Coordinates(latitude=51.5, longitude=-0.1))
    end = Point(coordinates=Coordinates(latitude=51.6, longitude=-0.2))
    m = TransportLinkDefinition(transport_link_id="L1", **BASE, location=start, end_point=end)
    wire = json.loads(m.model_dump_json())
    assert wire["endPoint"]["coordinates"]["latitude"] == 51.6
    assert TransportLinkDefinition.model_validate(wire) == m
    bare = json.loads(TransportLinkDefinition(transport_link_id="L1", **BASE).model_dump_json())
    assert "endPoint" not in bare and "location" not in bare
