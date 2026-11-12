# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-04
# Version:     1.21
# Description: The asyncapi_clients generator: per-language header rendering,
#              snippet template substitution and its error paths, the
# language, scenario and SDK registries,
#              and content-keyed _gen_date behaviour — hash match,
#              hash mismatch, unusable stamps, and independence from git
#              history.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for the asyncapi_clients generator."""
import json
from datetime import date

import pytest

from asyncapi_clients.generator import generate as orch
from asyncapi_clients.generator import headers, snippets


# ---- headers --------------------------------------------------------------

def test_python_header_uses_triple_quote_block():
    out = headers.build_header(
        relative_path="output/python/snippets/s1.py",
        language="python",
        gen_date=date(2026, 5, 2),
        sdk_generator="datamodel-code-generator@0.56.1",
        generator_version="0.1.0",
    )
    assert out.startswith('"""')
    assert out.rstrip().endswith('"""')
    assert "Author:      Christopher Kettell, TRL Software" in out
    assert "License:     Apache-2.0" in out
    assert "Decisions" not in out
    assert "datamodel-code-generator@0.56.1" in out
    assert "2026-05-02" in out


def test_c_family_header_uses_block_comment():
    for lang in ("java", "csharp", "cpp", "typescript", "rust"):
        out = headers.build_header(
            relative_path=f"output/{lang}/snippets/s1.x",
            language=lang,
            gen_date=date(2026, 5, 2),
            sdk_generator="asyncapi_clients/generator",
            generator_version="0.1.0",
            )
        assert out.startswith("/*"), f"{lang} should use /* */ block comment"
        assert out.rstrip().endswith("*/"), f"{lang} should close with */"
        assert " * Author:      Christopher Kettell, TRL Software" in out


def test_unknown_language_raises():
    with pytest.raises(ValueError, match="unknown language"):
        headers.build_header(
            relative_path="x", language="cobol", gen_date=date(2026, 5, 2),
            sdk_generator="x", generator_version="0.1.0",
            )


# ---- snippet rendering ----------------------------------------------------

def test_render_snippet_substitutes_context(tmp_path):
    tmpl_dir = tmp_path / "templates" / "python"
    tmpl_dir.mkdir(parents=True)
    (tmpl_dir / "s1.py.j2").write_text(
        "channel = {{ channels.detector_measurement | tojson }}\n"
        "server = {{ server_uri | tojson }}\n"
    )
    out = snippets.render(
        templates_root=tmp_path / "templates",
        language="python",
        snippet_id="s1",
        ext="py",
        context={
            "channels": {"detector_measurement": "detector.measurement"},
            "server_uri": "amqps://broker.example:5671",
        },
    )
    assert 'channel = "detector.measurement"' in out
    assert 'server = "amqps://broker.example:5671"' in out


def test_render_missing_template_raises(tmp_path):
    (tmp_path / "templates" / "python").mkdir(parents=True)
    with pytest.raises(FileNotFoundError):
        snippets.render(
            templates_root=tmp_path / "templates", language="python",
            snippet_id="s9", ext="py", context={},
        )


# ---- orchestrator constants ----------------------------------------------

def test_languages_registry():
    # Go was removed entirely (templates, generated snippets, smoke leg and
    # CI matrix); C++ survives as generated-only scaffolding, off the roadmap.
    assert orch.LANGUAGES == [
        "python", "java", "csharp", "cpp", "typescript", "rust",
    ]


def test_scenarios_registry():
    assert [s.id for s in orch.SCENARIOS] == ["s1", "s2", "s3", "s4", "s5", "s6", "s7"]


def test_sdk_languages_registry():
    # Java/C#/TypeScript come from quicktype and Rust from cargo-typify.
    # C++ has no SDK and is off the roadmap; Go was dropped entirely.
    assert orch.SDK_LANGUAGES == ["python", "java", "csharp", "typescript", "rust"]


# ---- _snippet_stem -----------------------------------------------------------

from asyncapi_clients.generator.generate import _snippet_stem  # noqa: E402


@pytest.mark.parametrize("lang,sid,expected", [
    ("java", "s1", "S1"),
    ("java", "s7", "S7"),
    ("python", "s1", "s1"),
    ("csharp", "s3", "s3"),
    ("rust", "s7", "s7"),
])
def test_snippet_stem(lang, sid, expected):
    assert _snippet_stem(lang, sid) == expected


# ---- _gen_date ---------------------------------------------------------------

import hashlib as _hashlib  # noqa: E402
import json as _json  # noqa: E402
from datetime import date as _date, datetime, timezone  # noqa: E402
from unittest.mock import patch  # noqa: E402

from asyncapi_clients.generator.generate import (  # noqa: E402
    _asyncapi_sha256,
    _gen_date,
)

