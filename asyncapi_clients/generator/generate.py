# ──────────────────────────────────────────────────────────────────────
# Origin:     Hand-written. Orchestrator for the asyncapi-clients generator.
# Changes:    n/a
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-05-02
# Version:    1.0
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Orchestrator for the asyncapi-clients generator.

Steps:
  1. Python SDK generation via datamodel-code-generator (Python only).
  2. Java / C# / TypeScript SDK generation via pinned quicktype.
  3. Jinja snippet rendering per (language, scenario) for all 7 languages.
  4. File-header injection over all emitted files.
  5. Per-language README rendering.

Multi-language SDKs were first deferred because Modelina rejected the
shared `extensions` field's `additionalProperties: true`. The work was
revisited with quicktype, which handles that case correctly. Go/C++/Rust SDKs followed later.

CLI flags:
  --only-lang <name>   limit snippet/README rendering to one language
  --only-step <name>   one of: python_sdk, java_sdk, csharp_sdk,
                       typescript_sdk, snippets, headers, readme
"""
from __future__ import annotations

import argparse
import hashlib
import json
import re
import shutil
import subprocess
import sys
import tempfile
from contextlib import contextmanager
from dataclasses import dataclass
from datetime import date, datetime, timezone
from importlib import metadata
from pathlib import Path

from asyncapi_clients.generator import __version__ as GENERATOR_VERSION
from asyncapi_clients.generator import headers, snippets
from bin.scope import load_scope

ROOT = Path(__file__).resolve().parents[2]
ASYNCAPI_CLIENTS_DIR = ROOT / "asyncapi_clients"
TEMPLATES_ROOT = ASYNCAPI_CLIENTS_DIR / "templates"
OUTPUT_ROOT = ASYNCAPI_CLIENTS_DIR / "output"
CANONICAL_SCHEMAS_DIR = ROOT / "Schemas"
ASYNCAPI_YAML = ROOT / "asyncapi.yaml"
GEN_STAMP_PATH = Path(__file__).resolve().parent / "gen_stamp.json"

LANGUAGES = ["python", "java", "csharp", "cpp", "typescript", "rust"]

LANG_EXT = {
    "python": "py",
    "java": "java",
    "csharp": "cs",
    "cpp": "cpp",
    "typescript": "ts",
    "go": "go",
    "rust": "rs",
}


# Languages that get a generated typed-class SDK. Python via
# datamodel-code-generator; Java/C#/TypeScript via pinned
# quicktype; Rust via pinned cargo-typify.
# C++ has no SDK and is off the roadmap; Go was dropped.
SDK_LANGUAGES = ["python", "java", "csharp", "typescript", "rust"]


@dataclass(frozen=True)
class Scenario:
    id: str
    description: str


SCENARIOS = [
    Scenario("s1", "Subscribe-and-print one telemetry stream (detector.measurement)"),
    Scenario("s2", "Wildcard subscription across a domain (*.fault)"),
    Scenario("s3", "Send a command, await its response (vms.command -> vms.command-response)"),
    Scenario("s4", "Sync-vs-delta consumer (the [smoke] sync_delta_channel in scope.toml)"),
    Scenario("s5", "Traffic signal dynamic state (traffic-signal.measurement)"),
    Scenario("s6", "Transport link discriminated-union consumer (transport-link.measurement)"),
    Scenario("s7", "Network topology consumer — node/link/zone/path/turn"),
]


def _asyncapi_sha256() -> str | None:
    """SHA-256 of asyncapi.yaml's bytes, or None if it cannot be read."""
    try:
        return hashlib.sha256(ASYNCAPI_YAML.read_bytes()).hexdigest()
    except OSError:
        return None


def _gen_date() -> date:
    """Date for header stamps — keyed to asyncapi.yaml's *content*.

    `gen_stamp.json` records the hash of the asyncapi.yaml that the
    committed output was generated from, alongside the date that
    generation happened. When the hash still matches, that date is
    reused verbatim; when it does not, the inputs really have changed,
    so today's UTC date is stamped and recorded.

    Content, not git history. The previous implementation read
    asyncapi.yaml's commit date, which is a property of the repository
    rather than of the spec: porting the tree into a second repo re-dated
    the YAML there and every header regenerated differently, failing the
    drift gate with nothing changed. A hash travels with the content, so
    any checkout of the same bytes — any repo, any history, shallow or
    full clone, no git at all — stamps the same date.

    Falls back to today when the stamp is missing, unreadable, malformed,
    or describes different bytes; each of those means the recorded date
    cannot be trusted to describe this asyncapi.yaml.
    """
    digest = _asyncapi_sha256()
    recorded = _read_gen_stamp()
    if digest is not None and recorded is not None:
        stamped_date, stamped_hash = recorded
        if stamped_hash == digest:
            return stamped_date
    today = datetime.now(timezone.utc).date()
    if digest is not None:
        _write_gen_stamp(digest, today)
    return today


def _read_gen_stamp() -> tuple[date, str] | None:
    """(date, hash) from gen_stamp.json, or None if it is unusable."""
    try:
        raw = json.loads(GEN_STAMP_PATH.read_text())
    except (OSError, ValueError):
        return None
    if not isinstance(raw, dict):
        return None
    stamped_hash = raw.get("asyncapi_sha256")
    if not isinstance(stamped_hash, str):
        return None
    try:
        return date.fromisoformat(raw.get("date", "")), stamped_hash
    except (TypeError, ValueError):
        return None


def _write_gen_stamp(digest: str, when: date) -> None:
    """Record the hash/date pair. Committed alongside the regenerated output."""
    GEN_STAMP_PATH.write_text(json.dumps(
        {"asyncapi_sha256": digest, "date": when.isoformat()}, indent=2,
    ) + "\n")


def _datamodel_codegen_version() -> str:
    return metadata.version("datamodel-code-generator")


# Pinned quicktype version for multi-language SDK generation.
# Bump in lockstep with a new DC entry.
QUICKTYPE_VERSION = "23.0.171"


