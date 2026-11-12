# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_traffic_event-v1.0.xsd (shared enumerations and base types)
# Changes:    Dedicated common library file; I/T lane pattern strings; EventStatus lifecycle enum; lane impact counts; public description field; string enums replace XSD integer codes; lane patterns kept as a closed list, which cannot carry v1's TTII, TTTI or roads wider than four lanes
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Traffic Event shared enumerations.

Emitted as traffic_event_common-v1.0.json library file.
Cross-referenced by all 4 event subtype schemas.

Design choices: shared library, I/T lane patterns, PhaseType as plain
string, EventStatus lifecycle, lane count fields, public description.
"""

from enum import Enum


class EventStatus(str, Enum):
    """Event lifecycle state."""

    ACTIVE = "active"
    PLANNED = "planned"
    SUSPENDED = "suspended"
    CLEARED = "cleared"


class SeverityType(str, Enum):
    """Event severity."""

    UNKNOWN = "unknown"
    LOW = "low"
    MEDIUM = "medium"
    HIGH = "high"


class LanesAffectedType(str, Enum):
    """Lane closure pattern.

    I = open lane, T = closed lane, read nearside to offside.
    """

    I = "I"
    T = "T"
    II = "II"
    IT = "IT"
    TI = "TI"
    TT = "TT"
    III = "III"
    IIT = "IIT"
    ITI = "ITI"
    ITT = "ITT"
    TII = "TII"
    TIT = "TIT"
    TTI = "TTI"
    TTT = "TTT"
    IIII = "IIII"
    IIIT = "IIIT"
    IITT = "IITT"
    ITTT = "ITTT"
    TIII = "TIII"
    TTTT = "TTTT"
    ALL_OPEN = "allOpen"
