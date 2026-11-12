# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.0
# Description: Specification against implementation: the spec's channel
#              table equals the channels in asyncapi.yaml, every message
#              property and command status the document defines is stated in
#              the spec, and the superseded wire-format statements stay out.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""The prose specification must not drift from the implemented interface (H1, H2)."""

import re
from pathlib import Path

import pytest
import yaml

from topas_3000_model.common.command import CommandResponseStatus

ROOT = Path(__file__).resolve().parent.parent
SPEC = (ROOT / "README.md").read_text()
DOC = yaml.safe_load((ROOT / "asyncapi.yaml").read_text())
CHANNELS = DOC["channels"]
MESSAGES = DOC["components"]["messages"]


def _spec_channel_rows(text: str) -> dict[str, str]:
    """Channel name -> reply schema filename ('' when none), from the spec table."""
    block = text.split("<!-- channels:begin -->")[1].split("<!-- channels:end -->")[0]
    rows = {}
    for line in block.splitlines():
        m = re.match(r"\| `([a-z0-9-]+\.[a-z0-9-]+)` \| `([^`]+)` \| (.*?) \|", line)
        if m:
            reply = re.search(r"`([^`]+)`", m.group(3))
            rows[m.group(1)] = (m.group(2), reply.group(1) if reply else "")
    return rows


def _document_channel_rows() -> dict[str, tuple[str, str]]:
    import os

    out = {}
    for name, chan in CHANNELS.items():
        ref = chan["subscribe"]["message"]["$ref"].split("/")[-1]
        schema = os.path.basename(MESSAGES[ref]["payload"]["$ref"])
        reply = ""
        if "x-amqp1-request-reply" in chan:
            rref = chan["x-amqp1-request-reply"]["responseMessage"]["$ref"].split("/")[-1]
            reply = os.path.basename(MESSAGES[rref]["payload"]["$ref"])
        out[name] = (schema, reply)
    return out


def test_spec_channel_table_equals_the_document():
    assert _spec_channel_rows(SPEC) == _document_channel_rows()


def test_the_spec_states_the_channel_count():
    assert f"There are {len(CHANNELS)} channels" in SPEC


def test_every_message_property_is_stated_in_the_spec():
    for trait_name, trait in DOC["components"]["messageTraits"].items():
        for prop in trait["headers"]["properties"]:
            assert f"`{prop}`" in SPEC, f"{prop} ({trait_name}) is in asyncapi.yaml but not the spec"


def test_every_message_trait_is_named_in_the_spec():
    for trait_name in DOC["components"]["messageTraits"]:
        assert f"`{trait_name}`" in SPEC, trait_name


def test_the_sender_role_extension_is_described_and_complete():
    assert "`x-sender`" in SPEC
    roles = {c["x-sender"] for c in CHANNELS.values()}
    assert roles == {"data-publisher", "data-subscriber"}
    for role in roles:
        assert f"`{role}`" in SPEC


def test_every_command_status_is_listed_in_the_message_structure_table():
    row = next(l for l in SPEC.splitlines() if l.startswith("| `status` | Response |"))
    for status in CommandResponseStatus:
        assert f"`{status.value}`" in row, status.value


def test_the_request_reply_extension_is_described():
    assert "x-amqp1-request-reply" in SPEC


@pytest.mark.parametrize(
    "superseded",
    [
        "MUST subscribe to command response topics",
        "Data Publishers MUST subscribe to command request topics",
        "published as catalogue entries on the `faultSupport` channel",
        "Request full publish<br/>(vendor-specific)",
        '"messageType": "trafficSignalMeasurement"',
        "Channel names follow the pattern `{domain}{Category}`",
        "`trafficSignalCommandResponse` | `traffic_signal_command_response-v1.0.json`",
        "specifications.utmcv2.co.uk",
        "Stage 1 schemas shipped**: 34",
        "exactly as a sync batch of one object",
    ],
)
def test_superseded_statements_are_gone(superseded):
    assert superseded not in SPEC


def test_the_spec_names_no_channel_the_document_lacks():
    """Every dotted channel name written in code ticks anywhere in the spec exists."""
    named = set(re.findall(r"`((?:[a-z]+(?:-[a-z]+)*)\.(?:[a-z]+(?:-[a-z]+)*))`", SPEC))
    # Only names whose domain is a real domain are channel claims.
    domains = {c.split(".")[0] for c in CHANNELS}
    claimed = {n for n in named if n.split(".")[0] in domains}
    assert claimed <= set(CHANNELS), sorted(claimed - set(CHANNELS))
