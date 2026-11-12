# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_fault-v1.0.xsd (AbstractDeviceFaultGroup)
# Changes:    Flattened from XSD AbstractDeviceFaultGroup/Structure; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-08-29
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""CCTV Fault.

Maps to the `cctvFault` AsyncAPI channel.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId, TypeId
from topas_3000_model.common.time import Timestamp
from topas_3000_model.fault_support.enums import AcknowledgementState, FaultStatus


class CctvFault(UTMCBaseModel):
    """A fault reported for a CCTV camera."""

    cctv_id: ObjectId = Field(description="Identifier of the camera this fault is about.")
    fault_id: int = Field(ge=0, description="Unique identifier of this fault report.")
    fault_type_id: TypeId = Field(description="Reference to a FaultTypeDefinition entry.")
    creation_date: Timestamp = Field(description="When the fault was first raised.")
    last_updated: Timestamp | None = Field(default=None, description="When last modified.")
    cleared_date: Timestamp | None = Field(default=None, description="When cleared.")
    description: str | None = Field(default=None, description="Free-text description.")
    acknowledgement_state: AcknowledgementState | None = Field(default=None, description="Acknowledgement state.")
    acknowledgement_date: Timestamp | None = Field(default=None, description="When acknowledged.")
    severity: int | None = Field(default=None, ge=0, description="Fault severity.")
    status: FaultStatus | None = Field(default=None, description="Active or cleared.")
    # Values for the placeholders in the catalogue entry's messageTemplate
    # Positional and ordered; a consumer with the catalogue can render
    # the message, one without it still has the raw values.
    parameters: list[str] | None = Field(
        default=None,
        min_length=1,
        description=(
            "Values for the positional placeholders in the referenced "
            "FaultTypeDefinition's messageTemplate, in order. Rendered as "
            "text so a value carries whatever form the fault reports it in."
        ),
    )
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")
