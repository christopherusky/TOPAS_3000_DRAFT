# ──────────────────────────────────────────────────────────────────────
# Origin:     Cross-cutting CI helper for TOPAS 3000A
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Fail if the working tree under the given paths differs from the commit.

Used by the drift workflows after regenerating from the models. It reports
three kinds of difference, where ``git diff --exit-code`` reports only the
first:

* modified  - a committed file whose regenerated content differs;
* deleted   - a committed file the generators no longer produce;
* untracked - a file the generators now produce that was never committed.

``git diff`` ignores untracked files, so a newly generated schema or SDK file
that the author forgot to commit passed the old gate.

Usage: python bin/check_drift.py PATH [PATH ...]
"""

from __future__ import annotations

import subprocess
import sys


def drift(paths: list[str], cwd: str | None = None) -> list[str]:
    """Return one ``git status --porcelain`` line per differing file."""
    out = subprocess.run(
        ["git", "status", "--porcelain", "--untracked-files=all", "--", *paths],
        check=True,
        capture_output=True,
        text=True,
        cwd=cwd,
    ).stdout
    return [line for line in out.splitlines() if line.strip()]


def unmatched(paths: list[str], cwd: str | None = None) -> list[str]:
    """Return the paths that match no tracked and no untracked file.

    ``git status`` says nothing about a path that does not exist, so a
    misspelt path would read as "no drift" and the gate could never fail.
    """
    missing = []
    for path in paths:
        listed = subprocess.run(
            ["git", "ls-files", "--cached", "--others", "--exclude-standard", "--", path],
            check=True, capture_output=True, text=True, cwd=cwd,
        ).stdout.strip()
        if not listed:
            missing.append(path)
    return missing


def main(argv: list[str]) -> int:
    if not argv:
        print("usage: check_drift.py PATH [PATH ...]", file=sys.stderr)
        return 2
    missing = unmatched(argv)
    if missing:
        print(f"FAIL: no tracked or untracked file under: {' '.join(missing)}", file=sys.stderr)
        print("A path that matches nothing cannot show drift, so this is treated as an error.", file=sys.stderr)
        return 2
    lines = drift(argv)
    if lines:
        print(f"FAIL: {len(lines)} file(s) differ from the commit under {' '.join(argv)}:")
        for line in lines[:50]:
            print(f"  {line}")
        if len(lines) > 50:
            print(f"  ... and {len(lines) - 50} more")
        print("Regenerate locally and commit the result, including new files.")
        return 1
    print(f"OK: no drift under {' '.join(argv)}.")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
