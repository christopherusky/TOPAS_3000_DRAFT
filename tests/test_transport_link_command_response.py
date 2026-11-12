# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.2
# Description: Transport-link command responses: minimal and full
#              construction, camelCase serialisation, and rejection of extra
#              fields, naive datetimes and negative correlation ids.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.transport_link.response."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.common.command import CommandResponseStatus
from topas_3000_model.transport_link.response import TransportLinkCommandResponse


NOW = datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc)


def test_minimal():
    r = TransportLinkCommandResponse(
        transport_link_id="tl-001",
        command_correlation_id=1,
        status=CommandResponseStatus.SUCCESS,
        response_time=NOW,
    )
    assert r.transport_link_id == "tl-001"
    assert r.command_correlation_id == 1
    assert r.status == CommandResponseStatus.SUCCESS
    assert r.response_time == NOW
    assert r.code is None
    assert r.message is None
    assert r.extensions is None


def test_full():
    r = TransportLinkCommandResponse(
        transport_link_id="tl-001",
        command_correlation_id=1,
        status=CommandResponseStatus.FAILED,
        response_time=NOW,
        code="ERR_TIMEOUT",
        message="The device did not respond within 30 seconds.",
        extensions={"vendor": "acme"},
    )
    assert r.code == "ERR_TIMEOUT"
    assert r.message == "The device did not respond within 30 seconds."
    assert r.extensions == {"vendor": "acme"}


def test_dumps_camel_case():
    r = TransportLinkCommandResponse(
        transport_link_id="tl-001",
        command_correlation_id=1,
        status=CommandResponseStatus.SUCCESS,
        response_time=NOW,
    )
    dumped = r.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert "transportLinkId" in dumped
    assert "commandCorrelationId" in dumped
    assert "responseTime" in dumped
    assert "status" in dumped
    assert dumped["status"] == "success"


def test_rejects_extra_fields():
    with pytest.raises(ValidationError):
        TransportLinkCommandResponse(
            transport_link_id="tl-001",
            command_correlation_id=1,
            status=CommandResponseStatus.SUCCESS,
            response_time=NOW,
            bogus="field",
        )


def test_rejects_naive_datetime():
    with pytest.raises(ValidationError):
        TransportLinkCommandResponse(
            transport_link_id="tl-001",
            command_correlation_id=1,
            status=CommandResponseStatus.SUCCESS,
            response_time=datetime(2026, 4, 11, 10, 0),
        )


def test_rejects_negative_command_correlation_id():
    with pytest.raises(ValidationError):
        TransportLinkCommandResponse(
            transport_link_id="tl-001",
            command_correlation_id=-1,
            status=CommandResponseStatus.SUCCESS,
            response_time=NOW,
        )
