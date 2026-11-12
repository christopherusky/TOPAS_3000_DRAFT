# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.2
# Description: Transport-route fault payloads: minimal and full construction,
#              negative fault-id rejection, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.transport_route.fault."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.transport_route.fault import TransportRouteFault
from topas_3000_model.fault_support.enums import AcknowledgementState, FaultStatus


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_transport_route_fault_minimal():
    f = TransportRouteFault(
        transport_route_id="tr-001", fault_id=101, fault_type_id="5", creation_date=NOW,
    )
    assert f.fault_id == 101


def test_transport_route_fault_with_all_fields():
    f = TransportRouteFault(
        transport_route_id="tr-001", fault_id=101, fault_type_id="5", creation_date=NOW,
        description="Link sensor failure",
        acknowledgement_state=AcknowledgementState.ACKNOWLEDGED,
        acknowledgement_date=NOW,
        severity=2, status=FaultStatus.ACTIVE,
    )
    assert f.acknowledgement_state == AcknowledgementState.ACKNOWLEDGED


def test_transport_route_fault_rejects_negative_fault_id():
    with pytest.raises(ValidationError):
        TransportRouteFault(transport_route_id="tr-001", fault_id=-1, fault_type_id="5", creation_date=NOW)


def test_transport_route_fault_dumps_camel_case():
    f = TransportRouteFault(
        transport_route_id="tr-001", fault_id=101, fault_type_id="5", creation_date=NOW,
        status=FaultStatus.ACTIVE,
    )
    dumped = f.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["transportRouteId"] == "tr-001"
    assert dumped["status"] == "active"
