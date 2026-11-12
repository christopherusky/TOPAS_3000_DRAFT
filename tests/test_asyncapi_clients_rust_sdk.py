# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.0
# Description: Rust SDK generation: const to single-member-
#              enum transformation, traversal of nested and listed nodes,
#              external $ref inlining, refusal to merge conflicting names,
#              per-domain module coverage, typed rather than bare-string
#              discriminators, and generator stamping that skips hand-written
#              stubs.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for the Rust SDK generation.

The Rust SDK's own `cargo test` covers runtime behaviour, but it needs a
Rust toolchain. These run in the ordinary pytest suite and guard the two
things that were actually wrong during development: unresolved cross-file
references, and a discriminator that could not discriminate.
"""
import json
import re
from pathlib import Path

import pytest

from asyncapi_clients.generator.generate import (
    SchemaBundleConflict,
    _assert_titles_consistent,
    _bundle_external_refs,
    _const_to_single_enum,
)

ROOT = Path(__file__).resolve().parents[1]
SDK = ROOT / "asyncapi_clients" / "output" / "rust" / "sdk"


# ── the const -> enum transform ──

def test_const_becomes_single_member_enum():
    node = {"properties": {"kind": {"const": "reset", "type": "string"}}}
    assert _const_to_single_enum(node) == 1
    assert node["properties"]["kind"]["enum"] == ["reset"]
    assert "const" not in node["properties"]["kind"]


def test_existing_enum_is_left_alone():
    node = {"kind": {"const": "a", "enum": ["a", "b"]}}
    assert _const_to_single_enum(node) == 0
    assert node["kind"]["const"] == "a"


def test_transform_reaches_nested_and_listed_nodes():
    node = {"oneOf": [{"properties": {"k": {"const": "x"}}},
                      {"items": [{"const": "y"}]}]}
    assert _const_to_single_enum(node) == 2


# ── the bundler ──

def _write(tmp_path, rel, payload):
    p = tmp_path / rel
    p.parent.mkdir(parents=True, exist_ok=True)
    p.write_text(json.dumps(payload))
    return p


def test_external_refs_are_inlined(tmp_path):
    _write(tmp_path, "common/common.json",
           {"$defs": {"Point": {"type": "object",
                                "properties": {"c": {"$ref": "#/$defs/Coords"}}},
                      "Coords": {"type": "array"}}})
    src = _write(tmp_path, "detector/_c.json",
                 {"$defs": {"D": {"properties": {
                     "loc": {"$ref": "../common/common.json#/$defs/Point"}}}}})
    out = _bundle_external_refs(json.loads(src.read_text()), src)
    # Point came across, and so did the Coords it referenced from *its* file
    # — the transitive step that was missing on the first attempt.
    assert "Point" in out["$defs"]
    assert "Coords" in out["$defs"]
    assert '"../' not in json.dumps(out)


def test_conflicting_names_are_refused_not_merged(tmp_path):
    _write(tmp_path, "other/o.json", {"$defs": {"Thing": {"type": "string"}}})
    src = _write(tmp_path, "d/_c.json",
                 {"$defs": {"Thing": {"type": "integer"},
                            "X": {"properties": {
                                "t": {"$ref": "../other/o.json#/$defs/Thing"}}}}})
    # Silently keeping one would let a field change type without a word.
    with pytest.raises(SchemaBundleConflict):
        _bundle_external_refs(json.loads(src.read_text()), src)


def test_titled_string_types_agree_across_the_model():
    """Sharing a Rust type by title is only sound while this holds."""
    _assert_titles_consistent()


# ── the committed output ──

def test_every_domain_has_a_module():
    src = SDK / "src"
    assert (src / "lib.rs").exists()
    lib = (src / "lib.rs").read_text()
    modules = set(re.findall(r"pub mod (\w+);", lib))
    on_disk = {p.stem for p in src.glob("*.rs")} - {"lib"}
    assert modules == on_disk, "lib.rs and the module files disagree"
    assert "detector" in modules and "common" not in modules


def test_discriminators_are_typed_not_bare_strings():
    """The regression guard, without needing a Rust toolchain.

    A `kind` field typed `String` means the untagged union cannot tell its
    variants apart — which is how a calibrate command deserialised as a
    reset one.
    """
    offenders = []
    for path in (SDK / "src").glob("*.rs"):
        for line in path.read_text().splitlines():
            if re.match(r"\s*pub kind: ::std::string::String,\s*$", line):
                offenders.append(path.name)
                break
    assert not offenders, f"untyped `kind` discriminator in: {sorted(set(offenders))}"


def test_generated_modules_carry_the_typify_generator_stamp():
    header = (SDK / "src" / "detector.rs").read_text()[:800]
    assert "typify@" in header
    assert "Do not hand-edit" in header


def test_stub_test_is_not_stamped_as_generated():
    """It is hand-written; saying otherwise sends an editor to the wrong file."""
    header = (SDK / "tests" / "discriminator.rs").read_text()[:600]
    assert "Hand-written" in header
    assert "Generated from asyncapi.yaml" not in header
