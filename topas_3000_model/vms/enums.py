# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_vms-v1.0.xsd (various enumeration types)
# Changes:    String enum values replace XSD integer codes; inline in domain module; VmsStatus is a closed enum where v1 has an extensible type catalogue
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""VMS domain-specific enumerations."""

from enum import Enum


class VmsColourCapability(str, Enum):
    """Colour capability of a variable message sign."""

    MONOCHROME = "monochrome"
    AMBER = "amber"
    MULTI_COLOUR = "multiColour"
    FULL_COLOUR = "fullColour"


class VmsStatus(str, Enum):
    """Operational status of a variable message sign."""

    OFF = "off"
    ON = "on"
    BLANK = "blank"
    DISPLAYING = "displaying"
    FAULT = "fault"
    UNKNOWN = "unknown"
