# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.2
# Description: The sync-request payload: minimal and fully-filtered
#              requests, alias round-trip, rejection of unknown fields, naive
#              timestamps, negative correlation ids and empty filter lists,
#              the category enum, and that every data domain emits a sync-
#              request schema while library domains do not.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for the sync-request payload.

Covers the four things every model test in this suite covers —
serialisation, alias round-trip, discriminator/enum dispatch and
`extra="forbid"` rejection — plus the filter semantics the PR #2
reviewers asked for by name.
"""
import json
from pathlib import Path

import pytest
from pydantic import ValidationError

from topas_3000_model.common.sync import (
    SyncCategoryEnum,
    SyncComplete,
    SyncRequest,
    SyncStatus,
)

ROOT = Path(__file__).resolve().parents[1]


def test_minimal_request_needs_only_correlation_and_time():
    """No filters means "the complete current state" — the first-connect case."""
    r = SyncRequest(
        sync_correlation_id=1,
        request_time="2026-08-27T10:00:00Z",
    )
    assert r.categories is None
    assert r.object_ids is None
    assert r.active_only is None


def test_dumps_camel_case():
    r = SyncRequest(sync_correlation_id=7, request_time="2026-08-27T10:00:00Z")
    dumped = r.model_dump()
    assert "syncCorrelationId" in dumped
    assert "requestTime" in dumped
    assert "sync_correlation_id" not in dumped


def test_alias_round_trip():
    payload = {
        "syncCorrelationId": 42,
        "requestTime": "2026-08-27T10:00:00Z",
        "activeOnly": True,
        "subArea": "SA-1",
    }
    r = SyncRequest.model_validate(payload)
    assert r.sync_correlation_id == 42
    assert r.active_only is True
    # mode="json" so the timestamp comes back as the wire string rather
    # than a datetime object.
    assert r.model_dump(mode="json", exclude_none=True) == payload


def test_active_only_faults_request():
    """doryiforgot's "send me every active fault" (thread #26)."""
    r = SyncRequest(
        sync_correlation_id=1,
        request_time="2026-08-27T10:00:00Z",
        categories=[SyncCategoryEnum.FAULT],
        active_only=True,
    )
    dumped = r.model_dump(exclude_none=True)
    assert dumped["categories"] == ["fault"]
    assert dumped["activeOnly"] is True


def test_full_filter_set():
    r = SyncRequest(
        sync_correlation_id=9,
        request_time="2026-08-27T10:00:00Z",
        categories=[SyncCategoryEnum.DEFINITION, SyncCategoryEnum.MEASUREMENT],
        object_ids=["TS-001", "TS-002"],
        sub_area="north",
        type_id="3",
        active_only=False,
        since="2026-08-27T09:00:00Z",
    )
    dumped = r.model_dump(exclude_none=True)
    assert dumped["objectIds"] == ["TS-001", "TS-002"]
    assert dumped["typeId"] == "3"
    assert "since" in dumped


def test_rejects_unknown_field():
    with pytest.raises(ValidationError):
        SyncRequest(
            sync_correlation_id=1,
            request_time="2026-08-27T10:00:00Z",
            everything=True,
        )


def test_rejects_naive_timestamp():
    """Timestamps are timezone-aware everywhere."""
    with pytest.raises(ValidationError):
        SyncRequest(sync_correlation_id=1, request_time="2026-08-27T10:00:00")


def test_rejects_negative_correlation_id():
    with pytest.raises(ValidationError):
        SyncRequest(sync_correlation_id=-1, request_time="2026-08-27T10:00:00Z")


def test_rejects_empty_filter_lists():
    """An empty list is a mistake, not "no filter" — omit the field instead."""
    with pytest.raises(ValidationError):
        SyncRequest(
            sync_correlation_id=1,
            request_time="2026-08-27T10:00:00Z",
            categories=[],
        )
    with pytest.raises(ValidationError):
        SyncRequest(
            sync_correlation_id=1,
            request_time="2026-08-27T10:00:00Z",
            object_ids=[],
        )


def test_category_enum_values():
    assert {e.value for e in SyncCategoryEnum} == {
        "definition",
        "configuration",
        "measurement",
        "fault",
        "type",
        "status",
    }


