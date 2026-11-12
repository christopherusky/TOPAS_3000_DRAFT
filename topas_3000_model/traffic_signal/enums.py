# ──────────────────────────────────────────────────────────────────────
# Origin:     UTMC-TS004.0064bis:2017 §4.1–4.4 + inherited UrgencyEnum/FaultSeverityEnum (§3.3)
# XSD:        UTMCv1_xml/UTMC-TS004.0064bis-2017-traffic-signal-interface.xsd
#             simpleTypes: TrafficSignalFaultEnum, TrafficSignalTypeEnum,
#             TrafficSignalModeEnum, TrafficSignalSubModeEnum, UrgencyEnum,
#             FaultSeverityEnum
# Changes:    XSD casing/values authoritative over PDF narrative: startUpMode
#             capitalisation, lowercase-first camelCase for type-enum values.
#             §5.1 mode/subMode combinations are documentation only.
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-08-26
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Enumerations for the UTMC-TS004.0064bis:2017 traffic-signal interface.

Where the PDF narrative and the XSD disagree on value casing, the XSD is
authoritative because it is what vendors implement.

Section 5.1 of the PDF references sub-mode values that do not exist in
§4.4 or the XSD (`hurryCall`, `cablelessLinkingBaseTime`,
`cablelessLinkingNonBaseTime`, `publicServiceVehiclePriority`,
`lightRailTransitMode`, `pre-emption`). By design this is treated as
informative documentation; no combination validator is applied.
"""

from enum import Enum


class TrafficSignalFaultEnum(str, Enum):
    """PDF §4.1 — traffic-signal fault types."""

    DETECTOR_FAULT = "detectorFault"
    ELECTRONICS_FAULT = "electronicsFault"
    INDICATOR_FAULT = "indicatorFault"
    LAMP_FAULT = "lampFault"
    MECHANICAL_FAULT = "mechanicalFault"
    UTILITY_FAULT = "utilityFault"
    OTHER = "other"


class TrafficSignalTypeEnum(str, Enum):
    """PDF §4.2 — traffic-signal types. XSD values (camelCase) are authoritative."""

    INTERSECTION = "intersection"
    PEGASUS = "pegasus"
    PELICAN = "pelican"
    PUFFIN = "puffin"
    TOUCAN = "toucan"
    TRAM_CROSSING = "tramCrossing"
    WIG_WAG = "wigWag"
    OTHER = "other"


class TrafficSignalModeEnum(str, Enum):
    """PDF §4.3 — high-level traffic-signal modes."""

    LOCAL_MODE = "localMode"
    CENTRAL_MODE = "centralMode"
    SIGNALS_OFF_MODE = "signalsOffMode"
    START_UP_MODE = "startUpMode"
    UNKNOWN_MODE = "unknownMode"


class TrafficSignalSubModeEnum(str, Enum):
    """PDF §4.4 — detailed traffic-signal sub-modes.

    Note: §5.1's mode/subMode combinations table references additional values
    (hurryCall, cablelessLinkingBaseTime, cablelessLinkingNonBaseTime,
    publicServiceVehiclePriority, lightRailTransitMode, pre-emption) that are
    NOT defined in §4.4 or the XSD. This enum follows §4.4 + XSD.
    """

    CABLELESS_LINKING = "cablelessLinking"
    FIXED_MODE_SIMPLE_OR_LINKED = "fixedModeSimpleOrLinked"
    FIXED_MODE_TO_CURRENT_MAX = "fixedModeToCurrentMax"
    MANUAL_MODE = "manualMode"
    MANUAL_STEP_ON = "manualStepOn"
    MOVA_MODE = "movaMode"
    OTHER_DISTRIBUTED_MODE = "otherDistributedMode"
    PRE_EMPT_LEVEL_1 = "preEmptLevel1"
    PRE_EMPT_LEVEL_2 = "preEmptLevel2"
    PRE_EMPT_LEVEL_3 = "preEmptLevel3"
    UTC_FIXED_TIME = "utcFixedTime"
    UTC_OTHER_CENTRAL_CONTROL_METHOD = "utcOtherCentralControlMethod"
    UTC_SCOOT = "utcScoot"
    VEHICLE_ACTUATED = "vehicleActuated"
    WARDEN_CONTROL = "wardenControl"


class UrgencyEnum(str, Enum):
    """DATEX II UrgencyEnum — inherited by TrafficSignalFault.trafficSignalFaultUrgency (PDF §3.3).

    ``UNKNOWN`` is a deliberate addition to the DATEX II baseline: a
    publisher that cannot determine urgency must be able to say so rather
    than defaulting to ``normalUrgency`` and understating a fault. This
    mirrors ``FaultSeverityEnum.UNKNOWN``, which already carries the value.
    """

    EXTREMELY_URGENT = "extremelyUrgent"
    URGENT = "urgent"
    NORMAL_URGENCY = "normalUrgency"
    UNKNOWN = "unknown"


class FaultSeverityEnum(str, Enum):
    """DATEX II FaultSeverityEnum — inherited via Fault base (PDF §3.3)."""

    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"
    UNKNOWN = "unknown"
