#!/usr/bin/env python3
# ──────────────────────────────────────────────────────────────────────
# Changes:    New file — generates AsyncAPI 2.6 from FILE_SPECS + template
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-05
# Version:    1.3
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Generate AsyncAPI 2.6 document from FILE_SPECS and template.

Reads the hand-authored asyncapi_template.yaml for server config,
security, and message traits. Generates channel definitions from the
FILE_SPECS registry in bin/emit.py using dot-separated hierarchical
channel naming (A8).

Usage: python bin/emit_asyncapi.py
"""

from __future__ import annotations

import json
import sys
from copy import deepcopy
from pathlib import Path

import yaml

ROOT = Path(__file__).resolve().parent.parent
sys.path.insert(0, str(ROOT))

from bin.emit import FILE_SPECS, _resolve_domain  # noqa: E402
from bin.generate_diagrams import DOMAIN_PREFIXES, domain_of  # noqa: E402

TEMPLATE_PATH = ROOT / "asyncapi_template.yaml"
OUTPUT_PATH = ROOT / "asyncapi.yaml"
CANONICAL_SCHEMAS_DIR = ROOT / "Schemas"
TRANSFORMED_SCHEMAS_DIR = ROOT / "Schemas_asyncapi"
SCHEMA_REL_DIR = "Schemas_asyncapi"


def _stem(filename: str) -> str:
    return filename.removesuffix("-v1.0.json")


def _schema_to_channel(filename: str) -> str:
    """Convert schema filename to dot-separated channel name.

    detector_measurement-v1.0.json -> detector.measurement
    traffic_signal_command-v1.0.json -> traffic-signal.command
    traffic_event_accident-v1.0.json -> traffic-event.accident
    """
    stem = _stem(filename)
    domain = domain_of(stem)
    category = stem.removeprefix(f"{domain}_") if stem != domain else ""
    if not category:
        return domain.replace("_", "-")
    domain_part = domain.replace("_", "-")
    category_part = category.replace("_", "-")
    return f"{domain_part}.{category_part}"


def _schema_to_content_type(filename: str) -> str:
    """Convert schema filename to x-content-type value.

    detector_measurement-v1.0.json -> detector.measurement-v1.0
    """
    stem = _stem(filename)
    domain = domain_of(stem)
    category = stem.removeprefix(f"{domain}_") if stem != domain else ""
    if not category:
        return f"{domain.replace('_', '-')}-v1.0"
    return f"{domain.replace('_', '-')}.{category.replace('_', '-')}-v1.0"


def _pascal_to_message_name(spec_title: str) -> str:
    """Extract a PascalCase message name from the FileSpec title.

    'UTMC Detector Measurement' -> 'DetectorMeasurement'
    """
    words = spec_title.removeprefix("UTMC ").split()
    return "".join(w.capitalize() for w in words)


def _downgrade_discriminators(node) -> None:
    """Rewrite OpenAPI 3.x discriminator objects to AsyncAPI 2.6 string form.

    AsyncAPI 2.6's Schema Object (JSON Schema Draft-07 dialect) expects
    ``discriminator`` to be a plain string (the property name). Pydantic v2
    and OpenAPI 3.x emit it as ``{propertyName, mapping}``. Validators
    reject the object form with ``"discriminator" property type must be
    string``. The ``mapping`` hint is OpenAPI-only; AsyncAPI validators
    resolve the variant via ``oneOf`` + ``const`` on the discriminator
    property, which our schemas already express.
    """
    if isinstance(node, dict):
        disc = node.get("discriminator")
        if isinstance(disc, dict) and "propertyName" in disc:
            node["discriminator"] = disc["propertyName"]
        for value in node.values():
            _downgrade_discriminators(value)
    elif isinstance(node, list):
        for value in node:
            _downgrade_discriminators(value)


def _strip_2020_12_meta_schema(root: dict) -> None:
    """Remove the top-level ``$schema`` draft-2020-12 declaration.

    AsyncAPI 2.6 payloads declare ``schemaFormat: draft-07`` at the message
    level. Leaving a ``$schema: .../draft/2020-12/schema`` header inside
    the referenced file contradicts that and some validators flag it.
    """
    if isinstance(root, dict) and "$schema" in root:
        del root["$schema"]


def generate_transformed_schemas() -> int:
    """Copy every canonical schema to ``Schemas_asyncapi/<domain>/`` with
    discriminator objects downgraded to string form for AsyncAPI 2.6
    validators. Canonical ``Schemas/`` files remain unchanged and retain
    the richer OpenAPI 3.x form for OpenAPI consumers.

    Per-domain subdirectory layout mirrors ``Schemas/`` so the canonical
    ``"../{domain}/{file}.json"`` cross-file ``$ref`` form resolves
    unchanged (no ref-rewriting needed).
    """
    import shutil

    # Messages come from the live registry but the schemas are read from disk, so a
    # stale `Schemas/` (not regenerated after a scope or model change) would give an
    # asyncapi.yaml that disagrees with Schemas_asyncapi/. Refuse rather than emit it.
    on_disk = {p.name for p in CANONICAL_SCHEMAS_DIR.rglob("*.json")}
    expected = {s.filename for s in FILE_SPECS}
    if on_disk != expected:
        raise SystemExit(
            "Schemas/ does not match the live registry "
            f"(missing: {sorted(expected - on_disk)[:3]}, unexpected: {sorted(on_disk - expected)[:3]}). "
            "Run `make schemas` first."
        )
    if TRANSFORMED_SCHEMAS_DIR.exists():
        shutil.rmtree(TRANSFORMED_SCHEMAS_DIR)
    TRANSFORMED_SCHEMAS_DIR.mkdir()
    count = 0
    for src in sorted(CANONICAL_SCHEMAS_DIR.rglob("*.json")):
        data = json.loads(src.read_text())
        _strip_2020_12_meta_schema(data)
        _downgrade_discriminators(data)
        domain = src.parent.name
        out_dir = TRANSFORMED_SCHEMAS_DIR / domain
        out_dir.mkdir(parents=True, exist_ok=True)
        (out_dir / src.name).write_text(
            json.dumps(data, indent=2) + "\n"
        )
        count += 1
    return count


def _trait_for(spec) -> str:
    """Pick the message trait: data messages carry the data envelope; request
    and reply messages are correlated by reply-to and carry none of it."""
    if getattr(spec, "reply_only", False) or getattr(spec, "reply_filename", None):
        return "topasRequestReply"
    return "topasEnvelope"


def generate_messages() -> dict:
    """Generate AsyncAPI message definitions for components/messages."""
    messages = {}

    for spec in FILE_SPECS:
        if spec.canonical is None:
            continue

        message_name = _pascal_to_message_name(spec.title)
        payload_ref = f"{SCHEMA_REL_DIR}/{_resolve_domain(spec)}/{spec.filename}"

        messages[message_name] = {
            "name": message_name,
            "title": spec.title,
            "schemaFormat": "application/schema+json;version=draft-07",
            "traits": [
                {"$ref": f"#/components/messageTraits/{_trait_for(spec)}"},
            ],
            "payload": {
                "$ref": payload_ref,
            },
        }

    return dict(sorted(messages.items()))


def _request_reply_binding(request_spec, reply_spec) -> dict:
    """Describe request/reply addressing for a channel (A11/A12).

    AsyncAPI 2.6 cannot express this: its AMQP 1.0 binding is effectively
    empty and first-class request/reply only arrived in 3.0. Specification
    extensions are permitted, so this is the least-wrong representation —
    generic 2.6 tooling ignores an `x-` key, whereas a response *channel*
    would describe an address that does not exist at runtime and which
    provisioning tooling could reasonably act on.

    Maps onto `operation.reply` when this document moves to AsyncAPI 3.x.
    """
    reply_message = _pascal_to_message_name(reply_spec.title)
    # Application-level correlation is the request payload's own identifier;
    # commands and syncs name it differently.
    correlation_property = (
        "syncCorrelationId"
        if "_sync_request-" in request_spec.filename
        else "commandCorrelationId"
    )
    return {
        "responseMessage": {
            "$ref": f"#/components/messages/{reply_message}",
        },
        "replyAddress": {
            "requestProperty": "reply-to",
            "required": True,
            "permittedTypes": ["dynamic", "provisioned"],
            # Which of the two a generator should demonstrate. Without this a
            # generator knows both are legal but not which to emit; with it the
            # behaviour is deterministic — create a dynamic receiver, wait for
            # the broker-assigned address, put it in reply-to.
            "preferredType": "dynamic",
        },
        "responseDestination": {
            "messageProperty": "to",
            "valueFrom": "request.reply-to",
        },
        "transportCorrelation": {
            "requestProperty": "message-id",
            "responseProperty": "correlation-id",
        },
        "applicationCorrelation": {
            "payloadProperty": correlation_property,
        },
        "fanout": False,
    }


def generate_channels() -> dict:
    """Generate AsyncAPI channel definitions from FILE_SPECS.

    Reply-only specs get no channel. Their responses are addressed to the
    requester's reply node (A11), so a channel would describe an address
    that never exists.
    """
    by_filename = {s.filename: s for s in FILE_SPECS}
    channels = {}

    for spec in FILE_SPECS:
        # Skip library files (no canonical type = no channel)
        if spec.canonical is None:
            continue
        # Skip replies — addressed, not published (A11/A12).
        if getattr(spec, "reply_only", False):
            continue

        channel_name = _schema_to_channel(spec.filename)
        message_name = _pascal_to_message_name(spec.title)
        message_ref = f"#/components/messages/{message_name}"

        reply_filename = getattr(spec, "reply_filename", None)
        channel_def = {
            "description": spec.description,
            # Every channel declares both publish and subscribe, because the
            # document describes the broker rather than one participant, so the
            # direction of travel cannot be read from them. This names the role
            # that sends: a request is sent by a Data Subscriber, everything
            # else by a Data Publisher.
            "x-sender": "data-subscriber" if reply_filename else "data-publisher",
            "subscribe": {
                "message": {
                    "$ref": message_ref,
                },
            },
            "publish": {
                "message": {
                    "$ref": message_ref,
                },
            },
        }

        if reply_filename:
            reply_spec = by_filename.get(reply_filename)
            if reply_spec is None:
                raise ValueError(
                    f"{spec.filename} names reply {reply_filename}, which is "
                    "not in FILE_SPECS"
                )
            channel_def["x-amqp1-request-reply"] = _request_reply_binding(
                spec, reply_spec
            )

        channels[channel_name] = channel_def

    return dict(sorted(channels.items()))


def generate_asyncapi() -> dict:
    """Generate complete AsyncAPI document."""
    with open(TEMPLATE_PATH) as f:
        doc = yaml.safe_load(f)

    doc["channels"] = generate_channels()
    doc.setdefault("components", {})["messages"] = generate_messages()
    return doc


def emit_asyncapi():
    """Generate and write the AsyncAPI document."""
    transformed_count = generate_transformed_schemas()
    doc = generate_asyncapi()

    yaml_output = yaml.dump(
        doc,
        default_flow_style=False,
        sort_keys=False,
        allow_unicode=True,
        width=120,
    )

    OUTPUT_PATH.write_text(yaml_output)
    channel_count = len(doc.get("channels", {}))
    print(
        f"Wrote {OUTPUT_PATH} ({channel_count} channels, "
        f"{transformed_count} transformed schemas in {TRANSFORMED_SCHEMAS_DIR.name}/)"
    )


if __name__ == "__main__":
    emit_asyncapi()
