# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_history-v1.0.xsd (AbstractDeviceHistoryGroup, AbstractDeviceHistoryStructure)
# Changes:    Flattened from XSD AbstractDeviceHistoryGroup/Structure; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Car Park History.

A CarParkHistoryEntry records one operational log line for a car park
(e.g. capacity change, threshold adjustment, equipment swap). Maps to
the `carParkHistory` AsyncAPI channel.

Field sources: utmc_device_history-v1.0.xsd AbstractDeviceHistoryGroup
(LogDate, LogEntry, LogUser) and AbstractDeviceHistoryStructure
(SystemCodeNumber, LogId), specialised to Car Park.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp


class CarParkHistoryEntry(UTMCBaseModel):
    """One history log entry for a car park."""

    car_park_id: ObjectId = Field(
        description="Identifier of the car park this log entry is about.",
    )
    log_id: int = Field(
        ge=0,
        description="Unique identifier of this log entry within the owning system.",
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
        description="Category or type of the history event (e.g. 'capacity_change', 'threshold_adjustment').",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )
