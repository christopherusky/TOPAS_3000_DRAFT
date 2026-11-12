# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.1
# Description: bin/validate.py: that a clean schema passes and that a broken
#              same-file ref, a broken cross-file ref and a ref to a non-
#              existent file are each caught; a ref to the right filename in
#              the wrong directory is caught; the AsyncAPI document's component,
#              payload-file and reply-message references are checked.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for bin/validate.py."""

import json
from pathlib import Path

import pytest

from bin.validate import validate_asyncapi, validate_ref_integrity


def test_clean_schema_passes(tmp_path: Path):
    """A schema with only valid refs returns no errors."""
    schema = {
        "$defs": {
            "Foo": {"type": "string"},
            "Bar": {"$ref": "#/$defs/Foo"},
        }
    }
    (tmp_path / "a.json").write_text(json.dumps(schema))
    errors = validate_ref_integrity(tmp_path)
    assert errors == []


def test_broken_same_file_ref_caught(tmp_path: Path):
    schema = {
        "$defs": {
            "Bar": {"$ref": "#/$defs/Missing"},
        }
    }
    (tmp_path / "a.json").write_text(json.dumps(schema))
    errors = validate_ref_integrity(tmp_path)
    assert len(errors) == 1
    assert "Missing" in errors[0]


def test_cross_file_ref_resolves(tmp_path: Path):
    schema_a = {"$defs": {"Foo": {"type": "string"}}}
    schema_b = {"$defs": {"Bar": {"$ref": "a.json#/$defs/Foo"}}}
    (tmp_path / "a.json").write_text(json.dumps(schema_a))
    (tmp_path / "b.json").write_text(json.dumps(schema_b))
    errors = validate_ref_integrity(tmp_path)
    assert errors == []


def test_broken_cross_file_ref_caught(tmp_path: Path):
    schema_a = {"$defs": {"Foo": {"type": "string"}}}
    schema_b = {"$defs": {"Bar": {"$ref": "a.json#/$defs/Missing"}}}
    (tmp_path / "a.json").write_text(json.dumps(schema_a))
    (tmp_path / "b.json").write_text(json.dumps(schema_b))
    errors = validate_ref_integrity(tmp_path)
    assert len(errors) == 1
    assert "Missing" in errors[0]


def test_ref_to_nonexistent_file_caught(tmp_path: Path):
    schema_b = {"$defs": {"Bar": {"$ref": "nonexistent.json#/$defs/Foo"}}}
    (tmp_path / "b.json").write_text(json.dumps(schema_b))
    errors = validate_ref_integrity(tmp_path)
    assert len(errors) == 1
    assert "nonexistent.json" in errors[0]


def _two_domains(tmp_path: Path) -> None:
    (tmp_path / "common").mkdir()
    (tmp_path / "detector").mkdir()
    (tmp_path / "other").mkdir()
    (tmp_path / "common" / "common.json").write_text(
        json.dumps({"$defs": {"Quality": {"type": "string"}}})
    )


def test_cross_file_ref_resolves_by_path(tmp_path: Path):
    _two_domains(tmp_path)
    (tmp_path / "detector" / "d.json").write_text(
        json.dumps({"$defs": {"X": {"$ref": "../common/common.json#/$defs/Quality"}}})
    )
    assert validate_ref_integrity(tmp_path) == []


def test_right_filename_in_the_wrong_directory_is_caught(tmp_path: Path):
    """The filename exists, the path does not: matching on basename let this pass."""
    _two_domains(tmp_path)
    (tmp_path / "detector" / "d.json").write_text(
        json.dumps({"$defs": {"X": {"$ref": "../other/common.json#/$defs/Quality"}}})
    )
    errors = validate_ref_integrity(tmp_path)
    assert len(errors) == 1
    assert "non-existent file" in errors[0]


def test_ref_escaping_the_tree_is_caught(tmp_path: Path):
    tree = tmp_path / "tree"
    tree.mkdir()
    (tmp_path / "outside.json").write_text(json.dumps({"$defs": {"Q": {"type": "string"}}}))
    (tree / "a.json").write_text(
        json.dumps({"$defs": {"X": {"$ref": "../outside.json#/$defs/Q"}}})
    )
    assert len(validate_ref_integrity(tree)) == 1


