# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.2
# Description: Snapshot coverage: every schema emit.py produces is compared
#              against the committed copy under Schemas/, so regeneration
#              cannot silently change the published contract.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Snapshot tests: emit.py output must remain semantically identical to committed schemas.

Compares parsed dicts (not raw bytes) — formatting changes in the JSON output
do not trip this test, only structural drift. Byte-level idempotency is
guaranteed separately by tests/test_emit.py::test_emit_all_is_idempotent.
"""

import json
from pathlib import Path

from bin.emit import FILE_SPECS, build_file_schema
from bin.emit import _resolve_domain

SCHEMAS_DIR = Path(__file__).resolve().parent.parent / "Schemas"


def test_every_schema_matches_committed():
    """Re-running emit.py produces structurally-identical JSON to what's committed.

    If this fails: run `make schemas` to regenerate and commit the updated
    output in the same PR as the model change.
    """
    for spec in FILE_SPECS:
        fresh = build_file_schema(spec)
        committed_path = SCHEMAS_DIR / _resolve_domain(spec) / spec.filename
        committed = json.loads(committed_path.read_text())
        assert fresh == committed, (
            f"{spec.filename} drift: regenerated output differs from committed. "
            f"Run `make schemas` and commit the updated file."
        )
