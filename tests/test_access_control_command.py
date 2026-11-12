# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Access-control command and response payloads: enableAccess and
#              disableAccess construction, discriminator dispatch on kind,
#              rejection of unknown kinds and negative correlation ids,
#              response detail fields, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.access_control.command and response."""

from datetime import datetime, timezone

import pytest
from pydantic import TypeAdapter, ValidationError

from topas_3000_model.access_control.command import (
    AccessControlCommand,
    DisableAccessCommand,
    EnableAccessCommand,
)
from topas_3000_model.access_control.response import AccessControlCommandResponse
from topas_3000_model.common.command import CommandResponseStatus


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)
_TA = TypeAdapter(AccessControlCommand)


def test_enable_access_minimal():
    c = EnableAccessCommand(
        access_control_id="ac-001",
        command_correlation_id=1,
        request_time=NOW,
    )
    assert c.kind == "enable"
    assert c.access_control_id == "ac-001"


def test_disable_access_minimal():
    c = DisableAccessCommand(
        access_control_id="ac-001",
        command_correlation_id=2,
        request_time=NOW,
    )
    assert c.kind == "disable"


def test_discriminator_routes_enable():
    obj = _TA.validate_python({
        "kind": "enable",
        "accessControlId": "ac-001",
        "commandCorrelationId": 1,
        "requestTime": NOW,
    })
    assert isinstance(obj, EnableAccessCommand)


def test_discriminator_routes_disable():
    obj = _TA.validate_python({
        "kind": "disable",
        "accessControlId": "ac-001",
        "commandCorrelationId": 2,
        "requestTime": NOW,
    })
    assert isinstance(obj, DisableAccessCommand)


def test_discriminator_rejects_unknown_kind():
    with pytest.raises(ValidationError):
        _TA.validate_python({
            "kind": "toggle",
            "accessControlId": "ac-001",
            "commandCorrelationId": 1,
            "requestTime": NOW,
        })


def test_enable_access_dumps_camel_case():
    c = EnableAccessCommand(
        access_control_id="ac-001",
        command_correlation_id=1,
        request_time=NOW,
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["accessControlId"] == "ac-001"
    assert dumped["commandCorrelationId"] == 1
    assert dumped["kind"] == "enable"


def test_command_rejects_negative_command_correlation_id():
    with pytest.raises(ValidationError):
        EnableAccessCommand(
            access_control_id="ac-001",
            command_correlation_id=-1,
            request_time=NOW,
        )


def test_command_response_minimal():
    r = AccessControlCommandResponse(
        access_control_id="ac-001",
        command_correlation_id=1,
        status=CommandResponseStatus.SUCCESS,
        response_time=NOW,
    )
    assert r.status == CommandResponseStatus.SUCCESS


def test_command_response_with_details():
    r = AccessControlCommandResponse(
        access_control_id="ac-001",
        command_correlation_id=1,
        status=CommandResponseStatus.FAILED,
        response_time=NOW,
        code="MECH_FAULT",
        message="Bollard mechanism jammed",
    )
    assert r.code == "MECH_FAULT"
    assert r.message == "Bollard mechanism jammed"


def test_command_response_dumps_camel_case():
    r = AccessControlCommandResponse(
        access_control_id="ac-001",
        command_correlation_id=1,
        status=CommandResponseStatus.SUCCESS,
        response_time=NOW,
    )
    dumped = r.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["accessControlId"] == "ac-001"
    assert dumped["status"] == "success"
    assert dumped["responseTime"] == "2026-04-16T10:00:00Z"
