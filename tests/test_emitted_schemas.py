# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-27
# Version:     1.13
# Description: Integration coverage: the emitted JSON Schemas are validated
#              against real payloads across every domain, accepting valid
#              documents and rejecting unknown discriminator kinds and
#              malformed input.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Integration tests: emitted schemas validate real payloads."""

import atexit
import contextlib
import functools
import io
import json
import shutil
import tempfile
from pathlib import Path

import jsonschema
import pytest

from bin.emit import main as _emit_main

SCHEMAS_DIR = Path(__file__).resolve().parent.parent / "Schemas"

# Deferred (Phase 4) domains are not in Schemas/. Their models stay
# tested, so emit them into a scratch directory and look there as well.
# The preview is built on first use, not at import, so collection does no I/O.
@functools.cache
def _schema_paths() -> dict[str, Path]:
    """Flat filename lookup over Schemas/<domain>/<filename> and the Phase 4 preview."""
    preview = Path(tempfile.mkdtemp(prefix="topas-phase4-preview-"))
    atexit.register(shutil.rmtree, preview, ignore_errors=True)
    with contextlib.redirect_stdout(io.StringIO()):
        _emit_main(["--phase", "4", "--output-dir", str(preview)])
    return {p.name: p for root in (preview, SCHEMAS_DIR) for p in root.rglob("*.json")}


def _load(name: str) -> dict:
    return json.loads(_schema_paths()[name].read_text())