_TODAY = datetime.now(timezone.utc).date()


@pytest.fixture
def stamp_env(tmp_path):
    """A throwaway asyncapi.yaml + stamp path wired into the generator."""
    yaml = tmp_path / "asyncapi.yaml"
    yaml.write_text("asyncapi: 2.6.0\n")
    stamp = tmp_path / "gen_stamp.json"
    with patch("asyncapi_clients.generator.generate.ASYNCAPI_YAML", yaml), \
         patch("asyncapi_clients.generator.generate.GEN_STAMP_PATH", stamp):
        yield yaml, stamp


def _write_stamp(stamp, yaml, when):
    stamp.write_text(_json.dumps(
        {"asyncapi_sha256": _hashlib.sha256(yaml.read_bytes()).hexdigest(),
         "date": when},
        indent=2,
    ) + "\n")


def test_gen_date_matching_hash_reuses_recorded_date(stamp_env):
    """The whole point: same bytes -> same stamp, whatever the git history."""
    yaml, stamp = stamp_env
    _write_stamp(stamp, yaml, "2026-04-01")
    before = stamp.read_text()
    assert _gen_date() == _date(2026, 4, 1)
    assert stamp.read_text() == before, "a matching stamp must not be rewritten"


def test_gen_date_is_independent_of_git_history(stamp_env):
    """No git call at all — that repo-scoped dependency is what broke the port."""
    yaml, stamp = stamp_env
    _write_stamp(stamp, yaml, "2026-04-01")
    with patch("asyncapi_clients.generator.generate.subprocess.run") as run:
        assert _gen_date() == _date(2026, 4, 1)
    run.assert_not_called()


def test_gen_date_changed_yaml_restamps_today_and_records_it(stamp_env):
    yaml, stamp = stamp_env
    _write_stamp(stamp, yaml, "2026-04-01")
    yaml.write_text("asyncapi: 2.6.0\n# edited\n")
    assert _gen_date() == _TODAY
    recorded = _json.loads(stamp.read_text())
    assert recorded["date"] == _TODAY.isoformat()
    assert recorded["asyncapi_sha256"] == _asyncapi_sha256()
    # Second call now finds a matching hash and is stable.
    assert _gen_date() == _TODAY


def test_gen_date_missing_stamp_bootstraps(stamp_env):
    yaml, stamp = stamp_env
    assert not stamp.exists()
    assert _gen_date() == _TODAY
    assert _json.loads(stamp.read_text())["asyncapi_sha256"] == _asyncapi_sha256()


@pytest.mark.parametrize("payload", [
    "{ not json",
    "[]",
    '{"asyncapi_sha256": "deadbeef", "date": "2026-04-01"}',
    '{"date": "2026-04-01"}',
    '{"asyncapi_sha256": null, "date": "not-a-date"}',
])
def test_gen_date_unusable_stamp_falls_back_to_today(stamp_env, payload):
    yaml, stamp = stamp_env
    stamp.write_text(payload)
    assert _gen_date() == _TODAY


def test_gen_date_malformed_date_with_matching_hash_falls_back(stamp_env):
    yaml, stamp = stamp_env
    _write_stamp(stamp, yaml, "not-a-date")
    assert _gen_date() == _TODAY


def test_gen_date_missing_asyncapi_yaml_falls_back_to_today(tmp_path):
    with patch("asyncapi_clients.generator.generate.ASYNCAPI_YAML",
               tmp_path / "absent.yaml"), \
         patch("asyncapi_clients.generator.generate.GEN_STAMP_PATH",
               tmp_path / "gen_stamp.json"):
        assert _gen_date() == _TODAY


def test_committed_stamp_matches_committed_asyncapi_yaml():
    """The shipped stamp must describe the shipped asyncapi.yaml.

    If this goes red, `make asyncapi-clients` was run without committing
    the regenerated stamp — the drift gate would fail next.
    """
    recorded = _json.loads(orch.GEN_STAMP_PATH.read_text())
    assert recorded["asyncapi_sha256"] == _asyncapi_sha256()
    _date.fromisoformat(recorded["date"])


# ---- quicktype helpers ------------------------------------------------

from asyncapi_clients.generator.generate import (  # noqa: E402
    QUICKTYPE_VERSION,
    _build_domain_composite,
    _def_names,
    _pascal,
    _strip_titles,
)


def test_quicktype_version_is_exactly_pinned():
    # quicktype output is stamped into every Java/C#/TS SDK file's header.
    # Drift between developer environments and CI breaks the drift workflow,
    # so the version must be exactly pinned (not a range).
    assert QUICKTYPE_VERSION == "23.0.171"


