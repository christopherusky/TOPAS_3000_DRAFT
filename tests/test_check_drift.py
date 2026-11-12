# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.0
# Description: bin/check_drift.py: a clean tree passes, and a modified, a
#              deleted and an untracked file are each caught. The untracked
#              case is the one `git diff --exit-code` misses; and a path that
#              matches nothing is an error, not a pass.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for bin/check_drift.py."""

import subprocess
from pathlib import Path

import pytest

from bin.check_drift import drift, main, unmatched


def _git(repo: Path, *args: str) -> None:
    subprocess.run(
        ["git", "-c", "user.email=t@example.invalid", "-c", "user.name=t", *args],
        cwd=repo, check=True, capture_output=True,
    )


@pytest.fixture()
def repo(tmp_path: Path) -> Path:
    _git(tmp_path, "init", "-q")
    (tmp_path / "gen").mkdir()
    (tmp_path / "gen" / "a.json").write_text("{}\n")
    (tmp_path / "other.txt").write_text("x\n")
    _git(tmp_path, "add", ".")
    _git(tmp_path, "commit", "-q", "-m", "init")
    return tmp_path


def test_clean_tree_passes(repo):
    assert drift(["gen"], cwd=str(repo)) == []


def test_modified_file_is_caught(repo):
    (repo / "gen" / "a.json").write_text('{"changed": true}\n')
    assert any("a.json" in l for l in drift(["gen"], cwd=str(repo)))


def test_deleted_file_is_caught(repo):
    (repo / "gen" / "a.json").unlink()
    assert any("a.json" in l for l in drift(["gen"], cwd=str(repo)))


def test_untracked_file_is_caught(repo):
    (repo / "gen" / "new.json").write_text("{}\n")
    assert any("new.json" in l for l in drift(["gen"], cwd=str(repo)))


def test_untracked_file_in_a_new_subdirectory_is_caught(repo):
    (repo / "gen" / "sub").mkdir()
    (repo / "gen" / "sub" / "deep.json").write_text("{}\n")
    assert any("deep.json" in l for l in drift(["gen"], cwd=str(repo)))


def test_git_diff_alone_misses_the_untracked_file(repo):
    """The reason this script exists: the old gate would have passed."""
    (repo / "gen" / "new.json").write_text("{}\n")
    old_gate = subprocess.run(
        ["git", "diff", "--exit-code", "--", "gen"], cwd=repo, capture_output=True
    )
    assert old_gate.returncode == 0
    assert drift(["gen"], cwd=str(repo)) != []


def test_paths_outside_the_gate_are_ignored(repo):
    (repo / "other.txt").write_text("changed\n")
    assert drift(["gen"], cwd=str(repo)) == []


def test_a_path_that_matches_nothing_is_reported(repo):
    assert unmatched(["gen", "gne"], cwd=str(repo)) == ["gne"]
    assert unmatched(["gen"], cwd=str(repo)) == []


def test_an_untracked_only_path_still_counts_as_matching(repo):
    (repo / "fresh").mkdir()
    (repo / "fresh" / "a.json").write_text("{}\n")
    assert unmatched(["fresh"], cwd=str(repo)) == []


def test_main_fails_on_a_misspelt_path(repo, monkeypatch, capsys):
    """The old behaviour printed OK for a path that does not exist."""
    monkeypatch.chdir(repo)
    assert main(["gne"]) == 2
    assert "no tracked or untracked file" in capsys.readouterr().err


def test_main_passes_on_a_clean_real_path(repo, monkeypatch):
    monkeypatch.chdir(repo)
    assert main(["gen"]) == 0