def test_detector_measurement_validates_flow_payload():
    schema = _load("detector_measurement-v1.0.json")
    payload = {
        "kind": "flow",
        "detectorId": "det-001",
        "timestamp": "2026-04-11T10:00:00Z",
        "totalFlow": 42,
        "periodSeconds": 300,
        "classCounts": [],
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_detector_measurement_rejects_unknown_kind():
    schema = _load("detector_measurement-v1.0.json")
    payload = {
        "kind": "teleport",
        "detectorId": "det-001",
        "timestamp": "2026-04-11T10:00:00Z",
    }
    with pytest.raises(jsonschema.ValidationError):
        jsonschema.validate(payload, schema)


def test_detector_definition_validates_minimal_payload():
    schema = _load("detector_definition-v1.0.json")
    payload = {
        "detectorId": "det-001",
        "typeId": "1",
        "creationDate": "2026-04-11T10:00:00Z",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_detector_fault_validates_minimal_payload():
    schema = _load("detector_fault-v1.0.json")
    payload = {
        "detectorId": "det-001",
        "faultId": 101,
        "faultTypeId": "5",
        "creationDate": "2026-04-11T10:00:00Z",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_traffic_signal_definition_validates_minimal_payload():
    schema = _load("traffic_signal_definition-v1.0.json")
    payload = {
        "trafficSignalId": "ts-001",
        "controlMethodId": "1",
        "creationDate": "2026-04-11T10:00:00Z",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_traffic_signal_measurement_validates_minimal_payload():
    schema = _load("traffic_signal_measurement-v1.0.json")
    payload = {
        "trafficSignalId": "ts-001",
        "timestamp": "2026-04-11T10:00:00Z",
        "trafficSignalMode": "centralMode",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_traffic_signal_command_validates_set_plan_payload():
    schema = _load("traffic_signal_command-v1.0.json")
    payload = {
        "kind": "setPlan",
        "trafficSignalId": "ts-001",
        "commandCorrelationId": 1,
        "requestTime": "2026-04-11T10:00:00Z",
        "planNumber": 5,
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_traffic_signal_command_rejects_unknown_kind():
    schema = _load("traffic_signal_command-v1.0.json")
    payload = {
        "kind": "teleport",
        "trafficSignalId": "ts-001",
        "commandCorrelationId": 1,
        "requestTime": "2026-04-11T10:00:00Z",
    }
    with pytest.raises(jsonschema.ValidationError):
        jsonschema.validate(payload, schema)


def test_traffic_signal_fault_validates_minimal_payload():
    schema = _load("traffic_signal_fault-v1.0.json")
    payload = {
        "trafficSignalId": "ts-001",
        # Classification is a catalogue reference; the DATEX II enum is
        # retained but optional.
        "faultTypeId": "A3",
        "trafficSignalFaultType": "lampFault",
        "faultLastUpdateTime": "2026-04-11T10:00:00Z",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_transport_link_definition_validates_minimal_payload():
    schema = _load("transport_link_definition-v1.0.json")
    payload = {
        "transportLinkId": "tl-001",
        "typeId": "1",
        "creationDate": "2026-04-11T10:00:00Z",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_transport_link_measurement_validates_scoot_payload():
    schema = _load("transport_link_measurement-v1.0.json")
    payload = {
        "kind": "scoot",
        "transportLinkId": "tl-001",
        "timestamp": "2026-04-11T10:00:00Z",
        "congestionPercent": 42.5,
        "currentFlowVehicles": 120,
        "occupancyPercent": 35.0,
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_transport_link_measurement_validates_anpr_payload():
    schema = _load("transport_link_measurement-v1.0.json")
    payload = {
        "kind": "anpr",
        "transportLinkId": "tl-001",
        "timestamp": "2026-04-11T10:00:00Z",
        "numberPlatesIn": 100,
        "numberPlatesOut": 95,
        "numberPlatesMatches": 80,
        "numberPlatesMatchesUsed": 75,
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_transport_link_measurement_rejects_unknown_kind():
    schema = _load("transport_link_measurement-v1.0.json")
    payload = {
        "kind": "teleport",
        "transportLinkId": "tl-001",
        "timestamp": "2026-04-11T10:00:00Z",
    }
    with pytest.raises(jsonschema.ValidationError):
        jsonschema.validate(payload, schema)


def test_transport_link_command_validates_set_stoc_payload():
    schema = _load("transport_link_command-v1.0.json")
    payload = {
        "kind": "setStoc",
        "transportLinkId": "tl-001",
        "commandCorrelationId": 1,
        "requestTime": "2026-04-11T10:00:00Z",
        "stocValue": "42",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_transport_link_fault_validates_minimal_payload():
    schema = _load("transport_link_fault-v1.0.json")
    payload = {
        "transportLinkId": "tl-001",
        "faultId": 101,
        "faultTypeId": "5",
        "creationDate": "2026-04-11T10:00:00Z",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_transport_link_type_validates_transport_link_payload():
    schema = _load("transport_link_type-v1.0.json")
    payload = {
        "kind": "transportLink",
        "typeId": "1",
        "description": "SCOOT",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_transport_link_type_validates_link_status_payload():
    schema = _load("transport_link_type-v1.0.json")
    payload = {
        "kind": "linkStatus",
        "typeId": "10",
        "description": "Congested",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_transport_link_type_rejects_unknown_kind():
    schema = _load("transport_link_type-v1.0.json")
    payload = {
        "kind": "bogus",
        "typeId": "1",
        "description": "Nope",
    }
    with pytest.raises(jsonschema.ValidationError):
        jsonschema.validate(payload, schema)


def test_car_park_definition_validates_minimal_payload():
    schema = _load("car_park_definition-v1.0.json")
    payload = {
        "carParkId": "cp-001",
        "typeId": "1",
        "creationDate": "2026-04-11T10:00:00Z",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_car_park_measurement_validates_full_payload():
    schema = _load("car_park_measurement-v1.0.json")
    payload = {
        "carParkId": "cp-001",
        "timestamp": "2026-04-11T10:00:00Z",
        "occupancy": 320,
        "occupancyPercent": 64.0,
        "fillRateVehiclesPerMinute": 5.5,
        "exitRateVehiclesPerMinute": 2.1,
        "queueTimeSeconds": 90.0,
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_car_park_measurement_rejects_occupancy_percent_over_100():
    schema = _load("car_park_measurement-v1.0.json")
    payload = {
        "carParkId": "cp-001",
        "timestamp": "2026-04-11T10:00:00Z",
        "occupancyPercent": 150.0,
    }
    with pytest.raises(jsonschema.ValidationError):
        jsonschema.validate(payload, schema)


def test_car_park_command_validates_set_occupancy_payload():
    schema = _load("car_park_command-v1.0.json")
    payload = {
        "kind": "setOccupancy",
        "carParkId": "cp-001",
        "commandCorrelationId": 1,
        "requestTime": "2026-04-11T10:00:00Z",
        "occupancy": 320,
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_car_park_command_rejects_unknown_kind():
    schema = _load("car_park_command-v1.0.json")
    payload = {
        "kind": "teleport",
        "carParkId": "cp-001",
        "commandCorrelationId": 1,
        "requestTime": "2026-04-11T10:00:00Z",
    }
    with pytest.raises(jsonschema.ValidationError):
        jsonschema.validate(payload, schema)


def test_car_park_fault_validates_minimal_payload():
    schema = _load("car_park_fault-v1.0.json")
    payload = {
        "carParkId": "cp-001",
        "faultId": 101,
        "faultTypeId": "5",
        "creationDate": "2026-04-11T10:00:00Z",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_detector_history_validates_minimal_payload():
    schema = _load("detector_history-v1.0.json")
    payload = {
        "detectorId": "det-001",
        "logId": 1,
        "logDate": "2026-04-11T10:00:00Z",
        "logEntry": "Detector recalibrated",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_transport_link_history_validates_minimal_payload():
    schema = _load("transport_link_history-v1.0.json")
    payload = {
        "transportLinkId": "tl-001",
        "logId": 1,
        "logDate": "2026-04-11T10:00:00Z",
        "logEntry": "Link reconfigured",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_car_park_history_validates_minimal_payload():
    schema = _load("car_park_history-v1.0.json")
    payload = {
        "carParkId": "cp-001",
        "logId": 1,
        "logDate": "2026-04-11T10:00:00Z",
        "logEntry": "Capacity updated",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_car_park_type_validates_minimal_payload():
    schema = _load("car_park_type-v1.0.json")
    payload = {
        "typeId": "1",
        "description": "Multi-storey",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_detector_configuration_validates_minimal_payload():
    schema = _load("detector_configuration-v1.0.json")
    payload = {
        "detectorId": "det-001",
        "configurationDate": "2026-04-11T10:00:00Z",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_traffic_signal_configuration_validates_minimal_payload():
    schema = _load("traffic_signal_configuration-v1.0.json")
    payload = {
        "trafficSignalId": "ts-001",
        "configurationDate": "2026-04-11T10:00:00Z",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_transport_link_configuration_validates_minimal_payload():
    schema = _load("transport_link_configuration-v1.0.json")
    payload = {
        "kind": "scoot",
        "transportLinkId": "tl-001",
        "configurationDate": "2026-04-11T10:00:00Z",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_transport_link_configuration_validates_gps_payload():
    schema = _load("transport_link_configuration-v1.0.json")
    payload = {
        "kind": "gps",
        "transportLinkId": "tl-001",
        "configurationDate": "2026-04-11T10:00:00Z",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_transport_link_configuration_validates_anpr_payload():
    schema = _load("transport_link_configuration-v1.0.json")
    payload = {
        "kind": "anpr",
        "transportLinkId": "tl-001",
        "configurationDate": "2026-04-11T10:00:00Z",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_car_park_configuration_validates_minimal_payload():
    schema = _load("car_park_configuration-v1.0.json")
    payload = {
        "carParkId": "cp-001",
        "configurationDate": "2026-04-11T10:00:00Z",
    }
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(payload, schema)


def test_car_park_tariff_configuration_validates_minimal_payload():
    schema = _load("car_park_tariff_configuration-v1.0.json")
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate({"carParkId": "cp-001"}, schema)


def test_car_park_opening_times_configuration_validates_minimal_payload():
    schema = _load("car_park_opening_times_configuration-v1.0.json")
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate({"carParkId": "cp-001"}, schema)


def test_car_park_access_location_validates_minimal_payload():
    schema = _load("car_park_access_location-v1.0.json")
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate({"carParkId": "cp-001", "entryOrExit": "entry"}, schema)


def test_car_park_zone_list_entry_validates_minimal_payload():
    schema = _load("car_park_zone_list_entry-v1.0.json")
    jsonschema.Draft202012Validator.check_schema(schema)
    jsonschema.validate(
        {"carParkId": "cp-001", "networkZoneId": "zone-001"}, schema
    )


def test_every_schema_is_valid_draft_2020_12():
    """Every emitted schema must itself be a valid JSON Schema."""
    for path in SCHEMAS_DIR.rglob("*.json"):
        schema = json.loads(path.read_text())
        jsonschema.Draft202012Validator.check_schema(schema)
