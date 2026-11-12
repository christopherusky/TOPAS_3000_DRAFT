# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_history-v1.0.xsd (AbstractDeviceHistoryGroup,
#             AbstractDeviceHistoryStructure) via UTMC-TS004.0061:2010 Annex D.
#             TS004.0064bis:2017 defers to TS004.0061 for concepts it does not
#             redefine (§1.1.2).
#
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-08-26
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Traffic Signal History.

A TrafficSignalHistoryEntry records one operational log line for a
traffic signal (e.g. controller reboot, firmware upgrade, manual
intervention). Maps to the `trafficSignalHistory` AsyncAPI channel.

Field sources: utmc_device_history-v1.0.xsd AbstractDeviceHistoryGroup
(LogDate, LogEntry, LogUser) and AbstractDeviceHistoryStructure
(SystemCodeNumber, LogId).
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp


class TrafficSignalHistoryEntry(UTMCBaseModel):
    """One history log entry for a traffic signal device."""

    traffic_signal_id: ObjectId = Field(
        description="Identifier of the traffic signal this log entry is about.",
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
        description="Category or type of the history event (e.g. 'configuration_change', 'fault_cleared').",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )
