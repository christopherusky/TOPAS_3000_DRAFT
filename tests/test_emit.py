# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.26
# Description: bin/emit.py: that the file specs cover the pilot domains,
#              discriminated-union and cross-file $ref construction, common
#              library emission, and that emit_all writes its files, is
#              idempotent, and produces schemas whose refs all resolve; and
#              that a schema the generators no longer produce is pruned,
#              including from a subdirectory that is then left empty.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for bin/emit.py."""

import json
from pathlib import Path

import pytest

from bin.emit import FILE_SPECS, SCHEMA_ID_BASE, SCOPE, build_file_schema, emit_all


_ALL_PILOT_FILENAMES = {
    "common-v1.0.json",
    "fault_support-v1.0.json",
    "detector_definition-v1.0.json",
    "detector_configuration-v1.0.json",
    "detector_measurement-v1.0.json",
    "detector_command-v1.0.json",
    "detector_command_response-v1.0.json",
    "detector_fault-v1.0.json",
    "detector_type-v1.0.json",
    "detector_history-v1.0.json",
    "traffic_signal_definition-v1.0.json",
    "traffic_signal_configuration-v1.0.json",
    "traffic_signal_measurement-v1.0.json",
    "traffic_signal_command-v1.0.json",
    "traffic_signal_command_response-v1.0.json",
    "traffic_signal_fault-v1.0.json",
    "traffic_signal_type-v1.0.json",
    "traffic_signal_history-v1.0.json",
    "transport_link_definition-v1.0.json",
    "transport_link_configuration-v1.0.json",
    "transport_link_measurement-v1.0.json",
    "transport_link_command-v1.0.json",
    "transport_link_command_response-v1.0.json",
    "transport_link_fault-v1.0.json",
    "transport_link_type-v1.0.json",
    "transport_link_history-v1.0.json",
    "car_park_definition-v1.0.json",
    "car_park_configuration-v1.0.json",
    "car_park_measurement-v1.0.json",
    "car_park_command-v1.0.json",
    "car_park_command_response-v1.0.json",
    "car_park_fault-v1.0.json",
    "car_park_type-v1.0.json",
    "car_park_history-v1.0.json",
    "car_park_tariff_configuration-v1.0.json",
    "car_park_opening_times_configuration-v1.0.json",
    "car_park_access_location-v1.0.json",
    "car_park_zone_list_entry-v1.0.json",
    "access_control_definition-v1.0.json",
    "access_control_configuration-v1.0.json",
    "access_control_measurement-v1.0.json",
    "access_control_command-v1.0.json",
    "access_control_command_response-v1.0.json",
    "access_control_fault-v1.0.json",
    "access_control_type-v1.0.json",
    "access_control_history-v1.0.json",
    "air_quality_definition-v1.0.json",
    "air_quality_configuration-v1.0.json",
    "air_quality_measurement-v1.0.json",
    "air_quality_command-v1.0.json",
    "air_quality_command_response-v1.0.json",
    "air_quality_fault-v1.0.json",
    "air_quality_type-v1.0.json",
    "air_quality_history-v1.0.json",
    "air_quality_enhanced_definition-v1.0.json",
    "air_quality_enhanced_configuration-v1.0.json",
    "air_quality_enhanced_measurement-v1.0.json",
    "meteorological_definition-v1.0.json",
    "meteorological_measurement-v1.0.json",
    "meteorological_command-v1.0.json",
    "meteorological_command_response-v1.0.json",
    "meteorological_fault-v1.0.json",
    "meteorological_type-v1.0.json",
    "meteorological_history-v1.0.json",
    "transport_route_definition-v1.0.json",
    "transport_route_configuration-v1.0.json",
    "transport_route_measurement-v1.0.json",
    "transport_route_command-v1.0.json",
    "transport_route_command_response-v1.0.json",
    "transport_route_fault-v1.0.json",
    "transport_route_type-v1.0.json",
    "transport_route_history-v1.0.json",
    "vms_definition-v1.0.json",
    "vms_configuration-v1.0.json",
    "vms_measurement-v1.0.json",
    "vms_command-v1.0.json",
    "vms_command_response-v1.0.json",
    "vms_fault-v1.0.json",
    "vms_type-v1.0.json",
    "vms_history-v1.0.json",
    "cctv_definition-v1.0.json",
    "cctv_configuration-v1.0.json",
    "cctv_measurement-v1.0.json",
    "cctv_command-v1.0.json",
    "cctv_command_response-v1.0.json",
    "cctv_fault-v1.0.json",
    "cctv_type-v1.0.json",
    "cctv_history-v1.0.json",
    "traffic_event_common-v1.0.json",
    "traffic_event_accident-v1.0.json",
    "traffic_event_event-v1.0.json",
    "traffic_event_incident-v1.0.json",
    "traffic_event_roadworks-v1.0.json",
    "network_node-v1.0.json",
    "network_link-v1.0.json",
    "network_zone-v1.0.json",
    "network_path-v1.0.json",
    "network_turn-v1.0.json",
    "strategy_definition-v1.0.json",
    "strategy_status-v1.0.json",
    "strategy_command-v1.0.json",
    "strategy_command_response-v1.0.json",
    # Sync-request channels — one per data-bearing domain.
    "access_control_sync_request-v1.0.json",
    "air_quality_sync_request-v1.0.json",
    "air_quality_enhanced_sync_request-v1.0.json",
    "car_park_sync_request-v1.0.json",
    "cctv_sync_request-v1.0.json",
    "detector_sync_request-v1.0.json",
    "meteorological_sync_request-v1.0.json",
    "network_sync_request-v1.0.json",
    "strategy_sync_request-v1.0.json",
    "traffic_event_sync_request-v1.0.json",
    "traffic_signal_sync_request-v1.0.json",
    "transport_link_sync_request-v1.0.json",
    "transport_route_sync_request-v1.0.json",
    "vms_sync_request-v1.0.json",
    # Sync completion — the reply that ends a sync batch (A12). Addressed
    # to the requester's reply node, so no channel of its own.
    "access_control_sync_complete-v1.0.json",
    "air_quality_sync_complete-v1.0.json",
    "air_quality_enhanced_sync_complete-v1.0.json",
    "car_park_sync_complete-v1.0.json",
    "cctv_sync_complete-v1.0.json",
    "detector_sync_complete-v1.0.json",
    "meteorological_sync_complete-v1.0.json",
    "network_sync_complete-v1.0.json",
    "strategy_sync_complete-v1.0.json",
    "traffic_event_sync_complete-v1.0.json",
    "traffic_signal_sync_complete-v1.0.json",
    "transport_link_sync_complete-v1.0.json",
    "transport_route_sync_complete-v1.0.json",
    "vms_sync_complete-v1.0.json",
}

