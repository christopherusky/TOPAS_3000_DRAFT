# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_history-v1.0.xsd (AbstractDeviceHistoryGroup)
# Changes:    Flattened from XSD AbstractDeviceHistoryGroup/Structure; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Access Control History.

Maps to the `accessControlHistory` AsyncAPI channel.

Field sources: utmc_device_history-v1.0.xsd AbstractDeviceHistoryGroup.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp


class AccessControlHistoryEntry(UTMCBaseModel):
    """One history log entry for an access control device."""

    access_control_id: ObjectId = Field(
        description="Identifier of the device this log entry is about.",
    )
    log_id: int = Field(
        ge=0,
        description="Unique identifier of this log entry.",
    )
    log_date: Timestamp = Field(
        description="When the log entry was recorded.",
    )
    log_entry: str = Field(
        description="Equipment log data (free-form text).",
    )
    log_user: str | None = Field(
        default=None,
        description="User who created the log entry.",
    )
    event_type: str | None = Field(
        default=None,
        description="Category of the history event.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )
