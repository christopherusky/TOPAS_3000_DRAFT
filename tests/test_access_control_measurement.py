# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Access-control dynamic measurements: flow and disabled states,
#              rejection of negative flow and zero interval, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.access_control.measurement."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.access_control.measurement import (
    AccessControlDynamic,
    AccessControlState,
)


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_access_control_dynamic_minimal():
    d = AccessControlDynamic(
        access_control_id="ac-001",
        timestamp=NOW,
        active=True,
        state_type=AccessControlState.ENABLED,
    )
    assert d.active is True
    assert d.state_type == AccessControlState.ENABLED
    assert d.flow is None
    assert d.flow_interval_seconds is None


def test_access_control_dynamic_with_flow():
    d = AccessControlDynamic(
        access_control_id="ac-001",
        timestamp=NOW,
        active=True,
        state_type=AccessControlState.ENABLED,
        flow=42,
        flow_interval_seconds=300,
    )
    assert d.flow == 42
    assert d.flow_interval_seconds == 300


def test_access_control_dynamic_disabled():
    d = AccessControlDynamic(
        access_control_id="ac-001",
        timestamp=NOW,
        active=False,
        state_type=AccessControlState.DISABLED,
    )
    assert d.active is False
    assert d.state_type == AccessControlState.DISABLED


def test_access_control_dynamic_rejects_negative_flow():
    with pytest.raises(ValidationError):
        AccessControlDynamic(
            access_control_id="ac-001",
            timestamp=NOW,
            active=True,
            state_type=AccessControlState.ENABLED,
            flow=-1,
        )


def test_access_control_dynamic_rejects_zero_interval():
    with pytest.raises(ValidationError):
        AccessControlDynamic(
            access_control_id="ac-001",
            timestamp=NOW,
            active=True,
            state_type=AccessControlState.ENABLED,
            flow_interval_seconds=0,
        )


def test_access_control_dynamic_dumps_camel_case():
    d = AccessControlDynamic(
        access_control_id="ac-001",
        timestamp=NOW,
        active=True,
        state_type=AccessControlState.ENABLED,
        flow=42,
        flow_interval_seconds=300,
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["accessControlId"] == "ac-001"
    assert dumped["active"] is True
    assert dumped["stateType"] == "enabled"
    assert dumped["flow"] == 42
    assert dumped["flowIntervalSeconds"] == 300
