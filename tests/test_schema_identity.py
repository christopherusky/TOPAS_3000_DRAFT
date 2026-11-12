# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.0
# Description: Schema identity: every emitted schema's $id is the URL of its
#              own file under the schema base, and every cross-file $ref
#              resolves through a registry keyed by $id, not only on disk.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""A schema published at its `$id` must resolve its own references (H4)."""

import json
from pathlib import Path

import pytest
from referencing import Registry, Resource
from referencing.exceptions import Unresolvable
from referencing.jsonschema import DRAFT202012

from bin.emit import SCHEMA_ID_BASE

ROOT = Path(__file__).resolve().parent.parent
TREES = [ROOT / "Schemas", ROOT / "Schemas_asyncapi"]


def _files(tree: Path) -> list[Path]:
    return sorted(tree.rglob("*.json"))


def _refs(node):
    if isinstance(node, dict):
        for k, v in node.items():
            if k == "$ref" and isinstance(v, str):
                yield v
            else:
                yield from _refs(v)
    elif isinstance(node, list):
        for item in node:
            yield from _refs(item)


@pytest.mark.parametrize("tree", TREES, ids=lambda t: t.name)
def test_id_is_the_file_url_under_the_base(tree):
    for path in _files(tree):
        schema = json.loads(path.read_text())
        expected = f"{SCHEMA_ID_BASE}/{path.relative_to(tree).as_posix()}"
        assert schema["$id"] == expected, path


@pytest.mark.parametrize("tree", TREES, ids=lambda t: t.name)
def test_every_ref_resolves_by_id_not_only_on_disk(tree):
    docs = {p: json.loads(p.read_text()) for p in _files(tree)}
    registry = Registry().with_resources(
        (d["$id"], Resource.from_contents(d, default_specification=DRAFT202012))
        for d in docs.values()
    )
    checked = 0
    for path, doc in docs.items():
        resolver = registry.resolver(base_uri=doc["$id"])
        for ref in set(_refs(doc)):
            try:
                resolver.lookup(ref)
            except Unresolvable as exc:  # pragma: no cover - failure path
                pytest.fail(f"{path.relative_to(tree)}: {ref} -> {exc}")
            checked += 1
    assert checked > 100


def test_the_base_is_a_versioned_https_url_without_a_trailing_slash():
    """Holds for the placeholder and for the real location that replaces it
    (change SCHEMA_ID_BASE in bin/emit.py, then regenerate)."""
    assert SCHEMA_ID_BASE.startswith("https://")
    assert SCHEMA_ID_BASE.endswith("/v1.0")
    assert not SCHEMA_ID_BASE.endswith("//v1.0")
