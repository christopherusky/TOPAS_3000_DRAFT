# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_access_control-v1.0.xsd (AccessControlDynamicGroup)
# Changes:    Single flat type; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Access Control dynamic (live state) payload.

Maps to the `accessControlMeasurement` AsyncAPI channel. A single flat
type carrying the current active state and flow count.

Field sources: utmc_data_object_access_control-v1.0.xsd
AccessControlDynamicGroup.
"""

from enum import Enum
from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.quality import Quality
from topas_3000_model.common.time import Timestamp


class AccessControlState(str, Enum):
    """Operational state of an access control device."""

    ENABLED = "enabled"
    DISABLED = "disabled"


class AccessControlDynamic(UTMCBaseModel):
    """Live operational state for an access control device."""

    access_control_id: ObjectId = Field(
        description="Identifier of the device this reading is about.",
    )
    timestamp: Timestamp = Field(
        description="Time the state was observed.",
    )
    quality: Quality | None = Field(
        default=None,
        description="Optional quality statement for this reading.",
    )
    active: bool = Field(
        description="Whether the device is currently active.",
    )
    state_type: AccessControlState = Field(
        description="Current operational state (enabled/disabled).",
    )
    flow: int | None = Field(
        default=None,
        ge=0,
        description="Vehicle count over the flow interval.",
    )
    flow_interval_seconds: int | None = Field(
        default=None,
        gt=0,
        description="Duration of the flow measurement period in seconds.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )
