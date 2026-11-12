# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_vms-v1.0.xsd (VmsDynamicGroup)
# Changes:    Single flat type; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation; GraphicRef and GraphicImage not carried
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""VMS dynamic payload.

Maps to the `vmsMeasurement` AsyncAPI channel.
"""

from typing import Any

from pydantic import AnyUrl, Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.quality import Quality
from topas_3000_model.common.time import Timestamp
from topas_3000_model.vms.enums import VmsStatus


class VmsDynamic(UTMCBaseModel):
    """Live state of a variable message sign."""

    vms_id: ObjectId = Field(description="Identifier of the VMS.")
    timestamp: Timestamp = Field(description="Time of the observation.")
    quality: Quality | None = Field(default=None, description="Optional quality statement.")
    current_message: str | None = Field(default=None, description="Currently displayed message text.")
    message_lines: list[str] | None = Field(default=None, description="Message split by lines.")
    graphic_url: AnyUrl | None = Field(default=None, description="URL of the currently displayed graphic.")
    brightness_percent: float | None = Field(
        default=None, ge=0.0, le=100.0, description="Current brightness percentage.",
    )
    message_set_time: Timestamp | None = Field(default=None, description="When the current message was set.")
    message_expiry_time: Timestamp | None = Field(default=None, description="When the message expires.")
    status: VmsStatus | None = Field(default=None, description="Operational status.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")
