# ──────────────────────────────────────────────────────────────────────
# Origin:     UTMC-TS004.0063:2015 Updated Environment/Weather Data Model
# Changes:    String enums for threshold type, checked type, data status; camelCase JSON output via Pydantic alias_generator
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Air Quality Enhanced domain enumerations.

From UTMC-TS004.0063:2015 Updated Environment/Weather Data Model.
"""

from enum import Enum


class ThresholdType(str, Enum):
    """Threshold hysteresis type.

    Controls how threshold crossings are detected:
    - rising: triggers when value rises above threshold
    - falling: triggers when value falls below threshold
    - risingAndFalling: triggers on both crossings
    """

    RISING = "rising"
    FALLING = "falling"
    RISING_AND_FALLING = "risingAndFalling"


class CheckedType(str, Enum):
    """Data validation level applied to measurement.

    0 = raw from sensor, 1 = validated (cross-checked),
    2 = ratified (AURN fully validated), 99 = unknown.
    """

    RAW = "raw"
    VALIDATED = "validated"
    RATIFIED = "ratified"
    UNKNOWN = "unknown"


class DataStatusType(str, Enum):
    """Data quality status of measurement.

    0 = normal, 1 = incomplete (missing values),
    2 = inconsistent (time alignment issues),
    3 = suspect (other reason), 99 = unknown.
    """

    NORMAL = "normal"
    INCOMPLETE = "incomplete"
    INCONSISTENT = "inconsistent"
    SUSPECT = "suspect"
    UNKNOWN = "unknown"
