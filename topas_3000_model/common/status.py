# ──────────────────────────────────────────────────────────────────────
# Origin:     Cross-cutting; no single XSD source
# Changes:    RecordStatus enum for lifecycle state (active/cleared/historic), reserved and unused pending the lifecycle design
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Record lifecycle status for UTMC domain objects.

RESERVED, NOT YET USED. No model carries this enum. Faults report
``FaultStatus`` (active/cleared) and ``historicDate`` was removed, so
``historic`` currently has no producer.

The enum is kept because a lifecycle for definitions and other records is
planned: whether and where it applies, how it relates to ``FaultStatus``, and
how a retired object is signalled in sync and delta modes are undecided.
Do not attach it to a model before
that design is agreed.
"""

from enum import Enum


class RecordStatus(str, Enum):
    """Lifecycle status of a domain record."""

    ACTIVE = "active"
    CLEARED = "cleared"
    HISTORIC = "historic"
