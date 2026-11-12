# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: The CCTV measurement discriminated union: image, ANPR, ANPR-
#              VRN, broadcast, flow and queue variants, discriminator
#              dispatch, unknown-kind rejection, rejection of negative vehicle
#              counts, zero period and confidence over 100, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.cctv.measurement."""

from datetime import datetime, timezone

import pytest
from pydantic import TypeAdapter, ValidationError

from topas_3000_model.cctv.enums import ImageType, VehicleDirection
from topas_3000_model.cctv.measurement import (
    CctvAnprDynamic,
    CctvAnprVrnDynamic,
    CctvBroadcastDynamic,
    CctvFlowDynamic,
    CctvImageDynamic,
    CctvMeasurement,
    CctvQueueDynamic,
)


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)
_TA = TypeAdapter(CctvMeasurement)


def test_cctv_image_dynamic():
    d = CctvImageDynamic(
        cctv_id="cam-001", timestamp=NOW,
        image_url="https://images.example.com/cam-001/latest.jpg",
        image_type=ImageType.JPEG,
        preset=3,
    )
    assert d.kind == "image"
    assert d.image_type == ImageType.JPEG


def test_cctv_anpr_dynamic():
    d = CctvAnprDynamic(
        cctv_id="cam-001", timestamp=NOW,
        count_period_seconds=300, vehicle_count=42,
    )
    assert d.kind == "anpr"
    assert d.vehicle_count == 42


def test_cctv_anpr_vrn_dynamic():
    d = CctvAnprVrnDynamic(
        cctv_id="cam-001", timestamp=NOW,
        vrn="AB12 CDE", vrn_confidence=95.5,
        vehicle_direction=VehicleDirection.TOWARDS,
        speed_kph=30.0,
        vehicle_classification="Euro6",
        country="GB",
    )
    assert d.kind == "anprVrn"
    assert d.vrn == "AB12 CDE"
    assert d.vehicle_direction == VehicleDirection.TOWARDS


def test_cctv_broadcast_dynamic():
    d = CctvBroadcastDynamic(
        cctv_id="cam-001", timestamp=NOW, broadcast=True,
    )
    assert d.kind == "broadcast"
    assert d.broadcast is True


def test_cctv_flow_dynamic():
    d = CctvFlowDynamic(
        cctv_id="cam-001", timestamp=NOW,
        count_period_seconds=300, vehicle_count=120,
    )
    assert d.kind == "flow"


def test_cctv_queue_dynamic():
    d = CctvQueueDynamic(
        cctv_id="cam-001", timestamp=NOW,
        vehicle_count=8, estimated_speed_kph=5.0,
    )
    assert d.kind == "queue"


def test_discriminator_routes_image():
    obj = _TA.validate_python({
        "kind": "image", "cctvId": "cam-001", "timestamp": NOW,
        "imageUrl": "https://example.com/img.jpg",
    })
    assert isinstance(obj, CctvImageDynamic)


def test_discriminator_routes_anpr_vrn():
    obj = _TA.validate_python({
        "kind": "anprVrn", "cctvId": "cam-001", "timestamp": NOW,
    })
    assert isinstance(obj, CctvAnprVrnDynamic)


def test_discriminator_routes_broadcast():
    obj = _TA.validate_python({
        "kind": "broadcast", "cctvId": "cam-001", "timestamp": NOW,
        "broadcast": False,
    })
    assert isinstance(obj, CctvBroadcastDynamic)


def test_discriminator_rejects_unknown_kind():
    with pytest.raises(ValidationError):
        _TA.validate_python({
            "kind": "thermal", "cctvId": "cam-001", "timestamp": NOW,
        })


def test_anpr_rejects_negative_vehicle_count():
    with pytest.raises(ValidationError):
        CctvAnprDynamic(
            cctv_id="cam-001", timestamp=NOW,
            count_period_seconds=300, vehicle_count=-1,
        )


def test_anpr_rejects_zero_period():
    with pytest.raises(ValidationError):
        CctvAnprDynamic(
            cctv_id="cam-001", timestamp=NOW,
            count_period_seconds=0, vehicle_count=10,
        )


def test_vrn_confidence_rejects_over_100():
    with pytest.raises(ValidationError):
        CctvAnprVrnDynamic(
            cctv_id="cam-001", timestamp=NOW, vrn_confidence=101.0,
        )


def test_cctv_image_dumps_camel_case():
    d = CctvImageDynamic(
        cctv_id="cam-001", timestamp=NOW,
        image_url="https://example.com/img.jpg",
        image_type=ImageType.PNG,
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["cctvId"] == "cam-001"
    assert dumped["kind"] == "image"
    assert dumped["imageType"] == "png"


def test_cctv_anpr_vrn_dumps_camel_case():
    d = CctvAnprVrnDynamic(
        cctv_id="cam-001", timestamp=NOW,
        vrn="AB12 CDE", vehicle_direction=VehicleDirection.AWAY,
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["vrn"] == "AB12 CDE"
    assert dumped["vehicleDirection"] == "away"


def test_cctv_anpr_vrn_full_vehicle_metadata():
    d = CctvAnprVrnDynamic(
        cctv_id="cam-001", timestamp=NOW,
        vrn="AB12 CDE", vrn_confidence=95.5,
        tag=123456, tag_confidence=90.0,
        country="GB",
        vehicle_direction=VehicleDirection.TOWARDS,
        speed_kph=48.5,
        vehicle_classification="Euro6",
        vehicle_make="Ford",
        vehicle_model="Focus",
        vehicle_colour="Blue",
        serial_number="SN-001",
        lane_id=2,
        lane_direction_type_id="1",
    )
    assert d.vrn == "AB12 CDE"
    assert d.tag == 123456
    assert d.vehicle_make == "Ford"
    assert d.vehicle_model == "Focus"
    assert d.vehicle_colour == "Blue"
    assert d.vehicle_classification == "Euro6"
    assert d.country == "GB"