@pytest.mark.parametrize("domain", [
    "detector", "network", "strategy", "traffic_event", "traffic_signal",
    "transport_link", "transport_route", "vms",
])
def test_every_data_domain_emits_a_sync_request_schema(domain):
    """One channel per data-bearing domain; libraries are excluded."""
    path = ROOT / "Schemas" / domain / f"{domain}_sync_request-v1.0.json"
    assert path.exists(), f"missing sync request schema for {domain}"
    schema = json.loads(path.read_text())
    root = schema["$defs"]["SyncRequest"]
    assert root["required"] == ["syncCorrelationId", "requestTime"]
    assert root["additionalProperties"] is False


def test_library_domains_have_no_sync_request():
    for domain in ("common", "fault_support"):
        assert not list((ROOT / "Schemas" / domain).glob("*sync_request*"))


# ── SyncComplete (A12) ──


def test_empty_result_is_expressible():
    """The reason this message exists: a filtered sync matching nothing."""
    c = SyncComplete(sync_correlation_id=1, status=SyncStatus.SUCCESS,
                     object_count=0, completed_time="2026-08-28T12:00:00Z")
    assert c.object_count == 0
    assert c.sync_id is None


def test_full_batch():
    c = SyncComplete(sync_correlation_id=1234, sync_id="3f2a", status="success",
                     object_count=10037, completed_time="2026-08-28T12:04:11Z")
    d = c.model_dump(mode="json", exclude_none=True)
    assert d["objectCount"] == 10037
    assert d["syncId"] == "3f2a"


def test_rejected_and_failed_carry_a_reason():
    for st in (SyncStatus.REJECTED, SyncStatus.FAILED):
        c = SyncComplete(sync_correlation_id=1, status=st, object_count=0,
                         completed_time="2026-08-28T12:00:00Z",
                         message="rate limited")
        assert c.message == "rate limited"


def test_status_values():
    assert {s.value for s in SyncStatus} == {"success", "rejected", "failed"}


def test_dumps_camel_case_and_forbids_extras():
    c = SyncComplete(sync_correlation_id=1, status="success", object_count=0,
                     completed_time="2026-08-28T12:00:00Z")
    assert "syncCorrelationId" in c.model_dump()
    assert "completedTime" in c.model_dump()
    with pytest.raises(ValidationError):
        SyncComplete(sync_correlation_id=1, status="success", object_count=0,
                     completed_time="2026-08-28T12:00:00Z", nonsense=True)


def test_rejects_negative_count_and_naive_time():
    with pytest.raises(ValidationError):
        SyncComplete(sync_correlation_id=1, status="success", object_count=-1,
                     completed_time="2026-08-28T12:00:00Z")
    with pytest.raises(ValidationError):
        SyncComplete(sync_correlation_id=1, status="success", object_count=0,
                     completed_time="2026-08-28T12:00:00")


def test_rejected_sync_delivered_no_batch():
    with pytest.raises(ValidationError):
        SyncComplete(sync_correlation_id=1, status="rejected", object_count=3,
                     completed_time="2026-08-28T12:00:00Z")
    with pytest.raises(ValidationError):
        SyncComplete(sync_correlation_id=1, status="rejected", object_count=0,
                     sync_id="3f2a", completed_time="2026-08-28T12:00:00Z")
    ok = SyncComplete(sync_correlation_id=1, status="rejected", object_count=0,
                      completed_time="2026-08-28T12:00:00Z", message="rate limited")
    assert ok.sync_id is None


def test_failed_sync_may_report_a_partial_batch():
    """`failed` means the batch began, so a count and sync id are legitimate."""
    c = SyncComplete(sync_correlation_id=1, status="failed", object_count=40,
                     sync_id="3f2a", completed_time="2026-08-28T12:00:00Z")
    assert c.object_count == 40


def test_rejects_duplicate_filter_values():
    with pytest.raises(ValidationError):
        SyncRequest(sync_correlation_id=1, request_time="2026-08-28T12:00:00Z",
                    categories=["measurement", "measurement"])
    with pytest.raises(ValidationError):
        SyncRequest(sync_correlation_id=1, request_time="2026-08-28T12:00:00Z",
                    object_ids=["a", "a"])
