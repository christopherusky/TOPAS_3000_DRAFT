# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.4
# Description: Transport-link commands: setStoc, recalibrate and
#              requestStatus, discriminator dispatch, rejection of unknown
#              kinds and negative correlation ids, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.transport_link.command."""

from datetime import datetime, timezone

import pytest
from pydantic import TypeAdapter, ValidationError

from topas_3000_model.transport_link.command import (
    RecalibrateTransportLinkCommand,
    RequestTransportLinkStatusCommand,
    SetStocCommand,
    TransportLinkCommand,
)


NOW = datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc)
_TA = TypeAdapter(TransportLinkCommand)


def test_set_stoc_minimal():
    c = SetStocCommand(
        transport_link_id="tl-001",
        command_correlation_id=1,
        request_time=NOW,
        stoc_value="42",
    )
    assert c.kind == "setStoc"
    assert c.stoc_value == "42"


def test_recalibrate_minimal():
    c = RecalibrateTransportLinkCommand(
        transport_link_id="tl-001",
        command_correlation_id=2,
        request_time=NOW,
    )
    assert c.kind == "recalibrate"


def test_request_status_minimal():
    c = RequestTransportLinkStatusCommand(
        transport_link_id="tl-001",
        command_correlation_id=3,
        request_time=NOW,
    )
    assert c.kind == "requestStatus"


def test_discriminator_routes_set_stoc():
    obj = _TA.validate_python({
        "kind": "setStoc",
        "transport_link_id": "tl-001",
        "command_correlation_id": 1,
        "request_time": NOW,
        "stoc_value": "42",
    })
    assert isinstance(obj, SetStocCommand)
    assert obj.stoc_value == "42"


def test_discriminator_routes_recalibrate():
    obj = _TA.validate_python({
        "kind": "recalibrate",
        "transport_link_id": "tl-001",
        "command_correlation_id": 2,
        "request_time": NOW,
    })
    assert isinstance(obj, RecalibrateTransportLinkCommand)


def test_discriminator_routes_request_status():
    obj = _TA.validate_python({
        "kind": "requestStatus",
        "transport_link_id": "tl-001",
        "command_correlation_id": 3,
        "request_time": NOW,
    })
    assert isinstance(obj, RequestTransportLinkStatusCommand)


def test_discriminator_rejects_unknown_kind():
    with pytest.raises(ValidationError):
        _TA.validate_python({
            "kind": "teleport",
            "transport_link_id": "tl-001",
            "command_correlation_id": 1,
            "request_time": NOW,
        })


def test_set_stoc_dumps_camel_case():
    c = SetStocCommand(
        transport_link_id="tl-001",
        command_correlation_id=1,
        request_time=NOW,
        stoc_value="42",
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert "transportLinkId" in dumped
    assert "commandCorrelationId" in dumped
    assert "requestTime" in dumped
    assert "kind" in dumped
    assert "stocValue" in dumped
    assert dumped["kind"] == "setStoc"


def test_rejects_negative_command_correlation_id():
    with pytest.raises(ValidationError):
        SetStocCommand(
            transport_link_id="tl-001",
            command_correlation_id=-1,
            request_time=NOW,
            stoc_value="42",
        )
