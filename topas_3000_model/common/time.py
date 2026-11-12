# ──────────────────────────────────────────────────────────────────────
# Origin:     Cross-cutting; used by all UTMC XSD timestamp fields
# Changes:    AwareDatetime alias ensures timezone-aware parsing
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Timezone-aware datetime alias for all UTMC timestamps.

All UTMC timestamps MUST be timezone-aware (UTC or explicit offset). Naive
datetimes are rejected at parse time.
"""

from pydantic import AwareDatetime

Timestamp = AwareDatetime
"""Alias for Pydantic's AwareDatetime. Serializes to ISO 8601 with timezone."""
