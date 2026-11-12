# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-05
# Version:     1.0
# Description: The requested-sync checks (smoketest/a12_sync.py) on synthetic
#              exchanges, with no broker: a conformant exchange passes and
#              each way of breaking the specification is reported.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for the A12 requested-sync checker."""

import copy
import sys
import uuid
from pathlib import Path

import pytest

SMOKE = Path(__file__).resolve().parent.parent / "asyncapi_clients" / "smoketest"
sys.path.insert(0, str(SMOKE))

from a12_sync import ALL, check  # noqa: E402
from envelope import data_headers  # noqa: E402

CID = 4242
MID = "msg-1"
REPLY = "reply-node-1"


def exchange(ids=ALL, *, status="success") -> dict:
    sync_id = str(uuid.uuid4()) if ids else None
    frames = [
        {
            "properties": data_headers(
                "detector.measurement", "sync",
                sync_id=sync_id, sequence=i, total=len(ids), request_id=CID,
            ),
            "correlation_id": MID, "to": REPLY, "after_complete": False,
            "body": {"detectorId": d},
        }
        for i, d in enumerate(ids, start=1)
    ]
    done = {
        "properties": {}, "correlation_id": MID, "to": REPLY, "after_complete": False,
        "body": {"syncCorrelationId": CID, "syncId": sync_id, "status": status, "objectCount": len(ids)},
    }
    return {"syncCorrelationId": CID, "messageId": MID, "replyAddress": REPLY,
            "frames": frames, "completes": [done]}


def test_a_conformant_exchange_has_no_problems():
    assert check(exchange(), ALL) == []


def test_an_empty_sync_and_a_declined_sync_are_conformant():
    assert check(exchange([]), []) == []
    assert check(exchange([], status="rejected"), [], status="rejected") == []


def mutate(fn):
    res = copy.deepcopy(exchange())
    fn(res)
    return res


MUTATIONS = {
    "wrong total": lambda r: r["frames"][0]["properties"].__setitem__("x-sync-total", 9),
    "request id missing": lambda r: [f["properties"].pop("x-sync-request-id") for f in r["frames"]],
    "wrong request id": lambda r: [f["properties"].__setitem__("x-sync-request-id", 1) for f in r["frames"]],
    "wrong correlation": lambda r: r["frames"][1].__setitem__("correlation_id", "x"),
    "wrong destination": lambda r: r["frames"][0].__setitem__("to", "detector.measurement"),
    "bad sequence": lambda r: [f["properties"].__setitem__("x-sync-sequence", 1) for f in r["frames"]],
    "frame after complete": lambda r: r["frames"][-1].__setitem__("after_complete", True),
    "wrong count": lambda r: r["completes"][0]["body"].__setitem__("objectCount", 4),
    "wrong sync id": lambda r: r["completes"][0]["body"].__setitem__("syncId", str(uuid.uuid4())),
    "no complete": lambda r: r.__setitem__("completes", []),
    "two completes": lambda r: r["completes"].append(copy.deepcopy(r["completes"][0])),
    "a frame missing": lambda r: r["frames"].pop(),
    "mixed batches": lambda r: r["frames"][0]["properties"].__setitem__("x-sync-id", str(uuid.uuid4())),
    "wrong status": lambda r: r["completes"][0]["body"].__setitem__("status", "failed"),
    "no reply node": lambda r: r.__setitem__("replyAddress", None),
}


@pytest.mark.parametrize("name", sorted(MUTATIONS))
def test_each_way_of_breaking_the_exchange_is_reported(name):
    assert check(mutate(MUTATIONS[name]), ALL), name


def test_an_empty_sync_carrying_a_sync_id_is_reported():
    res = exchange([])
    res["completes"][0]["body"]["syncId"] = str(uuid.uuid4())
    assert any("must not carry a syncId" in p for p in check(res, []))
