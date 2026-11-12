# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-29
# Version:     1.2
# Description: Access-control definition and configuration: minimal and fully-
#              populated construction, empty-id rejection, extensions, and
#              camelCase serialisation.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.access_control.definition."""

from datetime import datetime, timezone

import pytest
from pydantic import ValidationError

from topas_3000_model.access_control.definition import (
    AccessControlConfiguration,
    AccessControlDefinition,
)
from topas_3000_model.common.location import Coordinates, Point


NOW = datetime(2026, 4, 16, 10, 0, tzinfo=timezone.utc)


def test_access_control_definition_minimal():
    d = AccessControlDefinition(
        access_control_id="ac-001",
        type_id="1",
        creation_date=NOW,
    )
    assert d.access_control_id == "ac-001"
    assert d.type_id == "1"
    assert d.location is None
    assert d.owner_operator is None


def test_access_control_definition_with_all_fields():
    d = AccessControlDefinition(
        access_control_id="ac-001",
        type_id="1",
        creation_date=NOW,
        location=Point(coordinates=Coordinates(latitude=51.5, longitude=-0.1)),
        short_description="High Street Bollard",
        long_description="Retractable bollard at High Street pedestrian zone entrance",
        participant_id="sys-001",
        owner_operator="Metro Council",
        originating_participant_id="sys-002",
        originating_object_id="ext-ac-001",
        extensions={"vendor": "acme"},
    )
    assert d.location.coordinates.latitude == 51.5
    assert d.owner_operator == "Metro Council"
    assert d.extensions == {"vendor": "acme"}


def test_access_control_definition_rejects_empty_id():
    with pytest.raises(ValidationError):
        AccessControlDefinition(
            access_control_id="",
            type_id="1",
            creation_date=NOW,
        )


def test_access_control_definition_dumps_camel_case():
    d = AccessControlDefinition(
        access_control_id="ac-001",
        type_id="1",
        creation_date=NOW,
        owner_operator="Metro Council",
    )
    dumped = d.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["accessControlId"] == "ac-001"
    assert dumped["typeId"] == "1"
    assert dumped["creationDate"] == "2026-04-16T10:00:00Z"
    assert dumped["ownerOperator"] == "Metro Council"


def test_access_control_configuration_minimal():
    c = AccessControlConfiguration(
        access_control_id="ac-001",
        configuration_date=NOW,
    )
    assert c.access_control_id == "ac-001"
    assert c.extensions is None


def test_access_control_configuration_with_extensions():
    c = AccessControlConfiguration(
        access_control_id="ac-001",
        configuration_date=NOW,
        extensions={"schedule": "weekdays-only"},
    )
    assert c.extensions == {"schedule": "weekdays-only"}


def test_access_control_configuration_dumps_camel_case():
    c = AccessControlConfiguration(
        access_control_id="ac-001",
        configuration_date=NOW,
    )
    dumped = c.model_dump(by_alias=True, exclude_none=True, mode="json")
    assert dumped["accessControlId"] == "ac-001"
    assert dumped["configurationDate"] == "2026-04-16T10:00:00Z"
