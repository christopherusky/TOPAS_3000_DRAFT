# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Strategy trigger commands and responses: enable and disable,
#              command correlation, optional audit-only requester,
#              error-code and outcome consistency,
#              rejection of an empty id or requester, success responses and
#              the access-denied, not-found and not-authenticated failure
#              codes, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.strategy.command and response."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.common.command import CommandResponseStatus
from topas_3000_model.strategy.command import StrategyTrigger
from topas_3000_model.strategy.enums import TriggerStateEnum, TriggerUpdateErrorEnum
from topas_3000_model.strategy.response import StrategyCommandResponse


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_strategy_trigger_enable():
    t = StrategyTrigger(
        strategy_id="STR00000001",
        command_correlation_id=1,
        request_time=NOW,
        service_requester="kent-utmc",
        trigger_state=TriggerStateEnum.ENABLED,
    )
    assert t.trigger_state == TriggerStateEnum.ENABLED
    assert t.service_requester == "kent-utmc"


def test_strategy_trigger_disable():
    t = StrategyTrigger(
        strategy_id="STR00000001",
        command_correlation_id=1,
        request_time=NOW,
        service_requester="kent-utmc",
        trigger_state=TriggerStateEnum.DISABLED,
    )
    assert t.trigger_state == TriggerStateEnum.DISABLED


def test_strategy_trigger_rejects_empty_id():
    with pytest.raises(ValidationError):
        StrategyTrigger(
            strategy_id="",
            command_correlation_id=1,
            request_time=NOW,
            service_requester="kent-utmc",
            trigger_state=TriggerStateEnum.ENABLED,
        )


def test_strategy_trigger_rejects_empty_requester():
    with pytest.raises(ValidationError):
        StrategyTrigger(
            strategy_id="STR001",
            command_correlation_id=1,
            request_time=NOW,
            service_requester="",
            trigger_state=TriggerStateEnum.ENABLED,
        )


def test_strategy_trigger_dumps_camel_case():
    t = StrategyTrigger(
        strategy_id="STR00000001",
        command_correlation_id=1,
        request_time=NOW,
        service_requester="kent-utmc",
        trigger_state=TriggerStateEnum.ENABLED,
    )
    dumped = t.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["strategyId"] == "STR00000001"
    assert dumped["serviceRequester"] == "kent-utmc"
    assert dumped["triggerState"] == "enabled"


def test_strategy_response_success():
    r = StrategyCommandResponse(
        strategy_id="STR00000001",
        command_correlation_id=1,
        service_requester="kent-utmc",
        status=CommandResponseStatus.SUCCESS,
        response_time=NOW,
    )
    assert r.status == CommandResponseStatus.SUCCESS
    assert r.error_code is None
    assert r.message is None


def test_strategy_response_failed_access_denied():
    r = StrategyCommandResponse(
        strategy_id="STR00000001",
        command_correlation_id=1,
        service_requester="kent-utmc",
        status=CommandResponseStatus.FAILED,
        response_time=NOW,
        error_code=TriggerUpdateErrorEnum.ACCESS_DENIED,
        message="Your account does not have permission to access this strategy",
    )
    assert r.status == CommandResponseStatus.FAILED
    assert r.error_code == TriggerUpdateErrorEnum.ACCESS_DENIED
    assert r.message.startswith("Your account")


def test_strategy_response_failed_not_found():
    r = StrategyCommandResponse(
        strategy_id="STR99999999",
        command_correlation_id=1,
        service_requester="kent-utmc",
        status=CommandResponseStatus.FAILED,
        response_time=NOW,
        error_code=TriggerUpdateErrorEnum.STRATEGY_ID_DOES_NOT_EXIST,
        message="Strategy STR99999999 not found",
    )
    assert r.error_code == TriggerUpdateErrorEnum.STRATEGY_ID_DOES_NOT_EXIST


def test_strategy_response_failed_not_authenticated():
    r = StrategyCommandResponse(
        strategy_id="STR00000001",
        command_correlation_id=1,
        service_requester="unknown-system",
        status=CommandResponseStatus.FAILED,
        response_time=NOW,
        error_code=TriggerUpdateErrorEnum.NOT_AUTHENTICATED,
    )
    assert r.error_code == TriggerUpdateErrorEnum.NOT_AUTHENTICATED


def test_strategy_response_dumps_camel_case():
    r = StrategyCommandResponse(
        strategy_id="STR00000001",
        command_correlation_id=1,
        service_requester="kent-utmc",
        status=CommandResponseStatus.SUCCESS,
        response_time=NOW,
    )
    dumped = r.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["strategyId"] == "STR00000001"
    assert dumped["serviceRequester"] == "kent-utmc"
    assert dumped["status"] == "success"
    assert dumped["responseTime"] == "2026-04-16T10:00:00Z"


def test_strategy_response_failed_dumps_error_code():
    r = StrategyCommandResponse(
        strategy_id="STR00000001",
        command_correlation_id=1,
        service_requester="kent-utmc",
        status=CommandResponseStatus.FAILED,
        response_time=NOW,
        error_code=TriggerUpdateErrorEnum.ACCESS_DENIED,
        message="Not permitted",
    )
    dumped = r.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["status"] == "failed"
    assert dumped["errorCode"] == "accessDenied"
    assert dumped["message"] == "Not permitted"


def test_strategy_trigger_requires_correlation_and_request_time():
    with pytest.raises(ValidationError):
        StrategyTrigger(strategy_id="STR1", trigger_state=TriggerStateEnum.ENABLED)
    with pytest.raises(ValidationError):
        StrategyTrigger(
            strategy_id="STR1",
            command_correlation_id=1,
            trigger_state=TriggerStateEnum.ENABLED,
        )


def test_strategy_trigger_rejects_negative_correlation_id():
    with pytest.raises(ValidationError):
        StrategyTrigger(
            strategy_id="STR1",
            command_correlation_id=-1,
            request_time=NOW,
            trigger_state=TriggerStateEnum.ENABLED,
        )


def test_strategy_trigger_service_requester_is_optional_audit_label():
    t = StrategyTrigger(
        strategy_id="STR1",
        command_correlation_id=7,
        request_time=NOW,
        trigger_state=TriggerStateEnum.DISABLED,
    )
    assert t.service_requester is None
    dumped = t.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["commandCorrelationId"] == 7
    assert "serviceRequester" not in dumped


def test_strategy_response_requires_correlation_id():
    with pytest.raises(ValidationError):
        StrategyCommandResponse(
            strategy_id="STR1",
            status=CommandResponseStatus.SUCCESS,
            response_time=NOW,
        )


def test_strategy_response_dumps_correlation_and_code():
    r = StrategyCommandResponse(
        strategy_id="STR1",
        command_correlation_id=7,
        status=CommandResponseStatus.FAILED,
        response_time=NOW,
        code="E42",
        error_code=TriggerUpdateErrorEnum.OTHER,
    )
    dumped = r.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["commandCorrelationId"] == 7
    assert dumped["code"] == "E42"
    assert "serviceRequester" not in dumped


@pytest.mark.parametrize(
    "status", [CommandResponseStatus.SUCCESS, CommandResponseStatus.PENDING]
)
def test_strategy_response_rejects_error_code_on_non_failure(status):
    with pytest.raises(ValidationError):
        StrategyCommandResponse(
            strategy_id="STR1",
            command_correlation_id=1,
            status=status,
            response_time=NOW,
            error_code=TriggerUpdateErrorEnum.ACCESS_DENIED,
        )


@pytest.mark.parametrize(
    "status",
    [
        CommandResponseStatus.FAILED,
        CommandResponseStatus.INVALID,
        CommandResponseStatus.TIMED_OUT,
        CommandResponseStatus.NOT_SUPPORTED,
    ],
)
def test_strategy_response_allows_error_code_on_failure_outcomes(status):
    r = StrategyCommandResponse(
        strategy_id="STR1",
        command_correlation_id=1,
        status=status,
        response_time=NOW,
        error_code=TriggerUpdateErrorEnum.ACCESS_DENIED,
    )
    assert r.error_code == TriggerUpdateErrorEnum.ACCESS_DENIED
