# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: Transport-route dynamic measurements: journey time and speed
#              with status, rejection of out-of-range and negative values, and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.transport_route.measurement."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.transport_route.enums import TransportRouteStatus
from topas_3000_model.transport_route.measurement import TransportRouteDynamic


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_transport_route_dynamic_minimal():
    d = TransportRouteDynamic(transport_route_id="tr-001", timestamp=NOW)
    assert d.current_journey_time_seconds is None
    assert d.estimated_speed_kph is None
    assert d.route_status is None


def test_transport_route_dynamic_with_status():
    d = TransportRouteDynamic(
        transport_route_id="tr-001", timestamp=NOW,
        current_journey_time_seconds=420.0,
        estimated_speed_kph=35.0,
        route_status=TransportRouteStatus.IN_RANGE,
    )
    assert d.route_status == TransportRouteStatus.IN_RANGE
    assert d.current_journey_time_seconds == 420.0
    assert d.estimated_speed_kph == 35.0


def test_transport_route_dynamic_out_of_range():
    d = TransportRouteDynamic(
        transport_route_id="tr-001", timestamp=NOW,
        route_status=TransportRouteStatus.OUT_OF_RANGE,
    )
    assert d.route_status == TransportRouteStatus.OUT_OF_RANGE


def test_transport_route_dynamic_rejects_negative_journey_time():
    with pytest.raises(ValidationError):
        TransportRouteDynamic(
            transport_route_id="tr-001", timestamp=NOW,
            current_journey_time_seconds=-1.0,
        )


def test_transport_route_dynamic_rejects_negative_speed():
    with pytest.raises(ValidationError):
        TransportRouteDynamic(
            transport_route_id="tr-001", timestamp=NOW,
            estimated_speed_kph=-5.0,
        )


def test_transport_route_dynamic_status_dumps_camel_case():
    d = TransportRouteDynamic(
        transport_route_id="tr-001", timestamp=NOW,
        route_status=TransportRouteStatus.OUT_OF_RANGE,
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["routeStatus"] == "outOfRange"
    assert dumped["transportRouteId"] == "tr-001"


def test_transport_route_dynamic_dumps_camel_case():
    d = TransportRouteDynamic(
        transport_route_id="tr-001", timestamp=NOW,
        current_journey_time_seconds=300.0,
        estimated_speed_kph=50.0,
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["currentJourneyTimeSeconds"] == 300.0
    assert dumped["estimatedSpeedKph"] == 50.0
