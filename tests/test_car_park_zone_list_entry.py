# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: CarParkZoneListEntry, a car park's membership of a network zone
#: minimal construction, rejection of missing car-park and
#              zone ids, participant id, and camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for CarParkZoneListEntry — car park's membership in a network zone."""

from __future__ import annotations

import pytest
from pydantic import ValidationError

from topas_3000_model.car_park.definition import CarParkZoneListEntry


def test_zone_list_entry_minimal():
    z = CarParkZoneListEntry(car_park_id="cp-001", network_zone_id="zone-001")
    assert z.car_park_id == "cp-001"
    assert z.network_zone_id == "zone-001"
    assert z.participant_id is None


def test_zone_list_entry_rejects_missing_car_park_id():
    with pytest.raises(ValidationError):
        CarParkZoneListEntry(network_zone_id="zone-001")


def test_zone_list_entry_rejects_missing_network_zone_id():
    with pytest.raises(ValidationError):
        CarParkZoneListEntry(car_park_id="cp-001")


def test_zone_list_entry_with_participant_id():
    z = CarParkZoneListEntry(
        car_park_id="cp-001",
        network_zone_id="zone-001",
        participant_id="operator-42",
    )
    assert z.participant_id == "operator-42"


def test_zone_list_entry_dumps_camel_case():
    z = CarParkZoneListEntry(
        car_park_id="cp-001",
        network_zone_id="zone-001",
        participant_id="operator-42",
    )
    dumped = z.model_dump(by_alias=True)
    assert dumped["carParkId"] == "cp-001"
    assert dumped["networkZoneId"] == "zone-001"
    assert dumped["participantId"] == "operator-42"
