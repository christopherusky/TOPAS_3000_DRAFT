# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_fault-v1.0.xsd (AcknowledgementStateTypeIdType, fault status conventions)
# Changes:    String enum values replace XSD integer codes; FaultStatus and AcknowledgementState shared by all fault models; AcknowledgementState is a closed enum where v1 has an extensible type catalogue
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Shared fault support enumerations.

Source: utmc_device_fault-v1.0.xsd AcknowledgementStateTypeIdType. The XSD
uses integer values 0-3 with descriptions in prose; we convert to a clean
string enum with meaningful names.
"""

from enum import Enum


class FaultStatus(str, Enum):
    """Whether a fault is still active or has been cleared.

    This is the only fault lifecycle the payload carries. Record archival
    — distinguishing a live record from a retired one — is not modelled
    here and is a matter for the publishing system.
    """

    ACTIVE = "active"
    CLEARED = "cleared"


class AcknowledgementState(str, Enum):
    """Acknowledgement state of a device fault.

    Maps from legacy XSD integer values:
    - 0 -> notAcknowledged
    - 1 -> acknowledged
    - 2 -> deferred
    - 3 -> recordedForAction
    """

    NOT_ACKNOWLEDGED = "notAcknowledged"
    ACKNOWLEDGED = "acknowledged"
    DEFERRED = "deferred"
    RECORDED_FOR_ACTION = "recordedForAction"


class FaultCategory(str, Enum):
    """Coarse grouping for catalogue entries.

    Deliberately short. It exists so an operator can filter "show me the
    communications faults" without the catalogue having to agree on a
    taxonomy first; the precise meaning of a fault is its catalogue entry,
    not its category.
    """

    HARDWARE = "hardware"
    COMMUNICATIONS = "communications"
    SIGNAL_TIMINGS = "signalTimings"
    ENVIRONMENTAL = "environmental"
    OTHER = "other"
