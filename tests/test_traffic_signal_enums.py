# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-26
# Version:     1.3
# Description: The TS004.0064bis:2017 traffic-signal enumerations: the fault,
#              type, mode, sub-mode, urgency and fault-severity value sets,
#              and that all are str subclasses.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for TS004.0064bis:2017 traffic-signal enumerations."""

from topas_3000_model.traffic_signal.enums import (
    FaultSeverityEnum,
    TrafficSignalFaultEnum,
    TrafficSignalModeEnum,
    TrafficSignalSubModeEnum,
    TrafficSignalTypeEnum,
    UrgencyEnum,
)


def test_fault_enum_values():
    assert {e.value for e in TrafficSignalFaultEnum} == {
        "detectorFault",
        "electronicsFault",
        "indicatorFault",
        "lampFault",
        "mechanicalFault",
        "utilityFault",
        "other",
    }


def test_type_enum_values():
    assert {e.value for e in TrafficSignalTypeEnum} == {
        "intersection",
        "pegasus",
        "pelican",
        "puffin",
        "toucan",
        "tramCrossing",
        "wigWag",
        "other",
    }


def test_mode_enum_values():
    # §4.3 narrative vs XSD disagree on startUpMode casing — XSD wins.
    assert {e.value for e in TrafficSignalModeEnum} == {
        "localMode",
        "centralMode",
        "signalsOffMode",
        "startUpMode",
        "unknownMode",
    }


def test_sub_mode_enum_values():
    # §5.1 references additional values (hurryCall, cablelessLinkingBaseTime, etc.)
    # that are NOT in §4.4 or the XSD; we follow §4.4 + XSD.
    assert {e.value for e in TrafficSignalSubModeEnum} == {
        "cablelessLinking",
        "fixedModeSimpleOrLinked",
        "fixedModeToCurrentMax",
        "manualMode",
        "manualStepOn",
        "movaMode",
        "otherDistributedMode",
        "preEmptLevel1",
        "preEmptLevel2",
        "preEmptLevel3",
        "utcFixedTime",
        "utcOtherCentralControlMethod",
        "utcScoot",
        "vehicleActuated",
        "wardenControl",
    }


def test_urgency_enum_values():
    assert {e.value for e in UrgencyEnum} == {
        "extremelyUrgent",
        "urgent",
        "normalUrgency",
        # Added to match FaultSeverityEnum; a deliberate divergence from
        # the DATEX II baseline, which omits unknown on urgency.
        "unknown",
    }


def test_fault_severity_enum_values():
    assert {e.value for e in FaultSeverityEnum} == {"low", "medium", "high", "unknown"}


def test_enums_are_str_subclasses():
    # Every enum must inherit str so JSON serialisation emits the value, not the name.
    for enum_cls in (
        TrafficSignalFaultEnum,
        TrafficSignalTypeEnum,
        TrafficSignalModeEnum,
        TrafficSignalSubModeEnum,
        UrgencyEnum,
        FaultSeverityEnum,
    ):
        assert issubclass(enum_cls, str), f"{enum_cls.__name__} must be a str Enum"
