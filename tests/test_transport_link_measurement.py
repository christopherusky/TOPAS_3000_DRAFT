# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-06
# Version:     1.4
# Description: The transport-link measurement discriminated union: SCOOT and
#              ANPR variants, occupancy and flow range enforcement (flows
#              are non-negative finite numbers),
#              discriminator dispatch, unknown-kind rejection, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.transport_link.measurement."""

from datetime import datetime, timezone

import pytest
from pydantic import TypeAdapter, ValidationError

from topas_3000_model.transport_link.measurement import (
    TransportLinkANPRDynamic,
    TransportLinkGPSDynamic,
    TransportLinkMeasurement,
    TransportLinkSCOOTDynamic,
)


NOW = datetime(2026, 4, 11, 10, 0, tzinfo=timezone.utc)


def test_tl_scoot_dynamic_minimal():
    s = TransportLinkSCOOTDynamic(
        transport_link_id="tl-001",
        timestamp=NOW,
    )
    assert s.kind == "scoot"
    assert s.transport_link_id == "tl-001"
    assert s.congestion_percent is None
    assert s.current_flow_vehicles is None
    assert s.occupancy_percent is None
    assert s.average_speed_kph is None
    assert s.link_travel_time_seconds is None


def test_tl_scoot_dynamic_with_values():
    s = TransportLinkSCOOTDynamic(
        transport_link_id="tl-001",
        timestamp=NOW,
        congestion_percent=42.5,
        current_flow_vehicles=120,
        occupancy_percent=35.0,
        average_speed_kph=55.0,
        link_travel_time_seconds=42.0,
    )
    assert s.congestion_percent == 42.5
    assert s.current_flow_vehicles == 120
    assert s.occupancy_percent == 35.0
    assert s.average_speed_kph == 55.0
    assert s.link_travel_time_seconds == 42.0


def test_tl_scoot_dynamic_rejects_over_100_occupancy():
    with pytest.raises(ValidationError):
        TransportLinkSCOOTDynamic(
            transport_link_id="tl-001",
            timestamp=NOW,
            occupancy_percent=150.0,
        )


def test_tl_scoot_dynamic_rejects_negative_flow():
    with pytest.raises(ValidationError):
        TransportLinkSCOOTDynamic(
            transport_link_id="tl-001",
            timestamp=NOW,
            current_flow_vehicles=-1,
        )


@pytest.mark.parametrize(
    ("model", "field", "extra"),
    [
        (TransportLinkSCOOTDynamic, "current_flow_vehicles", {}),
        (TransportLinkGPSDynamic, "reserved_lane_flow_vehicles", {}),
    ],
    ids=["scoot-current-flow", "gps-reserved-lane-flow"],
)
class TestFlowIsANonNegativeNumber:
    """v1's VehiclesPerInterval is xsd:decimal: a rate per interval, so it may
    be fractional. The models had narrowed it to an integer."""

    def test_accepts_a_fractional_flow(self, model, field, extra):
        m = model(transport_link_id="tl-001", timestamp=NOW, **{field: 12.5}, **extra)
        assert getattr(m, field) == 12.5

    def test_still_accepts_an_integer(self, model, field, extra):
        m = model(transport_link_id="tl-001", timestamp=NOW, **{field: 420}, **extra)
        assert getattr(m, field) == 420

    @pytest.mark.parametrize("value", [-0.5, float("inf"), float("nan")])
    def test_rejects_negative_and_non_finite(self, model, field, extra, value):
        with pytest.raises(ValidationError):
            model(transport_link_id="tl-001", timestamp=NOW, **{field: value}, **extra)


def test_tl_anpr_dynamic_minimal():
    a = TransportLinkANPRDynamic(
        transport_link_id="tl-001",
        timestamp=NOW,
    )
    assert a.kind == "anpr"
    assert a.number_plates_in is None
    assert a.number_plates_out is None
    assert a.number_plates_matches is None
    assert a.number_plates_matches_used is None


def test_tl_anpr_dynamic_with_values():
    a = TransportLinkANPRDynamic(
        transport_link_id="tl-001",
        timestamp=NOW,
        number_plates_in=100,
        number_plates_out=95,
        number_plates_matches=80,
        number_plates_matches_used=75,
        average_speed_kph=48.0,
        link_travel_time_seconds=50.0,
    )
    assert a.number_plates_in == 100
    assert a.number_plates_out == 95
    assert a.number_plates_matches == 80
    assert a.number_plates_matches_used == 75
    assert a.average_speed_kph == 48.0
    assert a.link_travel_time_seconds == 50.0


def test_tl_anpr_dynamic_rejects_negative():
    with pytest.raises(ValidationError):
        TransportLinkANPRDynamic(
            transport_link_id="tl-001",
            timestamp=NOW,
            number_plates_in=-1,
        )


def test_discriminated_union_routes_scoot():
    adapter = TypeAdapter(TransportLinkMeasurement)
    instance = adapter.validate_python({
        "kind": "scoot",
        "transportLinkId": "tl-001",
        "timestamp": "2026-04-11T10:00:00Z",
        "congestionPercent": 42.5,
    })
    assert isinstance(instance, TransportLinkSCOOTDynamic)
    assert instance.congestion_percent == 42.5


def test_discriminated_union_routes_anpr():
    adapter = TypeAdapter(TransportLinkMeasurement)
    instance = adapter.validate_python({
        "kind": "anpr",
        "transportLinkId": "tl-001",
        "timestamp": "2026-04-11T10:00:00Z",
        "numberPlatesIn": 100,
    })
    assert isinstance(instance, TransportLinkANPRDynamic)
    assert instance.number_plates_in == 100


def test_discriminated_union_rejects_unknown_kind():
    adapter = TypeAdapter(TransportLinkMeasurement)
    with pytest.raises(ValidationError):
        adapter.validate_python({
            "kind": "bogus",
            "transportLinkId": "tl-001",
            "timestamp": "2026-04-11T10:00:00Z",
        })


def test_tl_scoot_dynamic_dumps_camel_case():
    s = TransportLinkSCOOTDynamic(
        transport_link_id="tl-001",
        timestamp=NOW,
        congestion_percent=42.5,
        current_flow_vehicles=120,
    )
    dumped = s.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["kind"] == "scoot"
    assert dumped["transportLinkId"] == "tl-001"
    assert dumped["congestionPercent"] == 42.5
    assert dumped["currentFlowVehicles"] == 120


def test_tl_anpr_dynamic_dumps_camel_case():
    a = TransportLinkANPRDynamic(
        transport_link_id="tl-001",
        timestamp=NOW,
        number_plates_in=100,
        number_plates_out=95,
    )
    dumped = a.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["kind"] == "anpr"
    assert dumped["numberPlatesIn"] == 100
    assert dumped["numberPlatesOut"] == 95


def test_gps_measurement_minimal():
    payload = {
        "kind": "gps",
        "transportLinkId": "TL-001",
        "timestamp": "2026-04-17T10:00:00Z",
    }
    m = TypeAdapter(TransportLinkMeasurement).validate_python(payload)
    assert isinstance(m, TransportLinkGPSDynamic)
    assert m.kind == "gps"


def test_gps_measurement_with_all_fields():
    payload = {
        "kind": "gps",
        "transportLinkId": "TL-001",
        "timestamp": "2026-04-17T10:00:00Z",
        "averageSpeedKph": 48.0,
        "linkTravelTimeSeconds": 72.0,
        "currentJourneyTimeSeconds": 75.0,
        "meanTravelTimeSeconds": 70.0,
        "medianTravelTimeSeconds": 68.0,
        "varianceTravelTimeSeconds": 4.5,
        "numberOfVehicles": 142,
        "reservedLaneFlowVehicles": 12,
    }
    m = TypeAdapter(TransportLinkMeasurement).validate_python(payload)
    assert isinstance(m, TransportLinkGPSDynamic)
    assert m.median_travel_time_seconds == 68.0
    assert m.reserved_lane_flow_vehicles == 12


def test_gps_measurement_rejects_negative_current_journey_time():
    with pytest.raises(ValidationError):
        TransportLinkGPSDynamic(
            transport_link_id="TL-001",
            timestamp="2026-04-17T10:00:00Z",
            current_journey_time_seconds=-1.0,
        )


def test_gps_measurement_rejects_negative_mean_travel_time():
    with pytest.raises(ValidationError):
        TransportLinkGPSDynamic(
            transport_link_id="TL-001",
            timestamp="2026-04-17T10:00:00Z",
            mean_travel_time_seconds=-1.0,
        )


def test_gps_measurement_rejects_negative_number_of_vehicles():
    with pytest.raises(ValidationError):
        TransportLinkGPSDynamic(
            transport_link_id="TL-001",
            timestamp="2026-04-17T10:00:00Z",
            number_of_vehicles=-1,
        )


def test_gps_measurement_dumps_camel_case():
    m = TransportLinkGPSDynamic(
        transport_link_id="TL-001",
        timestamp="2026-04-17T10:00:00Z",
        current_journey_time_seconds=75.0,
        mean_travel_time_seconds=70.0,
        reserved_lane_flow_vehicles=12,
    )
    dumped = m.model_dump(by_alias=True)
    assert dumped["kind"] == "gps"
    assert dumped["transportLinkId"] == "TL-001"
    assert dumped["currentJourneyTimeSeconds"] == 75.0
    assert dumped["reservedLaneFlowVehicles"] == 12
