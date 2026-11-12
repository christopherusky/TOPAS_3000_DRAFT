# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-03
# Version:     1.0
# Description: Command contract conformance: every command request carries
#              commandCorrelationId and requestTime, every command response
#              copies commandCorrelationId, and every correlation id is
#              bounded to the JSON safe-integer range. Sync requests correlate by
#              syncCorrelationId and are outside this contract.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Every command-bearing domain honours the same request/reply contract."""

from typing import Annotated, Union, get_args, get_origin

import pytest

from datetime import datetime, timezone

from bin.emit import FILE_SPECS
from pydantic import BaseModel, ValidationError

REQUEST_FIELDS = {"command_correlation_id", "request_time"}
RESPONSE_FIELDS = {"command_correlation_id"}

_BY_NAME = {s.filename: s for s in FILE_SPECS}


def _variants(tp) -> list[type[BaseModel]]:
    """Flatten an Annotated/Union alias into its concrete model classes."""
    if get_origin(tp) is Annotated:
        return _variants(get_args(tp)[0])
    if get_origin(tp) is Union:
        return [m for arg in get_args(tp) for m in _variants(arg)]
    return [tp]


_PAIRS = [
    (s, _BY_NAME[s.reply_filename])
    for s in FILE_SPECS
    if s.reply_filename is not None and "_sync_request-" not in s.filename
]


def test_every_command_domain_is_covered():
    assert len(_PAIRS) >= 6


@pytest.mark.parametrize("request_spec,reply_spec", _PAIRS, ids=lambda s: getattr(s, "filename", ""))
def test_command_pair_carries_correlation_fields(request_spec, reply_spec):
    for model in _variants(request_spec.canonical):
        missing = REQUEST_FIELDS - set(model.model_fields)
        assert not missing, f"{model.__name__} lacks {sorted(missing)}"
    for model in _variants(reply_spec.canonical):
        missing = RESPONSE_FIELDS - set(model.model_fields)
        assert not missing, f"{model.__name__} lacks {sorted(missing)}"


def _correlation_fields():
    seen = set()
    for spec in FILE_SPECS:
        if spec.canonical is None:
            continue
        for model in _variants(spec.canonical):
            for name, field in model.model_fields.items():
                if name.endswith("correlation_id"):
                    seen.add((model.__name__, name, field))
    return sorted(seen, key=lambda t: (t[0], t[1]))


@pytest.mark.parametrize(
    "model_name,field_name,field", _correlation_fields(), ids=lambda v: v if isinstance(v, str) else ""
)
def test_correlation_ids_are_bounded_to_json_safe_integers(model_name, field_name, field):
    """TypeScript carries a JSON number as a double, exact only to 2**53 - 1."""
    meta = {type(m).__name__: m for m in field.metadata}
    assert meta["Ge"].ge == 0
    assert meta["Le"].le == 2**53 - 1


# ── the bound, exercised with real values ──

_NOW = datetime(2026, 10, 3, 12, 0, tzinfo=timezone.utc)
_MAX_ID = 2**53 - 1


def _build_cases():
    from topas_3000_model.common.sync import SyncComplete, SyncRequest
    from topas_3000_model.detector.command import ResetDetectorCommand
    from topas_3000_model.detector.response import DetectorCommandResponse
    from topas_3000_model.strategy.command import StrategyTrigger
    from topas_3000_model.strategy.enums import TriggerStateEnum
    from topas_3000_model.strategy.response import StrategyCommandResponse
    from topas_3000_model.common.command import CommandResponseStatus

    ok = CommandResponseStatus.SUCCESS
    return {
        "detector command": lambda i: ResetDetectorCommand(detector_id="d", command_correlation_id=i, request_time=_NOW),
        "detector response": lambda i: DetectorCommandResponse(detector_id="d", command_correlation_id=i, status=ok, response_time=_NOW),
        "strategy command": lambda i: StrategyTrigger(strategy_id="s", command_correlation_id=i, request_time=_NOW, trigger_state=TriggerStateEnum.ENABLED),
        "strategy response": lambda i: StrategyCommandResponse(strategy_id="s", command_correlation_id=i, status=ok, response_time=_NOW),
        "sync request": lambda i: SyncRequest(sync_correlation_id=i, request_time=_NOW),
        "sync complete": lambda i: SyncComplete(sync_correlation_id=i, status="success", object_count=0, completed_time=_NOW),
    }


@pytest.mark.parametrize("name", list(_build_cases()))
def test_the_largest_json_safe_id_is_accepted_and_the_next_is_not(name):
    build = _build_cases()[name]
    build(_MAX_ID)
    with pytest.raises(ValidationError):
        build(_MAX_ID + 1)
    with pytest.raises(ValidationError):
        build(-1)