def _quicktype(
    *,
    lang: str,
    src: Path,
    dest: Path,
    top_level: str,
    extra: list[str] | None = None,
) -> None:
    """Invoke pinned quicktype to generate one SDK file from one schema."""
    cmd = [
        "npx", "-y", f"quicktype@{QUICKTYPE_VERSION}",
        "--src-lang", "schema",
        "--lang", lang,
        "--top-level", top_level,
        "--out", str(dest),
    ]
    if extra:
        cmd.extend(extra)
    cmd.append(str(src))
    subprocess.run(cmd, check=True, cwd=ROOT)


def _def_names(schema_path: Path) -> list[str]:
    """Return every `$defs` key from a canonical schema file.

    Some schemas declare multiple top-level types in one file (e.g.
    discriminated-union variants like AccessControlCommand +
    DisableAccessCommand + EnableAccessCommand). Each gets its own
    quicktype invocation so every declared type ships in the SDK.
    """
    data = json.loads(schema_path.read_text())
    return list(data.get("$defs", {}))


def _pascal(snake: str) -> str:
    return "".join(part.capitalize() for part in snake.split("_"))


def _build_template_context() -> dict:
    """Shared context dict for all snippet templates."""
    return {
        "server_uri": "amqps://{host}:{port}",
        "channels": {
            "detector_measurement": "detector.measurement",
            "wildcard_fault": "*.fault",
            "vms_command": "vms.command",
            "vms_command_response": "vms.command-response",
            # Read from scope.toml so S4 runs against a live domain.
            "sync_delta_measurement": load_scope().smoke["sync_delta_channel"],
            "traffic_signal_measurement": "traffic-signal.measurement",
            "transport_link_measurement": "transport-link.measurement",
            "network_node": "network.node",
            "network_link": "network.link",
            "network_zone": "network.zone",
            "network_path": "network.path",
            "network_turn": "network.turn",
        },
        "headers": {
            "originator": "x-originator",
            "message_mode": "x-message-mode",
            "sync_id": "x-sync-id",
            "sync_sequence": "x-sync-sequence",
            "sync_total": "x-sync-total",
        },
    }


def _downgrade_prefix_items(node) -> None:
    """Rewrite Draft 2020-12 tuple validation into the pre-2020-12 form.

    quicktype's JSON Schema support predates `prefixItems`, so a tuple such
    as GeoJSON's ``[longitude, latitude]`` carries no inferable element type
    and is emitted untyped — ``Object[]`` in Java, ``object[]`` in C#,
    ``any[]`` in TypeScript. Where every prefix entry declares the same
    schema, collapse it to ``items`` so quicktype produces ``double[]``.

    ``minItems``/``maxItems`` already carry the arity, so nothing is lost for
    code generation. This runs only on the throwaway copy handed to
    quicktype — the canonical Schemas/ tree keeps its 2020-12 form.
    """
    if isinstance(node, dict):
        prefix = node.get("prefixItems")
        if isinstance(prefix, list) and prefix and "items" not in node:
            distinct = {json.dumps(entry, sort_keys=True) for entry in prefix}
            if len(distinct) == 1:
                node["items"] = json.loads(distinct.pop())
                node.pop("prefixItems")
        for value in node.values():
            _downgrade_prefix_items(value)
    elif isinstance(node, list):
        for value in node:
            _downgrade_prefix_items(value)


@contextmanager
def _quicktype_schema_tree():
    """Yield a temp mirror of Schemas/ with tuple validation downgraded.

    The layout is preserved so cross-file `$ref ../common/common-v1.0.json`
    still resolves. Removed on exit.
    """
    tmp = Path(tempfile.mkdtemp(prefix="utmc-quicktype-"))
    mirror = tmp / "Schemas"
    try:
        for src in CANONICAL_SCHEMAS_DIR.rglob("*.json"):
            data = json.loads(src.read_text())
            _downgrade_prefix_items(data)
            dst = mirror / src.relative_to(CANONICAL_SCHEMAS_DIR)
            dst.parent.mkdir(parents=True, exist_ok=True)
            dst.write_text(json.dumps(data, indent=2))
        yield mirror
    finally:
        shutil.rmtree(tmp, ignore_errors=True)


def _strip_titles(node) -> None:
    """Recursively strip `title` fields so quicktype names classes after
    `$defs` keys (composite-mode helper)."""
    if isinstance(node, dict):
        node.pop("title", None)
        for v in node.values():
            _strip_titles(v)
    elif isinstance(node, list):
        for v in node:
            _strip_titles(v)


def _discriminated_unions(domain_dir: Path) -> dict:
    """Find the `oneOf` types that carry a JSON Schema `discriminator`.

    Every union in this model discriminates on a `kind` property whose
    variants each pin it with a `const`. Returns
    ``{UnionName: {"property": ..., "mapping": {wire_value: VariantName},
    "description": ...}}``.
    """
    unions: dict[str, dict] = {}
    for path in sorted(domain_dir.glob("*.json")):
        if path.name.startswith("_"):
            continue
        for name, body in json.loads(path.read_text()).get("$defs", {}).items():
            disc = isinstance(body, dict) and body.get("discriminator")
            if not disc:
                continue
            unions[name] = {
                "property": disc["propertyName"],
                "mapping": {
                    value: ref.rsplit("/", 1)[-1]
                    for value, ref in sorted(disc.get("mapping", {}).items())
                },
                "description": body.get("description", ""),
            }
    return unions


def _build_domain_composite(domain_dir: Path, dest: Path,
                            strip_titles: bool = True,
                            exclude: set[str] | None = None) -> int:
    """Lift every `$defs` entry from every schema in a domain into one
    synthetic JSON Schema. The synthetic root carries one property per
    $defs key so quicktype walks them all. Written next to the source
    directory so relative `$ref ../common/common-v1.0.json` resolves.
    Returns the count of unique $defs entries combined.

    `strip_titles` is a quicktype accommodation — it names classes after
    `$defs` keys only when titles are absent. typify does the opposite and
    uses `title` to name and *share* a type, which collapses the per-field
    newtype wrappers (`DetectorDefinitionDetectorId`,
    `ResetDetectorCommandDetectorId`, ...) into one `ObjectId`. That is
    only safe because every titled string type in this model carries the
    same constraints wherever it appears; a divergence would produce two
    incompatible types with one name, so the Rust step checks for it."""
    combined: dict = {}
    skip = exclude or set()
    for p in sorted(domain_dir.glob("*.json")):
        if p.name.startswith("_"):
            continue
        data = json.loads(p.read_text())
        for k, v in data.get("$defs", {}).items():
            if k in combined or k in skip:
                continue
            if strip_titles:
                _strip_titles(v)
            combined[k] = v
    root = {
        "$schema": "https://json-schema.org/draft/2020-12/schema",
        "type": "object",
        "title": "_AllTypes",
        "properties": {k: {"$ref": f"#/$defs/{k}"} for k in combined},
        "$defs": combined,
    }
    dest.write_text(json.dumps(root, indent=2))
    return len(combined)


class UnionVariantMissing(RuntimeError):
    """A union variant has no counterpart in the generated output."""


def _resolve_emitted_names(text: str, decl_pattern: str,
                           wanted: dict, context: str) -> dict:
    """Map schema `$defs` names onto the names quicktype actually emitted.

    quicktype normalises acronyms — `TransportLinkANPRConfiguration`
    becomes `TransportLinkAnprConfiguration` — and the rules differ by
    language. Rather than reimplement them and drift, the emitted
    declarations are read back and matched case-insensitively.

    Raises rather than emitting a reference to a type that does not exist:
    a union naming a missing variant is a compile error at best and a
    silently absent variant at worst.
    """
    declared = re.findall(decl_pattern, text)
    index = {re.sub(r"[^a-z0-9]", "", d.lower()): d for d in declared}
    resolved: dict = {}
    for value, variant in wanted.items():
        key = re.sub(r"[^a-z0-9]", "", variant.lower())
        if key not in index:
            raise UnionVariantMissing(
                f"{context}: variant {variant!r} was not emitted "
                f"(looked for {key!r} among {len(index)} declarations)"
            )
        resolved[value] = index[key]
    return resolved


# ── Tagged unions ────────────────────────────────────────────
# quicktype flattens a discriminated `oneOf` into a single type carrying
# every variant's fields, with the extras optional. That cannot mis-route
# a message, but it cannot express the union either: nothing stops a
# caller building a "reset" command that also carries threshold values,
# and a consumer gets no exhaustiveness checking. The variant types are
# emitted by quicktype as usual; these functions add the union on top,
# in whatever form is idiomatic for the language.


def _append_blocks(dest: Path, renderers: list) -> None:
    """Append rendered blocks to a just-generated SDK file.

    Names are resolved against the file as generated, so the text is read
    once up front and every block rendered from that same snapshot.
    """
    if not renderers:
        return
    text = dest.read_text()
    blocks = [render(text) for render in renderers]
    dest.write_text(text + "\n" + "\n".join(blocks))


def _csharp_post_emit(domain: str, dest: Path, unions: dict) -> None:
    """Append the tagged unions and the public serializer entry point."""
    namespace = f"Topas.Utmc.{_pascal(domain)}"
    renderers = [
        lambda text, name=name, spec=spec: _csharp_union(
            namespace, name, spec,
            _resolve_emitted_names(
                text, r"public partial class (\w+)", spec["mapping"],
                f"csharp/{domain}/{name}",
            ),
        )
        for name, spec in sorted(unions.items())
    ]
    # Every domain gets the entry point, unions or not.
    renderers.append(lambda _text: _csharp_json_entrypoint(namespace))
    _append_blocks(dest, renderers)


def _ts_union(name: str, spec: dict, mapping: dict) -> str:
    """A native TypeScript discriminated union.

    Each variant already pins `kind` to a single-member enum, so the
    compiler narrows on it and checks exhaustiveness in a switch.
    """
    variants = sorted(set(mapping.values()))
    lines = [f"/**", f" * {spec['description']}".rstrip(),
             " *",
             f" * Discriminated on `{spec['property']}`. Narrow with a switch to have",
             " * the compiler check exhaustiveness.",
             " */",
             f"export type {name} =", ]
    lines += [f"    | {v}" for v in variants]
    lines[-1] += ";"
    return "\n".join(lines) + "\n"


def _csharp_json_entrypoint(namespace: str) -> str:
    """Expose quicktype's converter registration to other assemblies.

    `attributes-only` emits `internal static class Converter` holding the
    JsonSerializerSettings that register every generated enum converter.
    Being internal, a consumer outside this assembly cannot reach it — and
    without those settings enum members serialise as their C# names rather
    than their wire values. This accessor lives in the same assembly, so
    it can read the internal class and hand the settings out.
    """
    return "\n".join([
        f"namespace {namespace}",
        "{",
        "    using Newtonsoft.Json;",
        "",
        "    /// <summary>",
        "    /// Serializer settings for this namespace's generated types.",
        "    /// </summary>",
        "    /// <remarks>",
        "    /// Always serialise and deserialise through these settings.",
        "    /// They register the converters that map enum members to their",
        "    /// wire values; without them a discriminator round-trips as",
        '    /// <c>"Calibrate"</c> rather than <c>"calibrate"</c> and the',
        "    /// payload no longer validates against the schema.",
        "    /// </remarks>",
        "    public static class UtmcJson",
        "    {",
        "        public static JsonSerializerSettings Settings => Converter.Settings;",
        "",
        "        public static T Deserialize<T>(string json) =>",
        "            JsonConvert.DeserializeObject<T>(json, Converter.Settings);",
        "",
        "        public static string Serialize(object value) =>",
        "            JsonConvert.SerializeObject(value, Converter.Settings);",
        "    }",
        "}",
    ]) + "\n"


def _csharp_union(namespace: str, name: str, spec: dict, mapping: dict) -> str:
    """A C# interface plus a Newtonsoft converter that dispatches on the tag.

    quicktype emits every class as `partial`, so the variants can be
    retro-fitted with the marker interface from a separate file without
    touching generated output.
    """
    prop = spec["property"]
    variants = mapping
    iface = f"I{name}"
    lines = [
        f"namespace {namespace}",
        "{",
        "    using System;",
        "    using Newtonsoft.Json;",
        "    using Newtonsoft.Json.Linq;",
        "",
        "    /// <summary>",
        f"    /// {spec['description']}",
        "    /// </summary>",
        "    /// <remarks>",
        f"    /// Tagged union over <c>{prop}</c>. Deserialise as this interface to",
        "    /// have the converter select the variant; the variant classes are",
        "    /// generated by quicktype and adopt the interface via partial",
        "    /// declarations below.",
        "    /// </remarks>",
        f"    [JsonConverter(typeof({name}Converter))]",
        f"    public interface {iface} {{ }}",
        "",
    ]
    for variant in sorted(set(variants.values())):
        lines.append(f"    public partial class {variant} : {iface} {{ }}")
    lines += [
        "",
        f"    internal class {name}Converter : JsonConverter",
        "    {",
        f"        public override bool CanConvert(Type t) => t == typeof({iface});",
        "",
        "        // Read-only. Newtonsoft inherits a [JsonConverter] declared on an",
        "        // interface onto its implementing classes, so writing through this",
        "        // converter re-enters it: `Self referencing loop detected`. The",
        "        // variant already carries its own tag, so default serialisation is",
        "        // exactly right and CanWrite=false selects it.",
        "        public override bool CanWrite => false;",
        "",
        "        public override void WriteJson(",
        "            JsonWriter writer, object value, JsonSerializer serializer) =>",
        "            throw new NotSupportedException();",
        "",
        "        public override object ReadJson(",
        "            JsonReader reader, Type t, object existingValue, JsonSerializer serializer)",
        "        {",
        "            JObject obj = JObject.Load(reader);",
        f'            string tag = (string)obj["{prop}"];',
        "            switch (tag)",
        "            {",
    ]
    for value, variant in sorted(variants.items()):
        lines.append(f'                case "{value}": return Read<{variant}>(obj, serializer);')
    lines += [
        "                default:",
        "                    throw new JsonSerializationException(",
        f'                        $"unknown {prop} \'{{tag}}\' for {name}");',
        "            }",
        "        }",
        "",
        "        // Populate, not ToObject: the same inherited-attribute problem",
        "        // applies on read, and constructing the variant through the",
        "        // serializer would recurse into this converter until the stack",
        "        // overflowed. Filling an existing instance does not.",
        "        private static T Read<T>(JObject obj, JsonSerializer serializer)",
        "            where T : new()",
        "        {",
        "            T instance = new T();",
        "            using (var reader = obj.CreateReader())",
        "            {",
        "                serializer.Populate(reader, instance);",
        "            }",
        "            return instance;",
        "        }",
        "    }",
        "}",
    ]
    return "\n".join(lines) + "\n"


def _java_union(package: str, name: str, spec: dict, mapping: dict) -> str:
    """A Java holder with a Jackson deserializer that dispatches on the tag.

    Java has no partial classes, so the variants cannot be retro-fitted
    into a type hierarchy without editing generated files. A holder with a
    custom deserializer keeps the generated classes untouched and still
    gives callers a typed, exhaustive way in.
    """
    prop = spec["property"]
    variants = mapping
    ordered = sorted(set(variants.values()))
    lines = [
        f"package {package};",
        "",
        "import java.io.IOException;",
        "",
        "import com.fasterxml.jackson.core.JsonParser;",
        "import com.fasterxml.jackson.databind.DeserializationContext;",
        "import com.fasterxml.jackson.databind.JsonNode;",
        "import com.fasterxml.jackson.databind.ObjectMapper;",
        "import com.fasterxml.jackson.databind.JsonDeserializer;",
        "import com.fasterxml.jackson.databind.JsonSerializer;",
        "import com.fasterxml.jackson.databind.SerializerProvider;",
        "import com.fasterxml.jackson.core.JsonGenerator;",
        "import com.fasterxml.jackson.databind.annotation.JsonDeserialize;",
        "import com.fasterxml.jackson.databind.annotation.JsonSerialize;",
        "",
        "/**",
        f" * {spec['description']}",
        " *",
        f" * <p>Tagged union over {{@code {prop}}}. Exactly one accessor is",
        " * non-null; {@link #getKind()} says which.",
        " */",
        f"@JsonDeserialize(using = {name}.Deserializer.class)",
        f"@JsonSerialize(using = {name}.Serializer.class)",
        f"public class {name} {{",
        "",
        "    private final String kind;",
        "    private final Object value;",
        "",
        f"    public {name}(String kind, Object value) {{",
        "        this.kind = kind;",
        "        this.value = value;",
        "    }",
        "",
        "    /** The wire discriminator that selected the variant. */",
        "    public String getKind() { return kind; }",
        "",
        "    /** The variant, as its concrete generated type. */",
        "    public Object getValue() { return value; }",
        "",
    ]
    for variant in ordered:
        lines += [
            f"    public {variant} as{variant}() {{",
            f"        return value instanceof {variant} ? ({variant}) value : null;",
            "    }",
            "",
        ]
    lines += [
        f"    public static class Deserializer extends JsonDeserializer<{name}> {{",
        "        private static final ObjectMapper MAPPER = new ObjectMapper()",
        "                .findAndRegisterModules();",
        "",
        "        @Override",
        f"        public {name} deserialize(JsonParser p, DeserializationContext ctx)",
        "                throws IOException {",
        "            JsonNode node = p.readValueAsTree();",
        f'            JsonNode tagNode = node.get("{prop}");',
        "            String tag = tagNode == null ? null : tagNode.asText();",
        "            if (tag == null) {",
        f'                throw new IOException("{name}: missing discriminator \'{prop}\'");',
        "            }",
        "            switch (tag) {",
    ]
    for value, variant in sorted(variants.items()):
        lines.append(f'                case "{value}":')
        lines.append(f'                    return new {name}(tag, MAPPER.treeToValue(node, {variant}.class));')
    lines += [
        "                default:",
        f'                    throw new IOException("{name}: unknown {prop} '" + tag + "'");',
        "            }",
        "        }",
        "    }",
        "",
        f"    public static class Serializer extends JsonSerializer<{name}> {{",
        "        @Override",
        f"        public void serialize({name} v, JsonGenerator gen, SerializerProvider sp)",
        "                throws IOException {",
        "            // The variant carries its own tag, so writing the concrete",
        "            // value round-trips without special handling.",
        "            gen.writeObject(v.getValue());",
        "        }",
        "    }",
        "}",
    ]
    return "\n".join(lines) + "\n"


def step_java_sdk() -> str:
    """Generate Jackson-annotated Java POJOs from Schemas/.

    Per-domain composite, one quicktype invocation per domain. Java's
    one-public-class-per-file rule still holds — quicktype writes each
    class as a sibling of the `--out` path — so composite mode costs
    nothing in file granularity and fixes a real bug.

    The previous per-`$defs` invocation named every discriminator enum
    `Kind`, from the property name, and wrote it to `Kind.java` in the
    package. Ten domains had two or more discriminated types, so the last
    invocation won and the rest lost their values: a detector *command*
    could not be deserialised at all, because the surviving `Kind` held
    only the *measurement* values.

        java.lang.IllegalArgumentException: Cannot deserialize Kind

    The step's own comment called that overwrite "benign because the
    content matches"; for shared types like Location it was, and for these
    it was not. One composite invocation sees every type at once and names
    them apart — `ResetDetectorCommandKind`, `FlowDynamicKind` — so
    nothing is overwritten.
    """
    out_root = OUTPUT_ROOT / "java" / "sdk"
    if out_root.exists():
        shutil.rmtree(out_root)
    union_count = 0
    with _quicktype_schema_tree() as schemas_root:
        domains = sorted(
            d.name for d in schemas_root.iterdir()
            if d.is_dir() and d.name != "common"
        )
        for domain in domains:
            domain_dir = schemas_root / domain
            composite = domain_dir / "_composite.json"
            # Withheld from quicktype and emitted as real tagged unions
            # below; quicktype would flatten them.
            unions = _discriminated_unions(CANONICAL_SCHEMAS_DIR / domain)
            pkg_dir = out_root / "com" / "topas" / "utmc" / domain
            pkg_dir.mkdir(parents=True, exist_ok=True)
            try:
                count = _build_domain_composite(
                    domain_dir, composite, exclude=set(unions)
                )
                print(
                    f"[java_sdk] Schemas/{domain}/ ({count} types) -> "
                    f"{pkg_dir.relative_to(ROOT)}",
                    flush=True,
                )
                subprocess.run(
                    [
                        "npx", "-y", f"quicktype@{QUICKTYPE_VERSION}",
                        "--src-lang", "schema",
                        "--src", str(composite),
                        "--lang", "java",
                        "--top-level", "AllTypes",
                        "--package", f"com.topas.utmc.{domain}",
                        # quicktype writes every other class as a sibling
                        # of this path.
                        "--out", str(pkg_dir / "AllTypes.java"),
                    ],
                    check=True, cwd=ROOT,
                )
            finally:
                composite.unlink(missing_ok=True)

            for name, spec in sorted(unions.items()):
                mapping = _resolve_emitted_names(
                    "\n".join(f"public class {q.stem}" for q in pkg_dir.glob("*.java")),
                    r"public class (\w+)", spec["mapping"],
                    f"java/{domain}/{name}",
                )
                dest = pkg_dir / f"{name}.java"
                dest.write_text(
                    _java_union(f"com.topas.utmc.{domain}", name, spec, mapping)
                )
                print(f"[java_sdk] tagged union -> {dest.relative_to(ROOT)}", flush=True)
                union_count += 1
    print(f"[java_sdk] {union_count} tagged union(s)", flush=True)
    return f"quicktype@{QUICKTYPE_VERSION}"


def _emit_per_domain_composite(
    *,
    lang_label: str,
    quicktype_lang: str,
    dest_for_domain,    # callable: domain -> Path (target file)
    extra_for_domain,   # callable: domain -> list[str]
    post_emitter=None,  # callable: (domain, dest, unions) -> None
) -> str:
    """Run quicktype once per domain on a synthetic composite schema.

    Used for C# and TypeScript: their multi-class-per-file model means
    a single invocation can emit every domain type with shared types
    (Location, Quality) deduped. Java can't use this approach without
    losing per-class file granularity.
    """
    out_root = OUTPUT_ROOT / lang_label / "sdk"
    if out_root.exists():
        shutil.rmtree(out_root)
    with _quicktype_schema_tree() as schemas_root:
        domains = sorted(
            d.name for d in schemas_root.iterdir()
            if d.is_dir() and d.name != "common"
        )
        for domain in domains:
            domain_dir = schemas_root / domain
            composite = domain_dir / "_composite.json"
            # Unions are withheld from quicktype and emitted separately as
            # real tagged unions; quicktype would flatten them.
            unions = _discriminated_unions(CANONICAL_SCHEMAS_DIR / domain)
            try:
                count = _build_domain_composite(
                    domain_dir, composite, exclude=set(unions)
                )
                dest = dest_for_domain(domain)
                dest.parent.mkdir(parents=True, exist_ok=True)
                print(
                    f"[{lang_label}_sdk] Schemas/{domain}/ ({count} types) "
                    f"-> {dest.relative_to(ROOT)}",
                    flush=True,
                )
                cmd = [
                    "npx", "-y", f"quicktype@{QUICKTYPE_VERSION}",
                    "--src-lang", "schema",
                    "--src", str(composite),
                    "--lang", quicktype_lang,
                    "--top-level", "_AllTypes",
                    "--out", str(dest),
                ]
                cmd.extend(extra_for_domain(domain))
                subprocess.run(cmd, check=True, cwd=ROOT)
                if post_emitter is not None:
                    post_emitter(domain, dest, unions)
                    if unions:
                        print(f"[{lang_label}_sdk] + {len(unions)} tagged union(s)",
                              flush=True)
            finally:
                composite.unlink(missing_ok=True)
    return f"quicktype@{QUICKTYPE_VERSION}"


def step_csharp_sdk() -> str:
    """Generate C# POCOs from Schemas/.

    Per-domain composite: one .cs file per domain holding every type in
    a single namespace. Per-schema invocations would redeclare shared
    types (Location, Quality) which C# rejects.
    """
    out_root = OUTPUT_ROOT / "csharp" / "sdk"
    return _emit_per_domain_composite(
        lang_label="csharp",
        quicktype_lang="csharp",
        dest_for_domain=lambda d: out_root / f"{_pascal(d)}.cs",
        extra_for_domain=lambda d: [
            "--namespace", f"Topas.Utmc.{_pascal(d)}",
            # `attributes-only`, not `just-types-and-namespace`: the latter
            # strips [JsonProperty] and the per-enum converters, so the SDK
            # could not produce valid wire JSON — PascalCase property names
            # and enum members serialised as their C# names ("Calibrate"
            # rather than "calibrate"). `complete` adds only
            # FromJson/ToJson helpers on the synthetic AllTypes root, which
            # is not a type anyone should be deserialising.
            "--features", "attributes-only",
        ],
        post_emitter=_csharp_post_emit,
    )


def step_typescript_sdk() -> str:
    """Generate TypeScript interfaces from Schemas/.

    Per-domain composite: one .ts file per domain holding every interface.
    Keeps the file count and consumer ergonomics consistent with C#.
    """
    out_root = OUTPUT_ROOT / "typescript" / "sdk"
    return _emit_per_domain_composite(
        lang_label="typescript",
        quicktype_lang="ts",
        dest_for_domain=lambda d: out_root / f"{d}.ts",
        extra_for_domain=lambda _d: ["--just-types"],
        post_emitter=lambda d, dest, unions: _append_blocks(
            dest,
            [
                lambda text, name=name, spec=spec: _ts_union(
                    name, spec,
                    _resolve_emitted_names(
                        text, r"export interface (\w+)", spec["mapping"],
                        f"typescript/{d}/{name}",
                    ),
                )
                for name, spec in sorted(unions.items())
            ],
        ),
    )


# ── Rust SDK ─────────────────────────────────────────────────
# typify is pinned for the same reason as quicktype and
# datamodel-code-generator: its output is committed, so an unpinned
# generator turns the drift workflow red on a day nothing here changed.
TYPIFY_VERSION = "0.7.0"


class SchemaBundleConflict(RuntimeError):
    """Two different schemas would occupy one generated type name."""


def _bundle_external_refs(root: dict, source_file: Path) -> dict:
    """Make a composite self-contained by inlining every external `$defs`.

    typify rejects cross-file references outright — ``external references
    are not supported`` — so the `../common/common-v1.0.json#/$defs/Point`
    style used throughout Schemas/ has to be resolved first.

    Two subtleties, both learned the hard way:

    * a definition pulled in from another file brings its own `#/$defs/...`
      references with it, and those resolve against *that* file rather than
      ours, so they must be inlined too (a bundled `Point` referencing
      `#/$defs/Coordinates` otherwise leaves a dangling ref);
    * a name collision is refused rather than resolved. Silently keeping
      the first definition would let a field quietly change type.
    """
    defs = root.setdefault("$defs", {})
    inlined: set[tuple[str, str]] = set()
    root_key = source_file.resolve()
    pending: list[tuple[object, Path]] = [(root, root_key)]

    while pending:
        node, base = pending.pop()
        for target_file, name in _refs_to_inline(node, base, root_key):
            key = (str(target_file), name)
            if key in inlined:
                continue
            target = json.loads(target_file.read_text())
            if name not in target.get("$defs", {}):
                raise KeyError(f"{target_file}#/$defs/{name} not found")
            body = target["$defs"][name]
            if name in defs and _canonical(defs[name]) != _canonical(body):
                raise SchemaBundleConflict(
                    f"{name} differs between {source_file.name} and {target_file.name}"
                )
            defs[name] = body
            inlined.add(key)
            pending.append((body, target_file))

    _localise_refs(root)
    return root


def _canonical(node) -> str:
    return json.dumps(node, sort_keys=True)


def _refs_to_inline(node, base: Path, root_key: Path) -> list[tuple[Path, str]]:
    """References that must be copied into the root's own `$defs`.

    From the root file, only cross-file ones. From a file we have already
    pulled from, its plain `#/$defs/...` references too — that namespace is
    not ours.
    """
    out: list[tuple[Path, str]] = []

    def walk(n):
        if isinstance(n, dict):
            ref = n.get("$ref")
            if isinstance(ref, str):
                path, _, fragment = ref.partition("#")
                name = fragment.rsplit("/", 1)[-1]
                if path:
                    out.append(((base.parent / path).resolve(), name))
                elif base != root_key:
                    out.append((base, name))
            for value in n.values():
                walk(value)
        elif isinstance(n, list):
            for value in n:
                walk(value)

    walk(node)
    return out


def _localise_refs(node) -> None:
    """Point every cross-file `$ref` at the now-local `$defs` entry."""
    if isinstance(node, dict):
        ref = node.get("$ref")
        if isinstance(ref, str) and not ref.startswith("#"):
            node["$ref"] = f"#/$defs/{ref.rsplit('/', 1)[-1]}"
        for value in node.values():
            _localise_refs(value)
    elif isinstance(node, list):
        for value in node:
            _localise_refs(value)


def _const_to_single_enum(node) -> int:
    """Present `const: X` as `enum: [X]`. Returns the count rewritten.

    This is a correctness fix, not cosmetics. typify renders a discriminated
    union as `#[serde(untagged)]`, which relies on each variant's
    discriminator being a type that rejects the other variants' values. It
    renders `const: "reset"` as a plain `String`, which rejects nothing —
    and ResetDetectorCommand and CalibrateDetectorCommand have identical
    property sets, so `kind: "calibrate"` silently deserialised as
    *ResetDetectorCommand*. A command to calibrate a detector would have
    been executed as a command to reset it.

    The same value written as a single-member `enum` becomes a generated
    unit type (`ResetDetectorCommandKind`) that only accepts its own
    variant, and discrimination is correct again. Verified both ways
    against a live round trip.

    Applied only to the throwaway copy handed to typify; the canonical
    Schemas/ tree keeps `const`, which is the correct JSON Schema.
    """
    rewritten = 0
    if isinstance(node, dict):
        if "const" in node and "enum" not in node:
            node["enum"] = [node.pop("const")]
            rewritten += 1
        for value in node.values():
            rewritten += _const_to_single_enum(value)
    elif isinstance(node, list):
        for value in node:
            rewritten += _const_to_single_enum(value)
    return rewritten


def _assert_titles_consistent() -> None:
    """Refuse to share a type name across differing constraints.

    The Rust step keeps `title`, which is what lets typify emit one
    `ObjectId` instead of a newtype per field. That is only sound while
    every titled string type agrees on its constraints everywhere it
    appears; if two ever diverge, one name would cover two shapes.
    """
    seen: dict[str, set[str]] = {}

    def walk(node):
        if isinstance(node, dict):
            title = node.get("title")
            if title and node.get("type") == "string":
                sig = json.dumps(
                    {k: v for k, v in node.items()
                     if k in ("type", "minLength", "maxLength", "pattern", "format")},
                    sort_keys=True,
                )
                seen.setdefault(title, set()).add(sig)
            for value in node.values():
                walk(value)
        elif isinstance(node, list):
            for value in node:
                walk(value)

    for path in CANONICAL_SCHEMAS_DIR.rglob("*.json"):
        walk(json.loads(path.read_text()))

    conflicts = {k: v for k, v in seen.items() if len(v) > 1}
    if conflicts:
        raise SchemaBundleConflict(
            "titled string types disagree on constraints, so the Rust SDK "
            f"cannot share them by name: {sorted(conflicts)}"
        )


def step_rust_sdk() -> str:
    """Generate serde-derived Rust types from Schemas/.

    Per-domain, like C# and TypeScript: one module per domain holding every
    type. quicktype's Rust output was rejected; typify is
    Rust-native and produces `serde` derives, builders and `TryFrom`
    validation from the same schemas.
    """
    if shutil.which("cargo-typify") is None:
        raise RuntimeError(
            f"cargo-typify is required for the Rust SDK — "
            f"install with: cargo install cargo-typify --version {TYPIFY_VERSION} --locked"
        )
    _assert_titles_consistent()

    out_root = OUTPUT_ROOT / "rust" / "sdk"
    if out_root.exists():
        shutil.rmtree(out_root)
    src_dir = out_root / "src"
    src_dir.mkdir(parents=True, exist_ok=True)

    modules: list[str] = []
    with _quicktype_schema_tree() as schemas_root:
        domains = sorted(
            d.name for d in schemas_root.iterdir()
            if d.is_dir() and d.name != "common"
        )
        for domain in domains:
            domain_dir = schemas_root / domain
            composite = domain_dir / "_rust_composite.json"
            try:
                # Titles kept: typify shares a type by title, quicktype does not.
                count = _build_domain_composite(domain_dir, composite, strip_titles=False)
                bundled = _bundle_external_refs(
                    json.loads(composite.read_text()), composite
                )
                consts = _const_to_single_enum(bundled)
                bundled["properties"] = {
                    k: {"$ref": f"#/$defs/{k}"} for k in bundled["$defs"]
                }
                composite.write_text(json.dumps(bundled, indent=2))

                dest = src_dir / f"{domain}.rs"
                print(
                    f"[rust_sdk] Schemas/{domain}/ ({count} types, "
                    f"{consts} discriminators) -> {dest.relative_to(ROOT)}",
                    flush=True,
                )
                subprocess.run(
                    ["cargo", "typify", str(composite), "--output", str(dest)],
                    check=True, cwd=ROOT,
                )
                modules.append(domain)
            finally:
                composite.unlink(missing_ok=True)

    lib = ["//! Generated Rust SDK — do not hand-edit.",
           "//!",
           "//! One module per domain, produced by cargo-typify from Schemas/.",
           "//! Regenerate with `make asyncapi-clients`.",
           ""]
    lib += [f"pub mod {m};" for m in modules]
    (src_dir / "lib.rs").write_text("\n".join(lib) + "\n")
    return f"typify@{TYPIFY_VERSION}"


# build stubs: hand-written project files copied alongside each SDK
# so CI legs can compile / typecheck the generated output. Stubs live in
# `templates/sdk_stubs/<lang>/`; `step_sdk_stubs` copies them into
# `output/<lang>/sdk/` after generation.
_SDK_STUB_FILES = {
    "java": ["pom.xml", ".gitignore", "tests/TaggedUnionTest.java"],
    "csharp": ["UtmcSdk.csproj", ".gitignore", "tests/TaggedUnionTests.cs"],
    "typescript": ["tsconfig.json", "tests/unions.ts"],
    "rust": ["Cargo.toml", ".gitignore", "tests/discriminator.rs"],
}


def step_sdk_stubs(languages: list[str]) -> None:
    """Copy build stubs (pom.xml, .csproj, tsconfig.json) into each SDK tree."""
    stubs_root = TEMPLATES_ROOT / "sdk_stubs"
    for lang in languages:
        if lang not in _SDK_STUB_FILES:
            continue
        sdk_root = OUTPUT_ROOT / lang / "sdk"
        if not sdk_root.exists():
            continue
        for filename in _SDK_STUB_FILES[lang]:
            src = stubs_root / lang / filename
            if not src.is_file():
                continue
            dest = sdk_root / filename
            dest.parent.mkdir(parents=True, exist_ok=True)
            dest.write_text(src.read_text())
            print(f"[sdk_stubs] {src.relative_to(ROOT)} -> {dest.relative_to(ROOT)}", flush=True)


def step_python_sdk() -> str:
    """Generate Pydantic v2 SDK from the canonical Schemas/ directory.

    Returns the short label that goes in the file header
    Generator field (e.g. 'datamodel-code-generator@0.56.1').
    """
    if shutil.which("uv") is None:  # pragma: no cover - sanity belt
        raise RuntimeError("uv is required to invoke datamodel-codegen via 'uv run'")
    out_dir = OUTPUT_ROOT / "python" / "sdk"
    out_dir.mkdir(parents=True, exist_ok=True)
    cmd = [
        "uv", "run", "datamodel-codegen",
        "--input", str(CANONICAL_SCHEMAS_DIR),
        "--input-file-type", "jsonschema",
        "--output-model-type", "pydantic_v2.BaseModel",
        "--output", str(out_dir),
        "--disable-timestamp",
    ]
    print(f"[python_sdk] {' '.join(cmd)}", flush=True)
    subprocess.run(cmd, check=True, cwd=ROOT)
    return f"datamodel-code-generator@{_datamodel_codegen_version()}"


def _snippet_stem(lang: str, snippet_id: str) -> str:
    """Filename stem for a snippet — uppercase for Java, lowercase elsewhere.

    Java requires the source filename to match the public class name
    (`s1.java` will not compile a `public class S1` on case-sensitive
    filesystems). The class names in our templates are already
    uppercase, so the filename has to follow.
    """
    return snippet_id.upper() if lang == "java" else snippet_id


def _has_templates(lang: str) -> bool:
    """True iff at least an S1 template exists for this language.

    Skipping languages whose templates haven't been authored yet keeps
    the generator runnable as Phase B lands one language at a time.
    """
    stem = _snippet_stem(lang, "s1")
    return (TEMPLATES_ROOT / lang / f"{stem}.{LANG_EXT[lang]}.j2").is_file()


def step_snippets(languages: list[str]) -> None:
    ctx = _build_template_context()
    for lang in languages:
        if not _has_templates(lang):
            print(f"[snippets] {lang}: no templates yet — skipped", flush=True)
            continue
        ext = LANG_EXT[lang]
        out_dir = OUTPUT_ROOT / lang / "snippets"
        out_dir.mkdir(parents=True, exist_ok=True)
        for scen in SCENARIOS:
            stem = _snippet_stem(lang, scen.id)
            rendered = snippets.render(
                templates_root=TEMPLATES_ROOT, language=lang,
                snippet_id=stem, ext=ext, context=ctx,
            )
            (out_dir / f"{stem}.{ext}").write_text(rendered)


def step_readme(languages: list[str]) -> None:
    ctx = _build_template_context()
    for lang in languages:
        if not (TEMPLATES_ROOT / lang / "readme.md.j2").is_file():
            continue
        rendered = snippets.render(
            templates_root=TEMPLATES_ROOT, language=lang,
            snippet_id="readme", ext="md", context=ctx,
        )
        (OUTPUT_ROOT / lang / "README.md").write_text(rendered)


def step_headers(languages: list[str], sdk_generators: dict[str, str]) -> None:
    """Prepend the header to every emitted source file under output/<lang>/.

    `sdk_generators` maps language -> Generator label (e.g. "quicktype@23.0.171"
    or "datamodel-code-generator@0.56.1"). Files under output/<lang>/sdk/ are
    credited to that label; snippet files are always credited to the
    asyncapi_clients generator itself.

    Copied build stubs are skipped. They are hand-written and live under
    templates/sdk_stubs/, so stamping them "Generated ... do not hand-edit"
    would point an editor at the wrong file.
    """
    today = _gen_date()
    stub_paths = {
        (OUTPUT_ROOT / lang / "sdk" / name).resolve()
        for lang, names in _SDK_STUB_FILES.items()
        for name in names
    }
    source_suffixes = {".py", ".java", ".cs", ".cpp", ".h", ".hpp", ".ts", ".go", ".rs"}
    for lang in languages:
        lang_root = OUTPUT_ROOT / lang
        if not lang_root.exists():
            continue
        sdk_root = lang_root / "sdk"
        for path in lang_root.rglob("*"):
            if not path.is_file() or path.suffix not in source_suffixes:
                continue
            if path.resolve() in stub_paths:
                continue
            # Skip files that already carry a header from a prior run.
            existing = path.read_text()
            if existing.lstrip().startswith(('"""', "/*")) and "Author:      Christopher Kettell" in existing[:1000]:
                continue
            is_sdk_file = sdk_root in path.parents
            if is_sdk_file:
                label = sdk_generators.get(lang, "asyncapi_clients/generator")
            else:
                label = "asyncapi_clients/generator"
            rel = path.relative_to(ASYNCAPI_CLIENTS_DIR)
            hdr = headers.build_header(
                relative_path=str(rel), language=lang, gen_date=today,
                sdk_generator=label, generator_version=GENERATOR_VERSION,
            )
            path.write_text(hdr + existing)


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--only-lang", choices=LANGUAGES, default=None)
    parser.add_argument(
        "--only-step",
        choices=[
            "python_sdk",
            "java_sdk",
            "csharp_sdk",
            "typescript_sdk",
            "snippets",
            "headers",
            "readme",
        ],
        default=None,
    )
    args = parser.parse_args(argv)
    languages = [args.only_lang] if args.only_lang else LANGUAGES
    requested_step = args.only_step

    sdk_generators: dict[str, str] = {
        "python": f"datamodel-code-generator@{_datamodel_codegen_version()}",
        "java": f"quicktype@{QUICKTYPE_VERSION}",
        "csharp": f"quicktype@{QUICKTYPE_VERSION}",
        "typescript": f"quicktype@{QUICKTYPE_VERSION}",
    }

    if requested_step in (None, "python_sdk"):
        if "python" in languages:
            sdk_generators["python"] = step_python_sdk()
    if requested_step in (None, "java_sdk"):
        if "java" in languages:
            sdk_generators["java"] = step_java_sdk()
    if requested_step in (None, "csharp_sdk"):
        if "csharp" in languages:
            sdk_generators["csharp"] = step_csharp_sdk()
    if requested_step in (None, "typescript_sdk"):
        if "typescript" in languages:
            sdk_generators["typescript"] = step_typescript_sdk()

        if "rust" in languages:
            sdk_generators["rust"] = step_rust_sdk()

    # Copy build stubs (pom.xml, csproj, tsconfig) into each SDK
    # tree so CI legs can compile/typecheck the output. Runs after every
    # SDK generation step (or on a no-op pass if --only-step is used).
    if requested_step in (None, "java_sdk", "csharp_sdk", "typescript_sdk"):
        step_sdk_stubs(languages)

    if requested_step in (None, "snippets"):
        step_snippets(languages)
    if requested_step in (None, "headers"):
        step_headers(languages, sdk_generators)
    if requested_step in (None, "readme"):
        step_readme(languages)
    return 0


if __name__ == "__main__":
    sys.exit(main())
