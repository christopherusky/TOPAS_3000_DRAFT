# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-05
# Version:     1.0
# Description: The data-message envelope checker (smoketest/envelope.py): it
#              follows the topasEnvelope trait in asyncapi.yaml, refuses a
#              non-conformant header set, and every smoke publisher builds its
#              headers through it. No broker needed; envelope_conformance.py
#              runs the same rules against a live one.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for the data-message envelope checker."""

import ast
import sys
import uuid
from pathlib import Path

import pytest
import yaml

SMOKE = Path(__file__).resolve().parent.parent / "asyncapi_clients" / "smoketest"
sys.path.insert(0, str(SMOKE))

from envelope import data_headers, validate_data_headers  # noqa: E402
from envelope_defect_publisher import DEFECTS  # noqa: E402

CHANNEL = "detector.measurement"
SYNC_ID = str(uuid.uuid4())
PUBLISHERS = [
    "publisher.py", "fault_publisher.py", "traffic_signal_publisher.py",
    "transport_link_publisher.py", "network_publisher.py", "sync_delta_publisher.py",
]


def test_a_delta_envelope_is_valid():
    assert validate_data_headers(data_headers(CHANNEL, "delta"), CHANNEL) == []


def test_a_requested_sync_envelope_is_valid():
    props = data_headers(CHANNEL, "sync", sync_id=SYNC_ID, sequence=2, total=3, request_id=0)
    assert validate_data_headers(props, CHANNEL) == []
    assert props["x-sync-request-id"] == 0  # zero is a valid request id, not "absent"


def test_a_republication_has_no_request_id():
    props = data_headers(CHANNEL, "sync", sync_id=SYNC_ID, sequence=1, total=1)
    assert "x-sync-request-id" not in props
    assert validate_data_headers(props, CHANNEL) == []


@pytest.mark.parametrize("name", sorted(DEFECTS))
def test_every_defect_the_live_gate_injects_is_reported(name):
    """The offline twin of the live negative self-test: a defect no rule catches would
    let the live gate pass for the wrong reason."""
    assert validate_data_headers(DEFECTS[name], CHANNEL), name


def test_the_checker_follows_the_trait_in_the_document():
    """The rules are read from asyncapi.yaml, not copied, so a change there changes the check."""
    doc = yaml.safe_load((SMOKE.parents[1] / "asyncapi.yaml").read_text())
    trait = doc["components"]["messageTraits"]["topasEnvelope"]["headers"]
    assert set(trait["required"]) == {"x-message-mode", "x-content-type"}
    for key in trait["required"]:
        errors = validate_data_headers({k: v for k, v in data_headers(CHANNEL).items() if k != key}, CHANNEL)
        assert any(key in e for e in errors), key


def test_builder_refuses_to_return_a_non_conformant_set():
    with pytest.raises(ValueError, match="non-conformant"):
        data_headers(CHANNEL, "sync")  # no triple
    with pytest.raises(ValueError, match="UUID"):
        data_headers(CHANNEL, "sync", sync_id="SYNC-1234abcd", sequence=1, total=1)
    with pytest.raises(ValueError, match="exceeds"):
        data_headers(CHANNEL, "sync", sync_id=SYNC_ID, sequence=4, total=3)


def test_content_type_must_name_the_channel_it_was_sent_to():
    props = data_headers("vms.measurement")
    assert validate_data_headers(props, "vms.measurement") == []
    assert any("does not name channel" in e for e in validate_data_headers(props, CHANNEL))


@pytest.mark.parametrize("script", PUBLISHERS)
def test_every_smoke_publisher_builds_its_headers_with_the_checker(script):
    """The publishers once omitted x-content-type, sent x-originator on data messages, and
    sent a sync message with no batch triple. Each now goes through data_headers."""
    tree = ast.parse((SMOKE / script).read_text())
    calls = [n for n in ast.walk(tree) if isinstance(n, ast.Call)
             and getattr(n.func, "id", getattr(n.func, "attr", "")) == "data_headers"]
    assert calls, f"{script} does not use data_headers"
    source = (SMOKE / script).read_text()
    assert '"x-originator"' not in source, f"{script} puts x-originator on a data message"
