# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_history-v1.0.xsd (AbstractDeviceHistoryGroup, AbstractDeviceHistoryStructure)
# Changes:    Flattened from XSD AbstractDeviceHistoryGroup/Structure; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Detector History.

A DetectorHistoryEntry records one operational log line for a detector
(e.g. recalibration, threshold change, hardware swap). Maps to the
`detectorHistory` AsyncAPI channel.

Field sources: utmc_device_history-v1.0.xsd AbstractDeviceHistoryGroup
(LogDate, LogEntry, LogUser) and AbstractDeviceHistoryStructure
(SystemCodeNumber, LogId), specialised to Detector.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp


class DetectorHistoryEntry(UTMCBaseModel):
    """One history log entry for a detector device."""

    detector_id: ObjectId = Field(
        description="Identifier of the detector this log entry is about.",
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
        description="Category or type of the history event (e.g. 'recalibration', 'threshold_change').",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )
