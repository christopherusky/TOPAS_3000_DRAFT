#!/usr/bin/env python3
# ──────────────────────────────────────────────────────────────────────
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-05
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Generate per-schema and per-domain diagrams from the emitted JSON Schemas.

Reads the JSON Schema files emitted by bin/emit.py and produces:
  Diagrams/mermaid/  — .mmd files (render natively on GitHub)
  Diagrams/uml/      — .puml files (PlantUML class diagrams, A3/A4)
  Diagrams/png/      — .png files (rendered from Mermaid via mermaid-cli)

One diagram per JSON schema file, plus one aggregate per domain.

Usage:
  python bin/generate_diagrams.py                    # all formats
  python bin/generate_diagrams.py --format mermaid   # Mermaid only
  python bin/generate_diagrams.py --format uml       # PlantUML only
  python bin/generate_diagrams.py --format png       # PNG only (renders Mermaid)
  python bin/generate_diagrams.py --domain detector  # single domain
  python bin/generate_diagrams.py --list-domains     # list available domains
  python bin/generate_diagrams.py --domain-only      # domain_ diagrams only

Prerequisites for PNG:
  Node.js >= 18, Chromium
  npx @mermaid-js/mermaid-cli@10
"""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import tempfile
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from bin.scope import packages_on_disk  # noqa: E402

SCHEMA_DIR = ROOT / "Schemas"
DIAGRAM_DIR = ROOT / "Diagrams"

# ── Data model ───────────────────────────────────────────────────────

FORMAT_DISPLAY = {
    "date-time": "datetime",
    "date": "date",
    "time": "time",
    "uri": "uri",
}


@dataclass
class FieldInfo:
    name: str
    type_display: str
    required: bool
    is_array: bool
    local_ref: str | None = None
    external_type: str | None = None
    external_source: str | None = None


@dataclass
class TypeDef:
    name: str
    description: str
    fields: list[FieldInfo]


@dataclass
class EnumDef:
    name: str
    description: str
    values: list[str]


@dataclass
class UnionDef:
    name: str
    description: str
    discriminator: str
    variants: list[str]


@dataclass
class SchemaModel:
    filename: str
    stem: str
    title: str
    root_type: str | None
    types: list[TypeDef]
    enums: list[EnumDef]
    unions: list[UnionDef]
    external_stubs: dict[str, str]  # type_name -> source filename


# ── Schema parsing ───────────────────────────────────────────────────

# Titles that are surfaced as semantic type names in diagrams. Set by
# `Field(title=...)` on the shared aliases in `topas_3000_model/common/ids.py`,
# preserved through `bin/emit.py::_clean_def` (which strips all other
# titles). Keep in sync with `bin/emit.py::_PRESERVED_TITLES`.
_TYPE_ALIAS_TITLES = frozenset({"ObjectId", "ParticipantId", "QualityId", "TypeId"})


def _format_type(spec: dict) -> str:
    if "const" in spec:
        return f'"{spec["const"]}"'
    if "enum" in spec:
        return "enum"
    title = spec.get("title")
    if title in _TYPE_ALIAS_TITLES:
        return title
    t = spec.get("type", "any")
    fmt = spec.get("format")
    if fmt in FORMAT_DISPLAY:
        return FORMAT_DISPLAY[fmt]
    return t


def _parse_field(name: str, spec: dict, required_set: set[str]) -> FieldInfo:
    is_required = name in required_set
    is_array = spec.get("type") == "array"

    ref = spec.get("$ref")
    if ref:
        return _ref_field(name, ref, is_required, is_array=False)

    if is_array:
        items = spec.get("items", {})
        ref = items.get("$ref")
        if ref:
            return _ref_field(name, ref, is_required, is_array=True)
        return FieldInfo(name, _format_type(items), is_required, is_array=True)

    return FieldInfo(name, _format_type(spec), is_required, is_array=False)


def _ref_field(name: str, ref: str, is_required: bool, is_array: bool) -> FieldInfo:
    if ref.startswith("#/$defs/"):
        target = ref[8:]
        return FieldInfo(name, target, is_required, is_array, local_ref=target)
    if "#/$defs/" in ref:
        source, target = ref.split("#/$defs/")
        return FieldInfo(name, target, is_required, is_array,
                         external_type=target, external_source=source)
    return FieldInfo(name, ref, is_required, is_array)


def parse_schema(filepath: Path) -> SchemaModel:
    with open(filepath) as f:
        data = json.load(f)

    filename = filepath.name
    stem = filename.removesuffix("-v1.0.json")
    title = data.get("title", "")
    defs = data.get("$defs", {})

    root_ref = data.get("$ref")
    root_type = root_ref[8:] if root_ref and root_ref.startswith("#/$defs/") else None

    # If root type's $def has a title, use it
    if root_type and root_type in defs:
        title = defs[root_type].get("title", title)
    if not title:
        title = stem.replace("_", " ").title()

    types: list[TypeDef] = []
    enums: list[EnumDef] = []
    unions: list[UnionDef] = []
    external_stubs: dict[str, str] = {}

    for def_name, defn in defs.items():
        desc = defn.get("description", "")

        # Enum (no properties, no oneOf)
        if "enum" in defn and "properties" not in defn and "oneOf" not in defn:
            enums.append(EnumDef(def_name, desc, defn["enum"]))
            continue

        # Discriminated union
        if "oneOf" in defn and "discriminator" in defn:
            disc = defn["discriminator"]
            variant_names = []
            for item in defn["oneOf"]:
                r = item.get("$ref", "")
                if r.startswith("#/$defs/"):
                    variant_names.append(r[8:])
            unions.append(UnionDef(
                def_name, desc, disc.get("propertyName", "kind"), variant_names,
            ))
            continue

        # Object type
        if defn.get("type") == "object" or "properties" in defn:
            required_set = set(defn.get("required", []))
            fields: list[FieldInfo] = []
            for prop_name, prop_spec in defn.get("properties", {}).items():
                fi = _parse_field(prop_name, prop_spec, required_set)
                fields.append(fi)
                if fi.external_type and fi.external_source:
                    external_stubs[fi.external_type] = fi.external_source
            types.append(TypeDef(def_name, desc, fields))

    return SchemaModel(
        filename=filename, stem=stem, title=title, root_type=root_type,
        types=types, enums=enums, unions=unions, external_stubs=external_stubs,
    )


# ── Domain grouping ──────────────────────────────────────────────────

# Read from the model packages, longest first, so `air_quality_enhanced_*` is not
# taken for `air_quality` and a new domain needs no edit here.
DOMAIN_PREFIXES = tuple(sorted(packages_on_disk(), key=len, reverse=True))


def domain_of(stem: str) -> str:
    for prefix in DOMAIN_PREFIXES:
        if stem.startswith(prefix):
            return prefix
    if stem == "common":
        return "common"
    return stem.split("_")[0]


def group_by_domain(models: list[SchemaModel]) -> dict[str, list[SchemaModel]]:
    domains: dict[str, list[SchemaModel]] = {}
    for m in models:
        domains.setdefault(domain_of(m.stem), []).append(m)
    return domains


# ── Helpers ──────────────────────────────────────────────────────────

def _all_type_names(model: SchemaModel) -> set[str]:
    names = {t.name for t in model.types}
    names.update(e.name for e in model.enums)
    names.update(u.name for u in model.unions)
    return names


def _pretty_domain(domain: str) -> str:
    return f"UTMC {domain.replace('_', ' ').title()} Domain"


# ── Mermaid ER-diagram generation ────────────────────────────────────

def _to_screaming_snake(name: str) -> str:
    """PascalCase to SCREAMING_SNAKE_CASE."""
    s = re.sub(r"(?<=[a-z0-9])([A-Z])", r"_\1", name)
    s = re.sub(r"([A-Z]+)([A-Z][a-z])", r"\1_\2", s)
    return s.upper()


def _er_type(f: FieldInfo) -> str:
    """Map a field to its ER-diagram type label."""
    if f.local_ref or f.external_type:
        return "object[]" if f.is_array else "object"
    t = f.type_display
    if t.startswith('"') and t.endswith('"'):
        return "string"
    if t == "integer":
        t = "int"
    elif t == "boolean":
        t = "bool"
    if f.is_array:
        t = f"{t}[]"
    return t


def _domain_id_field(domain: str) -> str:
    """e.g. 'traffic_signal' -> 'trafficSignalId'."""
    parts = domain.split("_")
    return parts[0] + "".join(p.capitalize() for p in parts[1:]) + "Id"


def _schema_category(stem: str, domain: str) -> str:
    """e.g. 'detector_fault' with domain 'detector' -> 'fault'."""
    if stem == domain:
        return "definition"
    return stem.removeprefix(f"{domain}_")


def _pk_fk(field_name: str, category: str, domain: str) -> str:
    """Return 'PK', 'FK', or '' for a field based on naming + schema category."""
    domain_id = _domain_id_field(domain)
    if field_name == domain_id:
        return "PK" if category == "definition" else "FK"
    if field_name == "typeId":
        return "PK" if category == "type" else "FK"
    if field_name == "faultId":
        return "PK" if "fault" in category else ""
    if field_name == "commandId":
        return "PK" if category in ("command", "command_response") else "FK"
    if field_name == "faultTypeId":
        return "FK"
    if field_name == "qualityId":
        return "PK"
    return ""


_LIBRARY_PKS = {"qualityId", "faultTypeId"}


def _pk_fk_library(field_name: str) -> str:
    """PK/FK for library schemas (common, fault_support)."""
    if field_name in _LIBRARY_PKS:
        return "PK"
    return ""


def _er_entity(
    entity_name: str,
    fields: list[FieldInfo],
    stem: str,
    domain: str,
    skip_extensions: bool = True,
) -> list[str]:
    """Emit one erDiagram entity block."""
    is_library = domain in ("common", "fault_support")
    lines = [f"    {entity_name} {{"]
    for f in fields:
        if skip_extensions and f.name == "extensions":
            continue
        er_t = _er_type(f)
        ann = _pk_fk_library(f.name) if is_library else _pk_fk(f.name, _schema_category(stem, domain), domain)
        parts = [f"        {er_t}", f.name]
        if ann:
            parts.append(ann)
        lines.append(" ".join(parts))
    lines.append("    }")
    return lines


def _er_enum_entity(entity_name: str, ed: EnumDef) -> list[str]:
    """Emit an enum as an ER entity listing its values."""
    lines = [f"    {entity_name} {{"]
    for v in ed.values:
        lines.append(f"        enum {v}")
    lines.append("    }")
    return lines


# ── Per-schema ER ────────────────────────────────────────────────────

def emit_mermaid_schema(model: SchemaModel) -> str:
    """Mermaid erDiagram for a single schema file."""
    lines = [
        "---",
        f"title: {model.title}",
        "---",
        "erDiagram",
    ]

    # Object types
    for td in model.types:
        entity = _to_screaming_snake(td.name)
        lines.extend(_er_entity(entity, td.fields, model.stem, domain_of(model.stem)))
        lines.append("")

    # Enums
    for ed in model.enums:
        entity = _to_screaming_snake(ed.name)
        lines.extend(_er_enum_entity(entity, ed))
        lines.append("")

    # Unions: show discriminator entity + variant relationships
    for ud in model.unions:
        u_entity = _to_screaming_snake(ud.name)
        lines.append(f"    {u_entity} {{")
        lines.append(f"        string {ud.discriminator}")
        lines.append("    }")
        lines.append("")
        for v in ud.variants:
            v_entity = _to_screaming_snake(v)
            disc_val = ""
            # find the const value for the discriminator in this variant
            for td in model.types:
                if td.name == v:
                    for f in td.fields:
                        if f.name == ud.discriminator and f.type_display.startswith('"'):
                            disc_val = f.type_display.strip('"')
            label = disc_val or v
            lines.append(f'    {u_entity} ||--|| {v_entity} : "{label}"')
        lines.append("")

    # Local $ref relationships (e.g. Point -> Coordinates)
    type_names = {t.name for t in model.types}
    for td in model.types:
        src = _to_screaming_snake(td.name)
        for f in td.fields:
            if f.local_ref and f.local_ref in type_names and f.local_ref != td.name:
                tgt = _to_screaming_snake(f.local_ref)
                lines.append(f"    {src} ||--|| {tgt} : {f.name}")

    lines.append("")
    return "\n".join(lines) + "\n"


# ── Per-domain ER ────────────────────────────────────────────────────

def _domain_entity_name(stem: str, domain: str) -> str:
    """Entity name for the domain diagram (SCREAMING_CASE from stem)."""
    cat = _schema_category(stem, domain)
    if cat == "definition":
        return domain.upper()
    return stem.upper()


def _union_common_fields(model: SchemaModel) -> list[FieldInfo]:
    """For a union schema, return fields common to all variants."""
    if not model.unions:
        return []
    union = model.unions[0]
    variant_map = {t.name: t for t in model.types}
    field_dicts: list[dict[str, FieldInfo]] = []
    for v in union.variants:
        if v in variant_map:
            field_dicts.append({f.name: f for f in variant_map[v].fields})
    if not field_dicts:
        return []
    common_names = set(field_dicts[0])
    for fd in field_dicts[1:]:
        common_names &= set(fd)
    # preserve order from first variant
    return [field_dicts[0][n] for n in field_dicts[0] if n in common_names]


CATEGORY_ORDER = [
    "type", "definition", "configuration", "measurement",
    "command", "command_response", "fault", "history",
]

DOMAIN_RELS: dict[str, tuple[str, str]] = {
    "type":             ("classifies", "||--o{"),
    "configuration":    ("has",        "||--o|"),
    "measurement":      ("emits",      "||--o{"),
    "status":           ("publishes",  "||--o{"),
    "command":          ("receives",   "||--o{"),
    "fault":            ("raises",     "||--o{"),
    "history":          ("records",    "||--o{"),
}


def _extract_fields_for_model(m: SchemaModel) -> list[FieldInfo]:
    """Get the display fields for a schema model."""
    if m.unions:
        return _union_common_fields(m)
    if m.types:
        root = next((t for t in m.types if t.name == m.root_type), m.types[0])
        return root.fields
    return []


def _find_common_field_names(
    models: list[SchemaModel], domain: str,
) -> set[str]:
    """Find field names that appear on ALL non-library models in a domain.

    Used by non-device domains (e.g. traffic_event) to suppress shared
    base fields and only show subtype-specific fields per entity.
    """
    all_field_sets: list[set[str]] = []
    for m in models:
        cat = _schema_category(m.stem, domain)
        if cat == "common" or (m.enums and not m.types):
            continue
        fields = _extract_fields_for_model(m)
        if fields:
            all_field_sets.append({f.name for f in fields})
    if len(all_field_sets) < 2:
        return set()
    return set.intersection(*all_field_sets)


def emit_mermaid_domain(domain: str, models: list[SchemaModel]) -> str:
    """Mermaid erDiagram for an entire domain — one entity per schema."""
    title = _pretty_domain(domain)
    lines = [
        "---",
        f"title: {title}",
        "---",
        "erDiagram",
    ]

    entity_by_cat: dict[str, str] = {}
    cat_order: list[str] = []
    # Edges added by expanded union variants and nested types — appended
    # after the standard domain-pattern relationships so the section reads
    # base-level edges first, intra-schema detail last.
    extra_edges: list[str] = []

    # Sort models by CATEGORY_ORDER for visual consistency
    def _sort_key(m: SchemaModel) -> int:
        cat = _schema_category(m.stem, domain)
        return CATEGORY_ORDER.index(cat) if cat in CATEGORY_ORDER else 99

    # Check if this is a device-pattern domain (has a definition category)
    has_definition = any(
        _schema_category(m.stem, domain) == "definition" for m in models
    )

    # For non-device domains, find fields shared across all subtypes
    # so we can show them once in a shared entity instead of repeating
    suppress_fields: set[str] = set()
    if not has_definition:
        suppress_fields = _find_common_field_names(models, domain)

    # Track which enums came from which category (for relationship logic)
    enum_source_cat: dict[str, str] = {}  # enum_name -> category

    # Pre-build ref mapping: shared enum type name -> (ref number, field name)
    # by scanning subtype models for external_type references to common enums
    shared_enum_ref: dict[str, int] = {}  # enum type name -> ref number
    enum_field_map: dict[str, str] = {}   # enum type name -> field name
    if not has_definition:
        ref_counter = 0
        for m in sorted(models, key=_sort_key):
            cat = _schema_category(m.stem, domain)
            if cat != "common":
                for td in m.types:
                    for fi in td.fields:
                        if fi.external_type and fi.external_type not in shared_enum_ref:
                            # Check if this external type comes from the common schema
                            common_stem = f"{domain}_common"
                            if fi.external_source and common_stem in fi.external_source:
                                ref_counter += 1
                                shared_enum_ref[fi.external_type] = ref_counter
                                enum_field_map[fi.external_type] = fi.name
                break  # only need one subtype to discover the mapping

    for m in sorted(models, key=_sort_key):
        cat = _schema_category(m.stem, domain)
        entity = _domain_entity_name(m.stem, domain)
        entity_by_cat[cat] = entity
        cat_order.append(cat)

        # Show enum entities (may coexist with type entities in same schema)
        if m.enums:
            for ed in m.enums:
                en = _to_screaming_snake(ed.name)
                # Shared enums get a ref prefix in their entity name
                if ed.name in shared_enum_ref:
                    ref_num = shared_enum_ref[ed.name]
                    en = f"ref{ref_num}__{en}"
                entity_by_cat[ed.name] = en
                enum_source_cat[ed.name] = cat
                lines.extend(_er_enum_entity(en, ed))
                lines.append("")

        # Show type entity (if schema has types)
        # For library schemas (no root type, multiple types), show each type
        # as its own entity so all $defs are visible
        if m.root_type is None and len(m.types) > 1:
            for td in m.types:
                type_ent = _to_screaming_snake(td.name)
                entity_by_cat[td.name] = type_ent
                lines.extend(_er_entity(type_ent, td.fields, m.stem, domain))
                lines.append("")
            # Remove the category-level entity since we expanded to individual types
            if cat in entity_by_cat:
                del entity_by_cat[cat]
            continue

        fields = _extract_fields_for_model(m)
        if not fields:
            continue

        if suppress_fields and cat != "common":
            # Show only subtype-specific fields (filter out shared base fields)
            fields = [f for f in fields if f.name not in suppress_fields]

        lines.extend(_er_entity(entity, fields, m.stem, domain))
        lines.append("")

        # Union schemas: emit each variant as its own entity so the
        # discriminator-specific payload (e.g. SetPlanCommand.planNumber,
        # ForceStageCommand.stageNumber) is visible at the domain level.
        if m.unions:
            union = m.unions[0]
            variant_by_name = {t.name: t for t in m.types}
            for v_name in union.variants:
                td = variant_by_name.get(v_name)
                if not td:
                    continue
                v_entity = _to_screaming_snake(td.name)
                # Skip when a variant's natural name matches the base entity
                # (e.g. transport_link_type union has a TransportLinkType
                # variant whose screaming-snake collides with the TYPE-stem
                # entity). The base entity already represents this variant;
                # per-schema diagrams show the full detail.
                if v_entity == entity:
                    continue
                lines.extend(_er_entity(v_entity, td.fields, m.stem, domain))
                lines.append("")
                disc_val = ""
                for fi in td.fields:
                    if fi.name == union.discriminator and fi.type_display.startswith('"'):
                        disc_val = fi.type_display.strip('"')
                label = disc_val or v_name
                extra_edges.append(f'    {entity} ||--|| {v_entity} : "{label}"')

        # Non-union schemas with nested object types reachable from the
        # root via local $ref (e.g. TrafficSignalDynamicState -> PlanLine
        # -> SignalGroupTimings). Emit each reachable non-root type as its
        # own box and connect via the referencing field name. Unreferenced
        # sibling $defs are skipped to avoid orphan entities.
        elif m.root_type and len(m.types) > 1:
            type_by_name = {t.name: t for t in m.types}
            reachable: set[str] = set()
            queue = [m.root_type]
            while queue:
                name = queue.pop(0)
                if name in reachable or name not in type_by_name:
                    continue
                reachable.add(name)
                for fi in type_by_name[name].fields:
                    if fi.local_ref and fi.local_ref in type_by_name:
                        queue.append(fi.local_ref)
            for td in m.types:
                if td.name == m.root_type or td.name not in reachable:
                    continue
                n_entity = _to_screaming_snake(td.name)
                lines.extend(_er_entity(n_entity, td.fields, m.stem, domain))
                lines.append("")
            # Iterate m.types, not `reachable` — the latter is a set, and
            # string hashing is randomised per process, so iterating it emits
            # these edges in a different order on every run. Declaration order
            # also matches the entity boxes emitted just above.
            for src_td in m.types:
                src_name = src_td.name
                if src_name not in reachable:
                    continue
                src_ent = entity if src_name == m.root_type else _to_screaming_snake(src_name)
                for fi in src_td.fields:
                    if fi.local_ref and fi.local_ref in reachable and fi.local_ref != src_name:
                        tgt_ent = _to_screaming_snake(fi.local_ref)
                        cardinality = "||--o{" if fi.is_array else "||--||"
                        extra_edges.append(
                            f"    {src_ent} {cardinality} {tgt_ent} : {fi.name}"
                        )

    # For non-device domains with shared fields, emit a SHARED_FIELDS entity
    # showing the common base fields that were suppressed from each subtype.
    # Fields that reference shared enums get a __refN suffix to match the
    # refN__ prefix on the enum entity — no relationship lines needed.
    if suppress_fields and not has_definition:
        representative = None
        for m in models:
            cat = _schema_category(m.stem, domain)
            if cat != "common" and not (m.enums and not m.types):
                representative = m
                break
        if representative:
            all_fields = _extract_fields_for_model(representative)
            shared = [f for f in all_fields if f.name in suppress_fields]
            shared_entity = f"{domain.upper()}_SHARED_FIELDS"
            entity_by_cat["_shared"] = shared_entity
            # Build the entity lines manually to inject ref annotations
            shared_lines = [f"    {shared_entity} {{"]
            for f in shared:
                if f.name == "extensions":
                    continue
                er_t = _er_type(f)
                # Check if this field references a shared enum
                ref_suffix = ""
                if f.external_type and f.external_type in shared_enum_ref:
                    ref_suffix = f"__ref{shared_enum_ref[f.external_type]}"
                shared_lines.append(f"        {er_t} {f.name}{ref_suffix}")
            shared_lines.append("    }")
            lines.extend(shared_lines)
            lines.append("")

    # Relationships — include the linking field name for traceability
    # Library domains (common, fault_support) have no device-pattern relationships
    is_library_domain = domain in ("common", "fault_support")
    central = entity_by_cat.get("definition") if not is_library_domain else None
    # Discover the actual PK field from the definition schema rather than
    # constructing from domain name (handles air_quality_enhanced using
    # airQualityId, not airQualityEnhancedId)
    domain_id = _domain_id_field(domain)  # fallback
    for m in models:
        if _schema_category(m.stem, domain) == "definition" and m.types:
            root = next((t for t in m.types if t.name == m.root_type), m.types[0])
            for fi in root.fields:
                if fi.name.endswith("Id") and fi.required and fi.name != "typeId":
                    domain_id = fi.name
                    break
            break
    if central:
        # Type -> Definition
        type_entity = entity_by_cat.get("type")
        if type_entity:
            # Find the FK field on Definition that refs the type catalogue.
            # The TypeId alias collapses to a bare integer at the JSON-Schema
            # layer, so we recognise the FK by name. Convention: most
            # domains use `typeId`; traffic_signal uses `controlMethodId`
            # to disambiguate from its closed `trafficSignalType` enum.
            type_fk = "typeId"
            for m in models:
                if _schema_category(m.stem, domain) == "definition" and m.types:
                    root = next((t for t in m.types if t.name == m.root_type), m.types[0])
                    candidates = [
                        fi.name for fi in root.fields
                        if fi.name == "typeId" or fi.name.endswith("MethodId")
                    ]
                    if candidates:
                        type_fk = candidates[0]
                    break
            lines.append(f"    {type_entity} ||--o{{ {central} : \"classifies ({type_fk})\"")

        # Definition -> others
        for cat, (label, cardinality) in DOMAIN_RELS.items():
            if cat == "type":
                continue  # handled above
            other = entity_by_cat.get(cat)
            if other:
                lines.append(f"    {central} {cardinality} {other} : \"{label} ({domain_id})\"")

        # Command -> Command Response (multi-response and the
        # Response Streams rule in README.md: zero or more
        # status: pending frames followed by exactly one terminal frame, all
        # sharing the same commandCorrelationId — so cardinality is one-to-many).
        cmd = entity_by_cat.get("command")
        cmd_resp = entity_by_cat.get("command_response")
        if cmd and cmd_resp:
            lines.append(
                f"    {cmd} ||--o{{ {cmd_resp} : "
                f"\"produces (commandCorrelationId, status-progression)\""
            )

        # Fallback: any extra category (e.g. detectors_configuration,
        # tariff_configuration, opening_times_configuration, access_location,
        # zone_list_entry) with a domain_id FK field gets a generic "has" edge
        # back to central. Without this, schemas not in DOMAIN_RELS would appear
        # as disconnected entities in the diagram.
        standard_cats = set(DOMAIN_RELS.keys()) | {
            "definition", "command_response", "type",
        }
        for m in models:
            cat = _schema_category(m.stem, domain)
            if cat in standard_cats:
                continue
            other = entity_by_cat.get(cat)
            if not other or other == central:
                continue
            fields = _extract_fields_for_model(m)
            if any(f.name == domain_id for f in fields):
                label_words = cat.replace("_", " ")
                lines.append(
                    f"    {central} ||--o{{ {other} : \"has {label_words} ({domain_id})\""
                )

        # Inline enum relationships — connect domain-specific enums
        # (e.g. AccessControlState, VmsStatus) to their parent entity
        for m in models:
            cat = _schema_category(m.stem, domain)
            if not m.enums or not m.types:
                continue
            parent_entity = entity_by_cat.get(cat)
            if not parent_entity:
                continue
            for ed in m.enums:
                enum_ent = entity_by_cat.get(ed.name)
                if not enum_ent:
                    continue
                # Find the field name that references this enum
                field_label = "uses"
                for td in m.types:
                    for fi in td.fields:
                        if fi.local_ref == ed.name:
                            field_label = fi.name
                            break
                lines.append(f"    {parent_entity} }}o--|| {enum_ent} : {field_label}")

    # Non-device domain relationships — shared enum refs are handled by
    # matching refN annotations (no relationship lines needed for those)
    if not central:
        # Per-subtype enums (not shared — these DO get relationship lines)
        subtype_enum_entities: dict[str, list[tuple[str, str]]] = {}
        for name, cat in enum_source_cat.items():
            if cat != "common" and name in entity_by_cat:
                subtype_enum_entities.setdefault(cat, []).append(
                    (name, entity_by_cat[name])
                )

        # Collect subtype entities (exclude enum-only entities)
        all_enum_ents = {ent for pairs in subtype_enum_entities.values() for _, ent in pairs}
        subtype_entities = [
            (cat, entity_by_cat[cat])
            for cat in cat_order
            if cat != "common" and cat in entity_by_cat
            and entity_by_cat[cat] not in all_enum_ents
        ]
        shared_entity = entity_by_cat.get("_shared")

        # Each subtype extends the shared fields
        if shared_entity:
            for _cat, subtype in subtype_entities:
                lines.append(f"    {shared_entity} ||--|| {subtype} : extends")

        # Per-subtype enum relationships (1:1, one line each)
        for cat, subtype in subtype_entities:
            for enum_name, enum_ent in subtype_enum_entities.get(cat, []):
                lines.append(f"    {subtype} }}o--|| {enum_ent} : classifiedBy")

    if extra_edges:
        lines.extend(extra_edges)

    lines.append("")
    return "\n".join(lines) + "\n"


# ── PlantUML generation ──────────────────────────────────────────────

_PUML_SKIN = """\
!pragma layout smetana
skinparam linetype ortho
skinparam classAttributeIconSize 0
skinparam classFontSize 14
skinparam classAttributeFontSize 11
skinparam classFontStyle bold
skinparam dpi 150
skinparam shadowing false
skinparam roundcorner 8
skinparam class {
  BackgroundColor #FEFEFE
  BorderColor #333333
  ArrowColor #555555
}"""


def _puml_field(f: FieldInfo) -> str:
    prefix = "+" if f.required else " "
    t = f.type_display
    if f.is_array:
        t = f"list<{t}>"
    return f"  {prefix}{f.name}: {t}"


def _puml_class(td: TypeDef) -> list[str]:
    lines = [f"class {td.name} {{"]
    for f in td.fields:
        lines.append(_puml_field(f))
    lines.append("}")
    return lines


def _puml_enum(ed: EnumDef) -> list[str]:
    lines = [f"enum {ed.name} {{"]
    for v in ed.values:
        lines.append(f"  {v}")
    lines.append("}")
    return lines


def _puml_union(ud: UnionDef) -> list[str]:
    return [
        f'class {ud.name} <<union>> {{',
        f"  discriminator: {ud.discriminator}",
        "}",
    ]


def _puml_relationships(model: SchemaModel) -> list[str]:
    lines: list[str] = []
    known = _all_type_names(model) | set(model.external_stubs)

    for ud in model.unions:
        for v in ud.variants:
            lines.append(f"{ud.name} <|-- {v}")

    for td in model.types:
        for f in td.fields:
            target = f.local_ref or f.external_type
            if target and target in known and target != td.name:
                arrow = "-->" if f.local_ref else "..>"
                lines.append(f'{td.name} {arrow} {target} : {f.name}')
    return lines


def emit_puml_schema(model: SchemaModel, page_size: str = "A4") -> str:
    if page_size == "A3":
        scale = "scale max 4961*3508"
    else:
        scale = "scale max 3508*2480"

    lines = [
        "@startuml",
        f"title {model.title}",
        _PUML_SKIN,
        scale,
        "",
    ]

    for td in model.types:
        lines.extend(_puml_class(td))
        lines.append("")
    for ed in model.enums:
        lines.extend(_puml_enum(ed))
        lines.append("")
    for ud in model.unions:
        lines.extend(_puml_union(ud))
        lines.append("")

    if model.external_stubs:
        for ext_name, source in sorted(model.external_stubs.items()):
            tag = source.removesuffix("-v1.0.json")
            lines.append(f'class {ext_name} <<{tag}>> #EEEEEE')
        lines.append("")

    rels = _puml_relationships(model)
    if rels:
        lines.extend(rels)
        lines.append("")

    lines.append("@enduml")
    return "\n".join(lines) + "\n"


def emit_puml_domain(domain: str, models: list[SchemaModel]) -> str:
    title = _pretty_domain(domain)
    lines = [
        "@startuml",
        f"title {title}",
        _PUML_SKIN,
        "scale max 4961*3508",
        "",
    ]

    all_externals: dict[str, str] = {}
    for m in models:
        lines.append(f"' ── {m.stem} ──")
        for td in m.types:
            lines.extend(_puml_class(td))
            lines.append("")
        for ed in m.enums:
            lines.extend(_puml_enum(ed))
            lines.append("")
        for ud in m.unions:
            lines.extend(_puml_union(ud))
            lines.append("")
        all_externals.update(m.external_stubs)

    if all_externals:
        lines.append("' ── External types ──")
        for ext_name, source in sorted(all_externals.items()):
            tag = source.removesuffix("-v1.0.json")
            lines.append(f'class {ext_name} <<{tag}>> #EEEEEE')
        lines.append("")

    for m in models:
        rels = _puml_relationships(m)
        if rels:
            lines.extend(rels)

    lines.append("")
    lines.append("@enduml")
    return "\n".join(lines) + "\n"


# ── PNG rendering ────────────────────────────────────────────────────

def _puppeteer_config() -> Path:
    """Write a temporary puppeteer config for mermaid-cli (--no-sandbox)."""
    tmp = Path(tempfile.gettempdir()) / "mmdc-puppeteer.json"
    if not tmp.exists():
        tmp.write_text('{"args": ["--no-sandbox"]}\n')
    return tmp


def render_png(mmd_path: Path, png_path: Path, width: int) -> bool:
    config = _puppeteer_config()
    try:
        result = subprocess.run(
            [
                "npx", "--yes", "@mermaid-js/mermaid-cli@10",
                "-i", str(mmd_path),
                "-o", str(png_path),
                "-w", str(width),
                "--backgroundColor", "white",
                "-p", str(config),
            ],
            capture_output=True, text=True, timeout=120,
        )
        if result.returncode == 0:
            return True
        print(f"    mmdc error: {result.stderr.strip()[:200]}")
        return False
    except FileNotFoundError:
        print("    npx / @mermaid-js/mermaid-cli not available")
        return False
    except subprocess.TimeoutExpired:
        print(f"    mmdc timed out for {mmd_path.name}")
        return False


# ── Main ─────────────────────────────────────────────────────────────

def main():
    parser = argparse.ArgumentParser(
        description="Generate diagrams from the emitted JSON Schemas",
    )
    parser.add_argument(
        "--format", choices=["mermaid", "uml", "png", "all"], default="all",
        help="Output format (default: all)",
    )
    parser.add_argument("--domain", type=str, help="Single domain only")
    parser.add_argument("--domain-only", action="store_true",
                        help="Emit only the per-domain aggregate diagrams (skip per-schema)")
    parser.add_argument("--list-domains", action="store_true")
    parser.add_argument("--output-dir", type=str, default=None)
    args = parser.parse_args()

    schema_files = sorted(SCHEMA_DIR.glob("**/*.json"))
    if not schema_files:
        print(f"No JSON schemas in {SCHEMA_DIR}")
        sys.exit(1)

    models = [parse_schema(f) for f in schema_files]
    domains = group_by_domain(models)

    if args.list_domains:
        print("Available domains:")
        for name, dm in sorted(domains.items()):
            schemas = ", ".join(m.stem for m in dm)
            print(f"  {name:20s} ({len(dm)} schemas): {schemas}")
        return

    if args.domain:
        if args.domain not in domains:
            print(f"Unknown domain '{args.domain}'. Available: {', '.join(sorted(domains))}")
            sys.exit(1)
        selected = {args.domain: domains[args.domain]}
        models = selected[args.domain]
        domains = selected

    output_dir = Path(args.output_dir) if args.output_dir else DIAGRAM_DIR
    mermaid_dir = output_dir / "mermaid"
    uml_dir = output_dir / "uml"
    png_dir = output_dir / "png"

    do_mermaid = args.format in ("mermaid", "all")
    do_uml = args.format in ("uml", "all")
    do_png = args.format in ("png", "all")

    if do_mermaid:
        mermaid_dir.mkdir(parents=True, exist_ok=True)
    if do_uml:
        uml_dir.mkdir(parents=True, exist_ok=True)
    if do_png:
        png_dir.mkdir(parents=True, exist_ok=True)
        # Ensure mermaid source exists for PNG rendering
        mermaid_dir.mkdir(parents=True, exist_ok=True)

    def _ensure_domain_subdir(base: Path, domain: str) -> Path:
        sub = base / domain
        sub.mkdir(parents=True, exist_ok=True)
        return sub

    # ── Per-schema diagrams ──
    if not args.domain_only:
        print("\n[Per-schema diagrams]")
        for m in models:
            print(f"  {m.filename}")
            domain = domain_of(m.stem)
            if do_mermaid or do_png:
                mmd = emit_mermaid_schema(m)
                mmd_dir = _ensure_domain_subdir(mermaid_dir, domain)
                (mmd_dir / f"{m.stem}.mmd").write_text(mmd)
                if do_mermaid:
                    print(f"    mermaid/{domain}/{m.stem}.mmd")
            if do_uml:
                puml = emit_puml_schema(m, page_size="A4")
                puml_sub = _ensure_domain_subdir(uml_dir, domain)
                (puml_sub / f"{m.stem}.puml").write_text(puml)
                print(f"    uml/{domain}/{m.stem}.puml")
            if do_png:
                mmd_path = mermaid_dir / domain / f"{m.stem}.mmd"
                png_sub = _ensure_domain_subdir(png_dir, domain)
                png_path = png_sub / f"{m.stem}.png"
                if render_png(mmd_path, png_path, width=2480):
                    print(f"    png/{domain}/{m.stem}.png")

    # ── Per-domain diagrams ──
    print("\n[Per-domain diagrams]")
    for domain_name, domain_models in sorted(domains.items()):
        print(f"  {domain_name} ({len(domain_models)} schemas)")
        if do_mermaid or do_png:
            mmd = emit_mermaid_domain(domain_name, domain_models)
            mmd_sub = _ensure_domain_subdir(mermaid_dir, domain_name)
            (mmd_sub / f"domain_{domain_name}.mmd").write_text(mmd)
            if do_mermaid:
                print(f"    mermaid/{domain_name}/domain_{domain_name}.mmd")
        if do_uml:
            puml = emit_puml_domain(domain_name, domain_models)
            puml_sub = _ensure_domain_subdir(uml_dir, domain_name)
            (puml_sub / f"domain_{domain_name}.puml").write_text(puml)
            print(f"    uml/{domain_name}/domain_{domain_name}.puml")
        if do_png:
            mmd_path = mermaid_dir / domain_name / f"domain_{domain_name}.mmd"
            png_sub = _ensure_domain_subdir(png_dir, domain_name)
            png_path = png_sub / f"domain_{domain_name}.png"
            if render_png(mmd_path, png_path, width=3600):
                print(f"    png/{domain_name}/domain_{domain_name}.png")

    print(f"\nDone. Output in {output_dir}/")


if __name__ == "__main__":
    main()
