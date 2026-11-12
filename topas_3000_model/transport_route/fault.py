# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_fault-v1.0.xsd (AbstractDeviceFaultGroup)
# Changes:    Flattened from XSD AbstractDeviceFaultGroup/Structure; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation; supplierFaultNumber, subsystemTypeId (SubSystemTypeRef), equipmentFault and communicationsFault restored as optional fields
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Transport Route Fault.

Maps to the `transportRouteFault` AsyncAPI channel.

SupplierFaultNumber and SubSystemTypeRef (DeviceFaultIdentificationGroup) and
EquipmentFault and CommunicationsFault (DeviceFaultClassificationGroup) were
dropped without a record and are restored as optional fields.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId, TypeId
from topas_3000_model.common.time import Timestamp
from topas_3000_model.fault_support.enums import AcknowledgementState, FaultStatus


class TransportRouteFault(UTMCBaseModel):
    """A fault reported for a transport route."""

    transport_route_id: ObjectId = Field(description="Identifier of the route this fault is about.")
    fault_id: int = Field(ge=0, description="Unique identifier of this fault report.")
    fault_type_id: TypeId = Field(description="Reference to a FaultTypeDefinition entry.")
    supplier_fault_number: str | None = Field(
        default=None,
        description=(
            "Fault identifier allocated by the supplier or configuration "
            "manager for maintenance purposes, if required."
        ),
    )
    subsystem_type_id: TypeId | None = Field(
        default=None,
        description=(
            "Reference to a SubsystemDefinition entry: the subsystem the "
            "fault arose in."
        ),
    )
    equipment_fault: bool | None = Field(
        default=None,
        description=(
            "Whether this is an equipment fault. The fault type's catalogue "
            "category classifies the type; this flag classifies this fault."
        ),
    )
    communications_fault: bool | None = Field(
        default=None,
        description=(
            "Whether this is a communications fault. See equipmentFault for "
            "how it relates to the catalogue category."
        ),
    )
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