# Deferred (Phase 4) domains are modelled and tested but not emitted, so
# the live expectation is the full list minus their files.
_DEFERRED_PREFIXES = tuple(
    f"{d}_" for d in sorted(SCOPE.deferred(SCOPE.live_phase), key=len, reverse=True)
)
EXPECTED_FILENAMES = {
    n for n in _ALL_PILOT_FILENAMES if not n.startswith(_DEFERRED_PREFIXES)
}
def test_file_specs_cover_pilot():
    """All pilot + traffic signal output files are defined."""
    names = {spec.filename for spec in FILE_SPECS}
    assert names == EXPECTED_FILENAMES


def test_build_detector_measurement_has_discriminated_union():
    """detector_measurement has DetectorMeasurement as the top-level canonical type."""
    spec = next(s for s in FILE_SPECS if s.filename == "detector_measurement-v1.0.json")
    schema = build_file_schema(spec)

    assert schema["$schema"] == "https://json-schema.org/draft/2020-12/schema"
    assert schema["$id"] == f"{SCHEMA_ID_BASE}/detector/detector_measurement-v1.0.json"
    assert schema["$ref"] == "#/$defs/DetectorMeasurement"
    assert "DetectorMeasurement" in schema["$defs"]
    assert "FlowDynamic" in schema["$defs"]
    assert "QueueDynamic" in schema["$defs"]
    assert "oneOf" in schema["$defs"]["DetectorMeasurement"]


def test_build_detector_measurement_cross_file_refs_to_common():
    """Refs to ObjectId and Quality are rewritten to common-v1.0.json cross-file refs."""
    spec = next(s for s in FILE_SPECS if s.filename == "detector_measurement-v1.0.json")
    schema = build_file_schema(spec)

    serialized = json.dumps(schema)
    # Quality and ObjectId should not appear as local $defs in this file
    assert "Quality" not in schema["$defs"]
    # But refs to them should appear as cross-file refs
    assert "common-v1.0.json#/$defs/Quality" in serialized


def test_build_common_library_file():
    """common-v1.0.json is a library file with no top-level $ref."""
    spec = next(s for s in FILE_SPECS if s.filename == "common-v1.0.json")
    schema = build_file_schema(spec)

    assert "$ref" not in schema  # library files have no canonical type
    assert "Quality" in schema["$defs"]
    assert "Point" in schema["$defs"]
    assert "Coordinates" in schema["$defs"]
    assert "DayTypeDefinition" in schema["$defs"]
    assert "DateTypeDefinition" in schema["$defs"]
    assert "SubsystemDefinition" in schema["$defs"]


def test_emit_all_writes_files(tmp_path: Path, monkeypatch):
    """emit_all writes all pilot + traffic signal schema files."""
    import bin.emit as emit_mod
    monkeypatch.setattr(emit_mod, "OUTPUT_DIR", tmp_path)
    emit_all()
    written = {p.name for p in tmp_path.rglob("*.json")}
    assert written == EXPECTED_FILENAMES


def test_emit_all_is_idempotent(tmp_path: Path, monkeypatch):
    """Running emit_all twice produces byte-identical output."""
    import bin.emit as emit_mod
    monkeypatch.setattr(emit_mod, "OUTPUT_DIR", tmp_path)
    emit_all()
    first = {p.name: p.read_bytes() for p in tmp_path.rglob("*.json")}
    emit_all()
    second = {p.name: p.read_bytes() for p in tmp_path.rglob("*.json")}
    assert first == second


