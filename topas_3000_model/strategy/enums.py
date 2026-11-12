# ──────────────────────────────────────────────────────────────────────
# Origin:     DS003 Strategy Interface Specification v1.2
# Changes:    String enums for strategy status and trigger state; AMQP payloads
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Strategy domain enumerations.

From UTMC DS003 Strategy Interface Specification v1.2.
"""

from enum import Enum


class StrategyStatusEnum(str, Enum):
    """Status of a traffic management strategy."""

    ACTIVE = "active"
    INACTIVE = "inactive"


class TriggerStateEnum(str, Enum):
    """Desired state when triggering a strategy."""

    ENABLED = "enabled"
    DISABLED = "disabled"


class TriggerUpdateErrorEnum(str, Enum):
    """Categorisation of strategy trigger rejection reason."""

    ACCESS_DENIED = "accessDenied"
    NOT_AUTHENTICATED = "notAuthenticated"
    STRATEGY_ID_DOES_NOT_EXIST = "strategyIdDoesNotExist"
    OTHER = "other"
