# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: The transport-link configuration discriminated union (SCOOT,
#              GPS and ANPR): each variant minimal and fully populated,
#              rejection of a missing or unknown kind, and camelCase
#              serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for the Transport Link configuration discriminated union (SCOOT/GPS/ANPR).

Configuration variants carry identical field content; only the `kind`
literal distinguishes which monitoring system configured the link.
Corresponds to TransportLinkScootConfiguration /
TransportLinkGpsConfiguration / TransportLinkAnprConfiguration in
utmc_data_object_transport_link-v1.0.xsd.
"""

from __future__ import annotations

import pytest
from pydantic import TypeAdapter, ValidationError

from topas_3000_model.transport_link.definition import (
    TransportLinkANPRConfiguration,
    TransportLinkConfiguration,
    TransportLinkGPSConfiguration,
    TransportLinkSCOOTConfiguration,
)


_ADAPTER = TypeAdapter(TransportLinkConfiguration)


def _base_payload(kind: str) -> dict:
    return {
        "kind": kind,
        "transportLinkId": "TL-001",
        "configurationDate": "2026-04-17T10:00:00Z",
    }


# ── SCOOT variant ──────────────────────────────────────────────────────

def test_scoot_configuration_minimal():
    c = _ADAPTER.validate_python(_base_payload("scoot"))
    assert isinstance(c, TransportLinkSCOOTConfiguration)
    assert c.kind == "scoot"


def test_scoot_configuration_with_all_fields():
    payload = {
        **_base_payload("scoot"),
        "lengthMetres": 120.0,
        "fullLengthMetres": 180.0,
        "numberOfLanes": 2,
        "maximumSpeedKph": 50.0,
        "capacityVehiclesPerHour": 1800.0,
    }
    c = _ADAPTER.validate_python(payload)
    assert isinstance(c, TransportLinkSCOOTConfiguration)
    assert c.number_of_lanes == 2


def test_scoot_configuration_dumps_camel_case():
    c = TransportLinkSCOOTConfiguration(
        transport_link_id="TL-001",
        configuration_date="2026-04-17T10:00:00Z",
        length_metres=120.0,
    )
    dumped = c.model_dump(by_alias=True)
    assert dumped["kind"] == "scoot"
    assert dumped["transportLinkId"] == "TL-001"
    assert dumped["lengthMetres"] == 120.0


# ── GPS variant ────────────────────────────────────────────────────────

def test_gps_configuration_minimal():
    c = _ADAPTER.validate_python(_base_payload("gps"))
    assert isinstance(c, TransportLinkGPSConfiguration)
    assert c.kind == "gps"


def test_gps_configuration_with_all_fields():
    payload = {
        **_base_payload("gps"),
        "lengthMetres": 120.0,
        "fullLengthMetres": 180.0,
        "numberOfLanes": 2,
        "maximumSpeedKph": 50.0,
        "capacityVehiclesPerHour": 1800.0,
    }
    c = _ADAPTER.validate_python(payload)
    assert isinstance(c, TransportLinkGPSConfiguration)
    assert c.maximum_speed_kph == 50.0


def test_gps_configuration_dumps_camel_case():
    c = TransportLinkGPSConfiguration(
        transport_link_id="TL-001",
        configuration_date="2026-04-17T10:00:00Z",
    )
    dumped = c.model_dump(by_alias=True)
    assert dumped["kind"] == "gps"
    assert dumped["transportLinkId"] == "TL-001"


# ── ANPR variant ───────────────────────────────────────────────────────

def test_anpr_configuration_minimal():
    c = _ADAPTER.validate_python(_base_payload("anpr"))
    assert isinstance(c, TransportLinkANPRConfiguration)
    assert c.kind == "anpr"


def test_anpr_configuration_with_all_fields():
    payload = {
        **_base_payload("anpr"),
        "lengthMetres": 120.0,
        "fullLengthMetres": 180.0,
        "numberOfLanes": 2,
        "maximumSpeedKph": 50.0,
        "capacityVehiclesPerHour": 1800.0,
    }
    c = _ADAPTER.validate_python(payload)
    assert isinstance(c, TransportLinkANPRConfiguration)
    assert c.capacity_vehicles_per_hour == 1800.0


def test_anpr_configuration_dumps_camel_case():
    c = TransportLinkANPRConfiguration(
        transport_link_id="TL-001",
        configuration_date="2026-04-17T10:00:00Z",
    )
    dumped = c.model_dump(by_alias=True)
    assert dumped["kind"] == "anpr"


# ── Discriminator & validation ─────────────────────────────────────────

def test_configuration_rejects_missing_kind():
    payload = {
        "transportLinkId": "TL-001",
        "configurationDate": "2026-04-17T10:00:00Z",
    }
    with pytest.raises(ValidationError):
        _ADAPTER.validate_python(payload)


def test_configuration_rejects_unknown_kind():
    payload = {**_base_payload("lidar")}
    with pytest.raises(ValidationError):
        _ADAPTER.validate_python(payload)


def test_configuration_rejects_negative_length():
    with pytest.raises(ValidationError):
        TransportLinkSCOOTConfiguration(
            transport_link_id="TL-001",
            configuration_date="2026-04-17T10:00:00Z",
            length_metres=-1.0,
        )


def test_configuration_rejects_negative_full_length():
    with pytest.raises(ValidationError):
        TransportLinkGPSConfiguration(
            transport_link_id="TL-001",
            configuration_date="2026-04-17T10:00:00Z",
            full_length_metres=-1.0,
        )


def test_configuration_rejects_zero_or_negative_lanes():
    with pytest.raises(ValidationError):
        TransportLinkANPRConfiguration(
            transport_link_id="TL-001",
            configuration_date="2026-04-17T10:00:00Z",
            number_of_lanes=0,
        )


def test_configuration_rejects_negative_maximum_speed():
    with pytest.raises(ValidationError):
        TransportLinkSCOOTConfiguration(
            transport_link_id="TL-001",
            configuration_date="2026-04-17T10:00:00Z",
            maximum_speed_kph=-1.0,
        )


def test_configuration_rejects_negative_capacity():
    with pytest.raises(ValidationError):
        TransportLinkSCOOTConfiguration(
            transport_link_id="TL-001",
            configuration_date="2026-04-17T10:00:00Z",
            capacity_vehicles_per_hour=-1.0,
        )


def test_configuration_discriminator_round_trip():
    for kind, cls in [
        ("scoot", TransportLinkSCOOTConfiguration),
        ("gps", TransportLinkGPSConfiguration),
        ("anpr", TransportLinkANPRConfiguration),
    ]:
        payload = _base_payload(kind)
        obj = _ADAPTER.validate_python(payload)
        assert isinstance(obj, cls)
        assert obj.model_dump(by_alias=True)["kind"] == kind