def test_duplicate_basenames_in_different_domains_are_reported(tmp_path: Path):
    _two_domains(tmp_path)
    (tmp_path / "other" / "common.json").write_text(json.dumps({"$defs": {}}))
    errors = validate_ref_integrity(tmp_path)
    assert any("duplicate filename" in e for e in errors)


# ── asyncapi.yaml ──


def _doc(tmp_path: Path, **overrides) -> Path:
    (tmp_path / "Schemas_asyncapi").mkdir(exist_ok=True)
    (tmp_path / "Schemas_asyncapi" / "p.json").write_text("{}")
    doc = {
        "channels": {
            "d.command": {
                "publish": {"message": {"$ref": "#/components/messages/Cmd"}},
                "subscribe": {"message": {"$ref": "#/components/messages/Cmd"}},
                "x-amqp1-request-reply": {
                    "responseMessage": {"$ref": "#/components/messages/Resp"}
                },
            }
        },
        "components": {
            "messages": {
                "Cmd": {"payload": {"$ref": "Schemas_asyncapi/p.json"}},
                "Resp": {"payload": {"$ref": "Schemas_asyncapi/p.json"}},
            }
        },
    }
    doc.update(overrides)
    path = tmp_path / "asyncapi.yaml"
    import yaml

    path.write_text(yaml.safe_dump(doc))
    return path


def test_a_consistent_asyncapi_document_passes(tmp_path: Path):
    assert validate_asyncapi(_doc(tmp_path), tmp_path) == []


def test_a_missing_component_reference_is_caught(tmp_path: Path):
    doc = _doc(tmp_path)
    import yaml

    data = yaml.safe_load(doc.read_text())
    data["channels"]["d.command"]["publish"]["message"]["$ref"] = "#/components/messages/Nope"
    doc.write_text(yaml.safe_dump(data))
    errors = validate_asyncapi(doc, tmp_path)
    assert any("Nope" in e for e in errors)


def test_a_payload_pointing_at_a_missing_file_is_caught(tmp_path: Path):
    doc = _doc(tmp_path)
    (tmp_path / "Schemas_asyncapi" / "p.json").unlink()
    errors = validate_asyncapi(doc, tmp_path)
    assert any("non-existent file" in e for e in errors)


def test_a_reply_message_that_does_not_exist_is_caught(tmp_path: Path):
    import yaml

    doc = _doc(tmp_path)
    data = yaml.safe_load(doc.read_text())
    data["channels"]["d.command"]["x-amqp1-request-reply"]["responseMessage"]["$ref"] = (
        "#/components/messages/Ghost"
    )
    doc.write_text(yaml.safe_dump(data))
    errors = validate_asyncapi(doc, tmp_path)
    assert any("Ghost" in e for e in errors)


def test_a_channel_missing_an_operation_is_caught(tmp_path: Path):
    import yaml

    doc = _doc(tmp_path)
    data = yaml.safe_load(doc.read_text())
    del data["channels"]["d.command"]["subscribe"]
    doc.write_text(yaml.safe_dump(data))
    assert any("subscribe" in e for e in validate_asyncapi(doc, tmp_path))


def test_a_payload_ref_that_escapes_schemas_asyncapi_is_caught(tmp_path: Path):
    """The file exists, but it is not one of the AsyncAPI schemas."""
    import yaml

    doc = _doc(tmp_path)
    (tmp_path / "elsewhere.json").write_text("{}")
    data = yaml.safe_load(doc.read_text())
    data["components"]["messages"]["Cmd"]["payload"]["$ref"] = "elsewhere.json"
    doc.write_text(yaml.safe_dump(data))
    errors = validate_asyncapi(doc, tmp_path)
    assert any("outside Schemas_asyncapi" in e for e in errors)
    data["components"]["messages"]["Cmd"]["payload"]["$ref"] = "Schemas_asyncapi/../elsewhere.json"
    doc.write_text(yaml.safe_dump(data))
    assert any("outside Schemas_asyncapi" in e for e in validate_asyncapi(doc, tmp_path))