def test_all_emitted_refs_resolve(tmp_path: Path, monkeypatch):
    """Every $ref in every emitted file resolves to an existing $def.

    Walks each emitted JSON Schema file, collects all $ref values, and verifies
    each one points at a $def that actually exists — either same-file
    (#/$defs/Name) or cross-file (other-file.json#/$defs/Name). Catches the
    class of bug where a type is referenced but not actually emitted in its
    home file's $defs.
    """
    import bin.emit as emit_mod
    monkeypatch.setattr(emit_mod, "OUTPUT_DIR", tmp_path)
    emit_all()

    # Load every emitted file
    files: dict[str, dict] = {}
    for path in sorted(tmp_path.rglob("*.json")):
        files[path.name] = json.loads(path.read_text())

    def _walk_refs(node, path: str = "") -> list[tuple[str, str]]:
        refs = []
        if isinstance(node, dict):
            if "$ref" in node and isinstance(node["$ref"], str):
                refs.append((path, node["$ref"]))
            for key, val in node.items():
                refs.extend(_walk_refs(val, f"{path}/{key}"))
        elif isinstance(node, list):
            for i, item in enumerate(node):
                refs.extend(_walk_refs(item, f"{path}[{i}]"))
        return refs

    errors: list[str] = []
    for filename, schema in files.items():
        local_defs = set(schema.get("$defs", {}).keys())
        for ref_path, ref in _walk_refs(schema):
            if ref.startswith("#/$defs/"):
                target = ref.removeprefix("#/$defs/")
                if target not in local_defs:
                    errors.append(f"{filename}{ref_path}: local $ref to missing '{target}'")
            elif "#/$defs/" in ref:
                file_part, _, frag = ref.partition("#/$defs/")
                # Cross-file refs are relative paths like "../common/common-v1.0.json";
                # the files dict is keyed by basename.
                file_key = Path(file_part).name
                if file_key not in files:
                    errors.append(
                        f"{filename}{ref_path}: cross-file $ref to non-existent file '{file_part}'"
                    )
                elif frag not in set(files[file_key].get("$defs", {}).keys()):
                    errors.append(
                        f"{filename}{ref_path}: cross-file $ref to missing '{file_part}#/$defs/{frag}'"
                    )

    assert errors == [], f"Dangling $refs found:\n  " + "\n  ".join(errors)


def test_emit_all_prunes_a_schema_it_no_longer_produces(tmp_path: Path, monkeypatch):
    """A stale file inside a domain directory is removed. The root-level cleanup
    never touched domain subdirectories, so such a file survived indefinitely."""
    import bin.emit as emit_mod
    monkeypatch.setattr(emit_mod, "OUTPUT_DIR", tmp_path)
    emit_all()
    stale = tmp_path / "detector" / "detector_retired-v1.0.json"
    stale.write_text(json.dumps({"$id": f"{SCHEMA_ID_BASE}/detector/detector_retired-v1.0.json"}))
    emit_all()
    assert not stale.exists()
    assert {p.name for p in tmp_path.rglob("*.json")} == EXPECTED_FILENAMES


def test_emit_all_removes_a_directory_left_empty(tmp_path: Path, monkeypatch):
    import bin.emit as emit_mod
    monkeypatch.setattr(emit_mod, "OUTPUT_DIR", tmp_path)
    emit_all()
    gone = tmp_path / "retired_domain"
    gone.mkdir()
    (gone / "retired_domain_x-v1.0.json").write_text(
        json.dumps({"$id": f"{SCHEMA_ID_BASE}/retired_domain/retired_domain_x-v1.0.json"})
    )
    emit_all()
    assert not gone.exists()


def test_emit_all_keeps_every_current_schema(tmp_path: Path, monkeypatch):
    """Pruning must not delete anything it has just written."""
    import bin.emit as emit_mod
    monkeypatch.setattr(emit_mod, "OUTPUT_DIR", tmp_path)
    emit_all()
    before = {p.relative_to(tmp_path) for p in tmp_path.rglob("*.json")}
    emit_all()
    assert {p.relative_to(tmp_path) for p in tmp_path.rglob("*.json")} == before


def test_emit_all_never_deletes_files_it_did_not_emit(tmp_path: Path):
    """A preview into a directory with other content must leave that content alone.
    Pruning once removed every unwritten *.json and every empty directory."""
    (tmp_path / "keep").mkdir()
    (tmp_path / "keep" / "mine.json").write_text('{"a": 1}\n')
    (tmp_path / "empty_dir").mkdir()
    (tmp_path / "notes.json").write_text("not json at all")
    (tmp_path / "other.json").write_text('{"$id": "https://example.org/other.json"}\n')
    emit_all(output_dir=tmp_path)
    assert (tmp_path / "keep" / "mine.json").read_text() == '{"a": 1}\n'
    assert (tmp_path / "empty_dir").is_dir()
    assert (tmp_path / "notes.json").read_text() == "not json at all"
    assert (tmp_path / "other.json").exists()
