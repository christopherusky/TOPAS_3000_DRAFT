# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/envelope.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-05
# Version:     1.0
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""The data-message envelope, as a checker and as a builder.

The specification says every data message MUST carry ``x-message-mode`` and
``x-content-type``, that a ``sync`` message MUST also carry the
``x-sync-id`` / ``x-sync-sequence`` / ``x-sync-total`` triple, that
``x-geo-tile-system`` MUST accompany ``x-geo-tile``, and that
``x-originator`` belongs to request/reply messages only (*Message
Properties*). Until now nothing asserted any of it: the smoke publishers
omitted ``x-content-type`` altogether and no subscriber looked.

This module reads the rules from the AsyncAPI document itself (the
``topasEnvelope`` trait), so the check cannot drift from the specification
the way a second copy of the rules would, and adds the rules the trait's JSON
Schema cannot state:

* ``x-originator`` is not a data-message property;
* ``x-content-type`` names the channel the message was published to;
* ``x-sync-id`` really is a UUID (the trait says ``format: uuid``, which a
  JSON Schema validator treats as an annotation);
* ``x-sync-sequence`` does not exceed ``x-sync-total``;
* ``x-sync-request-id`` appears only on a sync message.

``data_headers`` builds a valid set and refuses to return an invalid one, so
a publisher written against it cannot send a non-conformant message.
"""
from __future__ import annotations

import re
import uuid
from functools import lru_cache
from pathlib import Path

import jsonschema
import yaml

ROOT = Path(__file__).resolve().parents[2]
ASYNCAPI = ROOT / "asyncapi.yaml"
CONTENT_TYPE = re.compile(r"^[a-z0-9-]+\.[a-z0-9-]+-v\d+\.\d+$")
DEFAULT_VERSION = "v1.0"


@lru_cache(maxsize=1)
def _trait_schema() -> dict:
    doc = yaml.safe_load(ASYNCAPI.read_text())
    return doc["components"]["messageTraits"]["topasEnvelope"]["headers"]


def validate_data_headers(props: dict | None, channel: str | None = None) -> list[str]:
    """Every way ``props`` breaks the data-message envelope; empty if it holds."""
    props = dict(props or {})
    errors: list[str] = []

    # No format checker: `format` is only an annotation to a JSON Schema validator
    # and which formats it knows varies by version, so the UUID rule is explicit below.
    validator = jsonschema.Draft7Validator(_trait_schema())
    errors += [
        (f"{'/'.join(map(str, e.path))}: " if e.path else "") + e.message
        for e in validator.iter_errors(props)
    ]

    if "x-originator" in props:
        errors.append("x-originator is carried by request/reply messages only, not data messages")

    content_type = props.get("x-content-type")
    if isinstance(content_type, str):
        if not CONTENT_TYPE.match(content_type):
            errors.append(f"x-content-type {content_type!r} is not '<channel>-vN.N'")
        elif channel is not None and content_type.rsplit("-v", 1)[0] != channel:
            errors.append(f"x-content-type {content_type!r} does not name channel {channel!r}")

    sync_id = props.get("x-sync-id")
    if isinstance(sync_id, str):
        try:
            uuid.UUID(sync_id)
        except ValueError:
            errors.append(f"x-sync-id {sync_id!r} is not a UUID")

    seq, total = props.get("x-sync-sequence"), props.get("x-sync-total")
    if isinstance(seq, int) and isinstance(total, int) and seq > total:
        errors.append(f"x-sync-sequence {seq} exceeds x-sync-total {total}")

    if "x-sync-request-id" in props and props.get("x-message-mode") != "sync":
        errors.append("x-sync-request-id is only valid on an x-message-mode 'sync' message")

    return errors


def data_headers(
    channel: str,
    mode: str = "delta",
    *,
    sync_id: str | None = None,
    sequence: int | None = None,
    total: int | None = None,
    request_id: int | None = None,
    version: str = DEFAULT_VERSION,
) -> dict:
    """A conformant property set for a data message on ``channel``.

    Raises ``ValueError`` rather than return one that is not.
    """
    props: dict = {"x-message-mode": mode, "x-content-type": f"{channel}-{version}"}
    if mode == "sync":
        props["x-sync-id"] = sync_id
        props["x-sync-sequence"] = sequence
        props["x-sync-total"] = total
        if request_id is not None:
            props["x-sync-request-id"] = request_id
    props = {k: v for k, v in props.items() if v is not None}
    errors = validate_data_headers(props, channel)
    if errors:
        raise ValueError("non-conformant data envelope: " + "; ".join(errors))
    return props
