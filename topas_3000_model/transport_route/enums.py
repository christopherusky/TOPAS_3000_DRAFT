# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_transport_route-v1.0.xsd (transport route status values)
# Changes:    String enum (inRange/outOfRange) carries the XSD TransportRouteStatusTypeIdEnum values unchanged
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-05
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Transport Route domain-specific enumerations."""

from enum import Enum


class TransportRouteStatus(str, Enum):
    """Route status (a closed string enum, not an extensible TypeId catalogue)."""

    IN_RANGE = "inRange"
    OUT_OF_RANGE = "outOfRange"