@pytest.mark.parametrize("snake,expected", [
    ("traffic_signal", "TrafficSignal"),
    ("car_park", "CarPark"),
    ("vms", "Vms"),
    ("air_quality_enhanced", "AirQualityEnhanced"),
    ("traffic_signal_2010", "TrafficSignal2010"),
    ("", ""),
])
def test_pascal_case_conversion(snake, expected):
    assert _pascal(snake) == expected


def test_def_names_returns_all_keys_in_order(tmp_path):
    """`_def_names` must return every `$defs` key, preserving insertion order
    (so each top-level type gets its own quicktype invocation in Java's
    per-schema-per-top-level mode —)."""
    schema = tmp_path / "x.json"
    schema.write_text(json.dumps({
        "$defs": {
            "AccessControlCommand": {"type": "object"},
            "DisableAccessCommand": {"type": "object"},
            "EnableAccessCommand": {"type": "object"},
        }
    }))
    assert _def_names(schema) == [
        "AccessControlCommand",
        "DisableAccessCommand",
        "EnableAccessCommand",
    ]


def test_def_names_handles_single_def(tmp_path):
    schema = tmp_path / "x.json"
    schema.write_text(json.dumps({"$defs": {"OnlyOne": {"type": "object"}}}))
    assert _def_names(schema) == ["OnlyOne"]


def test_def_names_handles_no_defs(tmp_path):
    schema = tmp_path / "x.json"
    schema.write_text(json.dumps({"type": "object"}))
    assert _def_names(schema) == []


def test_strip_titles_removes_nested_titles():
    """`_strip_titles` must recurse into nested objects and arrays. The
    composite builder calls this so quicktype names classes after `$defs`
    keys, not the original schema files' `title` fields."""
    node = {
        "title": "Outer",
        "properties": {
            "inner": {"title": "Inner", "type": "string"},
        },
        "oneOf": [
            {"title": "Variant1", "type": "object"},
            {"title": "Variant2", "type": "object"},
        ],
    }
    _strip_titles(node)
    assert "title" not in node
    assert "title" not in node["properties"]["inner"]
    assert "title" not in node["oneOf"][0]
    assert "title" not in node["oneOf"][1]
    # Non-title fields must be preserved
    assert node["properties"]["inner"]["type"] == "string"


def test_strip_titles_is_safe_on_non_dict_inputs():
    """Should accept lists, primitives, None without raising."""
    _strip_titles(None)
    _strip_titles("string")
    _strip_titles(42)
    _strip_titles([1, 2, 3])  # no-op


def test_build_domain_composite_lifts_defs_and_strips_titles(tmp_path):
    """End-to-end check: a domain dir with two schema files produces one
    composite whose `$defs` is the union of both, with titles stripped."""
    domain = tmp_path / "fake_domain"
    domain.mkdir()
    (domain / "fake_definition-v1.0.json").write_text(json.dumps({
        "title": "UTMC Fake Definition",
        "$defs": {
            "FakeDefinition": {
                "title": "Fake Definition",
                "type": "object",
                "properties": {"id": {"type": "string"}},
            }
        }
    }))
    (domain / "fake_command-v1.0.json").write_text(json.dumps({
        "title": "UTMC Fake Command",
        "$defs": {
            "FakeCommand": {
                "title": "Fake Command",
                "type": "object",
            },
            "FakeCommandKind": {"type": "string", "enum": ["a", "b"]},
        }
    }))

    composite = tmp_path / "_composite.json"
    count = _build_domain_composite(domain, composite)

    data = json.loads(composite.read_text())
    assert count == 3
    assert set(data["$defs"]) == {"FakeDefinition", "FakeCommand", "FakeCommandKind"}
    # Synthetic root has one property per def so quicktype walks them all
    assert set(data["properties"]) == {"FakeDefinition", "FakeCommand", "FakeCommandKind"}
    # Titles stripped at every depth
    assert "title" not in data["$defs"]["FakeDefinition"]
    assert "title" not in data["$defs"]["FakeCommand"]


def test_build_domain_composite_skips_underscore_prefixed_files(tmp_path):
    """Stale composite files left from a previous run (named `_composite.json`)
    must not be re-included. `_`-prefix is the convention used to mark them."""
    domain = tmp_path / "fake_domain"
    domain.mkdir()
    (domain / "real-v1.0.json").write_text(json.dumps({
        "$defs": {"Real": {"type": "object"}}
    }))
    (domain / "_composite.json").write_text(json.dumps({
        "$defs": {"StaleArtefact": {"type": "object"}}
    }))

    composite = tmp_path / "_composite.json"
    _build_domain_composite(domain, composite)

    data = json.loads(composite.read_text())
    assert "Real" in data["$defs"]
    assert "StaleArtefact" not in data["$defs"]
