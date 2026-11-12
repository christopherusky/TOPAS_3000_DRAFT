# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.2
# Description: CCTV requestStatus command and response: discriminator
#              dispatch, unknown-kind rejection, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.cctv.command and response."""

from datetime import datetime, timezone

import pytest
from pydantic import TypeAdapter, ValidationError

from topas_3000_model.cctv.command import (
    CctvCommand,
    RequestCctvStatusCommand,
)
from topas_3000_model.cctv.response import CctvCommandResponse
from topas_3000_model.common.command import CommandResponseStatus


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)
_TA = TypeAdapter(CctvCommand)


def test_request_status_minimal():
    c = RequestCctvStatusCommand(
        cctv_id="cam-001", command_correlation_id=1, request_time=NOW,
    )
    assert c.kind == "requestStatus"


def test_discriminator_routes_request_status():
    obj = _TA.validate_python({
        "kind": "requestStatus",
        "cctvId": "cam-001",
        "commandCorrelationId": 1,
        "requestTime": NOW,
    })
    assert isinstance(obj, RequestCctvStatusCommand)


def test_discriminator_rejects_unknown_kind():
    with pytest.raises(ValidationError):
        _TA.validate_python({
            "kind": "panTilt",
            "cctvId": "cam-001",
            "commandCorrelationId": 1,
            "requestTime": NOW,
        })


def test_request_status_dumps_camel_case():
    c = RequestCctvStatusCommand(
        cctv_id="cam-001", command_correlation_id=1, request_time=NOW,
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["cctvId"] == "cam-001"
    assert dumped["kind"] == "requestStatus"


def test_command_response_minimal():
    r = CctvCommandResponse(
        cctv_id="cam-001", command_correlation_id=1,
        status=CommandResponseStatus.SUCCESS, response_time=NOW,
    )
    assert r.status == CommandResponseStatus.SUCCESS


def test_command_response_dumps_camel_case():
    r = CctvCommandResponse(
        cctv_id="cam-001", command_correlation_id=1,
        status=CommandResponseStatus.SUCCESS, response_time=NOW,
    )
    dumped = r.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["cctvId"] == "cam-001"
    assert dumped["status"] == "success"
