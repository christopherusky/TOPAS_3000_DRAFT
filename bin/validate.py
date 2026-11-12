# ──────────────────────────────────────────────────────────────────────
# Origin:     Cross-cutting $ref integrity checker for TOPAS 3000A
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.3
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""$ref integrity checker for the schema trees and the AsyncAPI document.

Checks three things and exits non-zero if any fails:

1. ``Schemas/``: every ``$ref`` (same-file ``#/$defs/...`` and cross-file
   ``../{domain}/{filename}#/$defs/...``) resolves, **by path**, to a
   definition that exists. A reference to the right filename in the wrong
   directory is an error; matching on the bare filename would let it pass.
2. ``Schemas_asyncapi/``: the same check on the AsyncAPI-flavoured copies.
3. ``asyncapi.yaml``: every ``$ref`` to a component resolves, every message
   payload points at a schema file that exists, and every request/reply
   extension names a reply message that exists.

Basename collisions are also reported, since SDK generators rely on schema
filenames being globally unique.

Usage: python bin/validate.py
"""

from __future__ import annotations

import json
import sys
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parent.parent
SCHEMAS_DIR = ROOT / "Schemas"
ASYNCAPI_SCHEMAS_DIR = ROOT / "Schemas_asyncapi"
ASYNCAPI_DOC = ROOT / "asyncapi.yaml"


def _collect_defs(schema: dict) -> set[str]:
    """Return the set of $defs keys defined in a schema."""
    return set(schema.get("$defs", {}).keys())


def _walk_refs(node, out: list[str], path: str = "") -> None:
    """Collect all $ref values in a schema tree."""
    if isinstance(node, dict):
        if "$ref" in node and isinstance(node["$ref"], str):
            out.append(node["$ref"])
        for key, val in node.items():
            _walk_refs(val, out, f"{path}/{key}")
    elif isinstance(node, list):
        for i, item in enumerate(node):
            _walk_refs(item, out, f"{path}[{i}]")


def validate_ref_integrity(schemas_dir: Path) -> list[str]:
    """Validate $ref integrity across every JSON file in schemas_dir (recursive).

    Schemas live in per-domain subdirectories (e.g. ``Schemas/detector/...``).
    A cross-file ref is resolved against the directory of the file that
    contains it and must land on a file inside ``schemas_dir``. Resolving by
    path, not by filename, is what catches ``../wrongdomain/common-v1.0.json``:
    the filename exists, the path does not.
    """
    errors: list[str] = []
    root = schemas_dir.resolve()
    files: dict[Path, dict] = {}
    first_seen: dict[str, Path] = {}
    for path in sorted(schemas_dir.rglob("*.json")):
        if path.name in first_seen:
            errors.append(
                f"duplicate filename '{path.name}' at '{path}' shadows "
                f"'{first_seen[path.name]}' — filenames must be globally unique"
            )
        else:
            first_seen[path.name] = path
        files[path.resolve()] = json.loads(path.read_text())

    for path, schema in files.items():
        label = str(path.relative_to(root))
        local_defs = _collect_defs(schema)
        refs: list[str] = []
        _walk_refs(schema, refs)

        for ref in refs:
            if ref.startswith("#/$defs/"):
                target = ref.removeprefix("#/$defs/")
                if target not in local_defs:
                    errors.append(f"{label}: local $ref to missing '{target}'")
            elif "#/$defs/" in ref:
                file_part, _, frag = ref.partition("#/$defs/")
                # $ref values are URI-style, so "/" is the only separator;
                # a backslash is a literal character and will not resolve.
                target_path = (path.parent / file_part).resolve()
                if target_path not in files:
                    errors.append(
                        f"{label}: cross-file $ref to non-existent file '{file_part}'"
                    )
                elif frag not in _collect_defs(files[target_path]):
                    errors.append(
                        f"{label}: cross-file $ref to missing '{file_part}#/$defs/{frag}'"
                    )
            else:
                errors.append(f"{label}: unrecognised $ref form '{ref}'")

    return errors


def validate_asyncapi(doc_path: Path, root: Path) -> list[str]:
    """Validate the internal and file references of the AsyncAPI document.

    ``root`` is the directory the document's relative payload references
    (``Schemas_asyncapi/...``) are written against.
    """
    errors: list[str] = []
    doc = yaml.safe_load(doc_path.read_text())
    components = doc.get("components", {})
    messages = components.get("messages", {})

    def component_exists(ref: str) -> bool:
        parts = ref.removeprefix("#/").split("/")
        node = doc
        for part in parts:
            if not isinstance(node, dict) or part not in node:
                return False
            node = node[part]
        return True

    refs: list[tuple[str, str]] = []

    def collect(node, where: str) -> None:
        if isinstance(node, dict):
            for k, v in node.items():
                if k == "$ref" and isinstance(v, str):
                    refs.append((where, v))
                else:
                    collect(v, f"{where}/{k}")
        elif isinstance(node, list):
            for i, item in enumerate(node):
                collect(item, f"{where}[{i}]")

    collect(doc, "")
    for where, ref in refs:
        if ref.startswith("#/"):
            if not component_exists(ref):
                errors.append(f"asyncapi.yaml{where}: $ref to missing '{ref}'")
        elif not (root / ref).is_file():
            errors.append(f"asyncapi.yaml{where}: $ref to non-existent file '{ref}'")
        elif not (root / ref).resolve().is_relative_to((root / "Schemas_asyncapi").resolve()):
            # An existing file is not enough: a payload must be one of the
            # AsyncAPI-flavoured schemas, not something the path wandered to.
            errors.append(
                f"asyncapi.yaml{where}: $ref '{ref}' resolves outside Schemas_asyncapi/"
            )

    for name, message in messages.items():
        if "payload" not in message:
            errors.append(f"asyncapi.yaml: message '{name}' has no payload")

    for name, channel in doc.get("channels", {}).items():
        for op in ("publish", "subscribe"):
            if op not in channel:
                errors.append(f"asyncapi.yaml: channel '{name}' has no {op} operation")
        rr = channel.get("x-amqp1-request-reply")
        if rr is not None:
            reply = rr.get("responseMessage", {}).get("$ref", "")
            if reply.removeprefix("#/components/messages/") not in messages:
                errors.append(
                    f"asyncapi.yaml: channel '{name}' names reply message '{reply}' that does not exist"
                )

    return errors


def main() -> int:
    errors: list[str] = []
    errors += [f"Schemas/{e}" for e in validate_ref_integrity(SCHEMAS_DIR)]
    errors += [f"Schemas_asyncapi/{e}" for e in validate_ref_integrity(ASYNCAPI_SCHEMAS_DIR)]
    errors += validate_asyncapi(ASYNCAPI_DOC, ROOT)
    if errors:
        print(f"FAIL: {len(errors)} broken reference(s):")
        for err in errors:
            print(f"  {err}")
        return 1
    n = len(list(SCHEMAS_DIR.rglob("*.json")))
    m = len(list(ASYNCAPI_SCHEMAS_DIR.rglob("*.json")))
    print(f"OK: all $refs resolve across {n} schema files, {m} AsyncAPI schema files and asyncapi.yaml.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
