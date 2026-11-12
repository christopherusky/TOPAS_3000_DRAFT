# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-29
# Version:     1.1
# Description: The tagged-union upgrade and the Kind collision
#: discriminator discovery, case-insensitive resolution
#              of emitted type names, failure rather than a dangling name on a
#              missing variant, and the emitted union shape per language - a
#              TypeScript alias, a C# interface plus converter, and a Java
#              holder with per-variant enums.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for the tagged-union upgrade and the Kind collision.

The SDKs' own test suites cover runtime behaviour but need a JDK, .NET or
Node. These run in the ordinary pytest suite and guard the structure of
the committed output, so a regression is caught even on a machine with
none of those toolchains.
"""
import json
import re
from pathlib import Path

import pytest

from asyncapi_clients.generator.generate import (
    UnionVariantMissing,
    _discriminated_unions,
    _resolve_emitted_names,
)

ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "asyncapi_clients" / "output"
SCHEMAS = ROOT / "Schemas"

UNION_DOMAINS = sorted(
    d.name for d in SCHEMAS.iterdir()
    if d.is_dir() and _discriminated_unions(d)
)


def test_the_model_still_has_unions_to_check():
    """A guard that guards nothing would pass silently forever."""
    assert len(UNION_DOMAINS) >= 5


# ── discovery ──

def test_discovery_reads_the_discriminator_block():
    unions = _discriminated_unions(SCHEMAS / "detector")
    assert set(unions) == {"DetectorCommand", "DetectorMeasurement"}
    cmd = unions["DetectorCommand"]
    assert cmd["property"] == "kind"
    assert cmd["mapping"]["calibrate"] == "CalibrateDetectorCommand"


def test_every_union_discriminates_on_kind():
    for domain in UNION_DOMAINS:
        for name, spec in _discriminated_unions(SCHEMAS / domain).items():
            assert spec["property"] == "kind", f"{domain}/{name}"
            assert len(spec["mapping"]) >= 1, f"{domain}/{name}"


# ── name resolution ──

def test_names_are_resolved_case_insensitively():
    """quicktype renames acronyms: TransportLinkANPRDynamic -> ...Anpr..."""
    text = "export interface TransportLinkAnprDynamic {"
    got = _resolve_emitted_names(
        text, r"export interface (\w+)",
        {"anpr": "TransportLinkANPRDynamic"}, "ctx",
    )
    assert got == {"anpr": "TransportLinkAnprDynamic"}


def test_a_missing_variant_raises_rather_than_emitting_a_dangling_name():
    with pytest.raises(UnionVariantMissing):
        _resolve_emitted_names(
            "export interface Something {", r"export interface (\w+)",
            {"x": "NotEmitted"}, "ctx",
        )


# ── committed output ──

@pytest.mark.parametrize("domain", UNION_DOMAINS)
def test_typescript_emits_a_real_union(domain):
    text = (OUT / "typescript" / "sdk" / f"{domain}.ts").read_text()
    for name in _discriminated_unions(SCHEMAS / domain):
        # A `type X = A | B` alias, not an `interface X` carrying every
        # variant's fields — which is what quicktype produces unaided.
        assert re.search(rf"export type {name} =\s*\n\s*\|", text), name
        assert f"export interface {name} {{" not in text, name


@pytest.mark.parametrize("domain", UNION_DOMAINS)
def test_csharp_emits_an_interface_and_converter(domain):
    pascal = "".join(p.title() for p in domain.split("_"))
    text = (OUT / "csharp" / "sdk" / f"{pascal}.cs").read_text()
    for name in _discriminated_unions(SCHEMAS / domain):
        assert f"public interface I{name} {{ }}" in text, name
        assert f"internal class {name}Converter : JsonConverter" in text, name
        # Newtonsoft inherits [JsonConverter] from the interface onto the
        # variants, so writing through the converter recurses. CanWrite
        # must stay false.
        assert "public override bool CanWrite => false;" in text, name


@pytest.mark.parametrize("domain", UNION_DOMAINS)
def test_java_emits_a_union_holder(domain):
    pkg = OUT / "java" / "sdk" / "com" / "topas" / "utmc" / domain
    for name, spec in _discriminated_unions(SCHEMAS / domain).items():
        text = (pkg / f"{name}.java").read_text()
        assert f"public class {name} {{" in text, name
        assert "JsonDeserialize(using" in text, name
        for variant in set(spec["mapping"].values()):
            assert f"as{variant}()" in text or variant.lower() in text.lower(), variant


@pytest.mark.parametrize("domain", UNION_DOMAINS)
def test_java_variant_enums_carry_their_own_wire_value(domain):
    """Tested as the invariant rather than as a proxy.

    Every discriminator enum used to be generated as `Kind` from the
    property name and written to one Kind.java per package, so in a domain
    with two discriminated types the last one won and the other could not
    be deserialised at all — `Cannot deserialize Kind`.

    A lone `Kind.java` is not itself the bug: a domain with a single
    discriminated type has nothing to collide with. What must hold is that
    each variant's enum actually contains that variant's wire value, which
    is what the collision destroyed.
    """
    pkg = OUT / "java" / "sdk" / "com" / "topas" / "utmc" / domain
    # quicktype normalises acronyms (TransportLinkANPRConfiguration ->
    # ...Anpr...), so resolve the emitted name the way the generator does.
    on_disk = {re.sub(r"[^a-z0-9]", "", q.stem.lower()): q for q in pkg.glob("*.java")}
    for name, spec in _discriminated_unions(SCHEMAS / domain).items():
        for wire_value, variant in spec["mapping"].items():
            key = re.sub(r"[^a-z0-9]", "", variant.lower())
            assert key in on_disk, f"{domain}: no class for {variant}"
            variant_src = on_disk[key].read_text()
            m = re.search(r"public (\w+) getKind\(\)", variant_src)
            assert m, f"{domain}/{variant}: no getKind()"
            enum_key = re.sub(r"[^a-z0-9]", "", m.group(1).lower())
            assert enum_key in on_disk, f"{domain}: no enum {m.group(1)}"
            enum_src = on_disk[enum_key].read_text()
            assert f'"{wire_value}"' in enum_src, (
                f"{domain}/{variant}: enum {m.group(1)} has no "
                f"{wire_value!r} — it was overwritten by another type"
            )


def test_java_discriminator_enums_are_per_variant():
    pkg = OUT / "java" / "sdk" / "com" / "topas" / "utmc" / "detector"
    names = {p.stem for p in pkg.glob("*Kind.java")}
    # Both a command variant and a measurement variant keep their own.
    assert "ResetDetectorCommandKind" in names
    assert "FlowDynamicKind" in names


def test_stub_tests_are_shipped_and_not_stamped_as_generated():
    for path in (
        OUT / "java" / "sdk" / "tests" / "TaggedUnionTest.java",
        OUT / "typescript" / "sdk" / "tests" / "unions.ts",
    ):
        header = path.read_text()[:600]
        assert "Hand-written" in header, path
        assert "Generated from asyncapi.yaml" not in header, path


# ── C# wire fidelity ──

@pytest.mark.parametrize("domain", sorted(
    d.name for d in SCHEMAS.iterdir() if d.is_dir() and d.name != "common"
))
def test_csharp_carries_wire_names_and_a_public_entrypoint(domain):
    """`just-types-and-namespace` stripped both, so the SDK could not
    produce valid wire JSON: PascalCase property names and enum members
    serialised as their C# names or ordinals."""
    pascal = "".join(p.title() for p in domain.split("_"))
    text = (OUT / "csharp" / "sdk" / f"{pascal}.cs").read_text()
    # [JsonProperty] fixes the property names, and applies with or without
    # serializer settings.
    assert '[JsonProperty("' in text, domain
    # The enum converters are registered in an *internal* Converter class,
    # so a consumer in another assembly needs this accessor to reach them.
    # Without it, `kind` serialises as an ordinal.
    assert "public static class UtmcJson" in text, domain
    assert "Settings => Converter.Settings" in text, domain


def test_csharp_enum_converters_map_to_wire_values():
    text = (OUT / "csharp" / "sdk" / "Detector.cs").read_text()
    assert 'if (value == "calibrate")' in text
    assert "class CalibrateDetectorCommandKindConverter" in text
