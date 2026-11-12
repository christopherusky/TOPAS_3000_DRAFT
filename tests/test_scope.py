# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-04
# Version:     1.0
# Description: scope.toml: the phase each domain belongs to is validated
#              against the packages on disk, the live emit contains no
#              deferred domain, the guard refuses a non-live phase into
#              Schemas/, and the smoke scenarios point at live domains.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for the live-phase scope."""

import json
from pathlib import Path

import pytest

from bin import emit
from bin.scope import ScopeError, check_against_disk, load_scope, packages_on_disk

ROOT = Path(__file__).resolve().parent.parent
SCOPE = load_scope()
DEFERRED = SCOPE.deferred(SCOPE.live_phase)


def test_every_package_on_disk_is_classified_and_nothing_else():
    assert check_against_disk(SCOPE) == []
    listed = set(SCOPE.domains) | set(SCOPE.libraries)
    assert listed == set(packages_on_disk())


def test_there_are_deferred_domains():
    """A scope that defers nothing would make every other test vacuous."""
    assert DEFERRED


def test_live_specs_exclude_every_deferred_domain():
    live = {s.domain for s in emit.specs_for_phase() if s.domain}
    assert live.isdisjoint(set(DEFERRED))
    assert len(emit.specs_for_phase()) < len(emit.ALL_FILE_SPECS)


def test_phases_are_cumulative():
    """Each phase's domains include every earlier phase's, and each step adds some."""
    domains = [
        {s.domain for s in emit.specs_for_phase(p) if s.domain}
        for p in range(1, SCOPE.live_phase + 1)
    ]
    for earlier, later in zip(domains, domains[1:]):
        assert earlier < later


def test_emitted_schemas_dir_has_no_deferred_domain():
    present = {d.name for d in (ROOT / "Schemas").iterdir() if d.is_dir()}
    assert present.isdisjoint(set(DEFERRED))


def test_asyncapi_has_no_deferred_channel_message_or_schema():
    import yaml

    doc = yaml.safe_load((ROOT / "asyncapi.yaml").read_text())
    for d in DEFERRED:
        kebab, snake = d.replace("_", "-"), d
        assert not [c for c in doc["channels"] if c.split(".")[0] == kebab], d
        text = json.dumps(doc["components"])
        assert f"{snake}_" not in text, d
        assert not (ROOT / "Schemas_asyncapi" / snake).exists(), d


def test_guard_refuses_a_non_live_phase_into_the_default_directory():
    with pytest.raises(ScopeError):
        emit.emit_all(SCOPE.live_phase + 1)


def test_preview_of_all_phases_into_a_scratch_directory(tmp_path):
    assert emit.main(["--phase", str(SCOPE.live_phase + 1), "--output-dir", str(tmp_path)]) == 0
    assert {d.name for d in tmp_path.iterdir() if d.is_dir()} >= set(DEFERRED)
    assert len(list(tmp_path.rglob("*.json"))) == len(emit.ALL_FILE_SPECS)


def test_smoke_scenarios_target_live_domains():
    live_dirs = {d.name.replace("_", "-") for d in (ROOT / "Schemas").iterdir()}
    channel = SCOPE.smoke["sync_delta_channel"]
    assert channel.split(".")[0].replace("_", "-") in live_dirs
    for d in SCOPE.smoke["fault_domains"]:
        assert d.replace("_", "-") in live_dirs


def test_deferred_packages_carry_the_visible_marker():
    for d in DEFERRED:
        doc = (ROOT / "topas_3000_model" / d / "__init__.py").read_text()
        assert "Deferred to Phase 4" in doc, d


def test_a_domain_missing_from_scope_is_rejected(tmp_path):
    bad = tmp_path / "scope.toml"
    bad.write_text('live_phase = 3\n[domains]\ndetector = 1\n[libraries]\nalways = ["common"]\n'
                   '[smoke]\nsync_delta_channel = "detector.measurement"\nfault_domains = ["detector"]\n')
    problems = check_against_disk(load_scope(bad))
    assert any("is not in scope.toml" in p for p in problems)


@pytest.mark.parametrize("smoke", [
    '[smoke]\nsync_delta_channel = "car-park.measurement"\nfault_domains = ["detector"]\n',
    '[smoke]\nsync_delta_channel = "detector.measurement"\nfault_domains = ["cctv"]\n',
    '[smoke]\nsync_delta_channel = "detector.measurement"\n',
    '[smoke]\nsync_delta_channel = "nodot"\nfault_domains = ["detector"]\n',
])
def test_smoke_section_must_name_live_domains(tmp_path, smoke):
    bad = tmp_path / "scope.toml"
    bad.write_text('live_phase = 3\n[domains]\ndetector = 1\ncar_park = 4\ncctv = 4\n'
                   '[libraries]\nalways = ["common"]\n' + smoke)
    with pytest.raises(ScopeError):
        load_scope(bad)


def test_libraries_must_be_a_table(tmp_path):
    bad = tmp_path / "scope.toml"
    bad.write_text('live_phase = 3\nlibraries = "common"\n[domains]\ndetector = 1\n')
    with pytest.raises(ScopeError):
        load_scope(bad)
