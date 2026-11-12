# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_fault-v1.0.xsd (AbstractDeviceFaultGroup, AbstractDeviceFaultStructure)
# Changes:    Flattened from XSD AbstractDeviceFaultGroup/Structure; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation; supplierFaultNumber, subsystemTypeId (SubSystemTypeRef), equipmentFault and communicationsFault restored as optional fields
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Transport Link Fault.

TransportLinkFault is a live fault report for a specific transport link instance.

Maps to the `transportLinkFault` AsyncAPI channel.

Field sources: utmc_device_fault-v1.0.xsd AbstractDeviceFaultGroup
(CreationDate, LastUpdated, ClearedDate, Description) and
AbstractDeviceFaultStructure (FaultTypeRef, SystemCodeNumber), plus
DeviceAcknowledgmentGroup (AcknowledgmentDate, AcknowledgementStateTypeRef)
which was flattened in Phase 1.

There is no TransportLinkFaultType subclass: the Stage 1 hardening pass removed
empty domain-specific fault-type wrappers. Fault-type entries for
transport links live in the shared fault_support catalogue
(FaultTypeDefinition) and are referenced by the string fault_type_id
below (a TypeId).

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


class TransportLinkFault(UTMCBaseModel):
    """A fault reported for a transport link."""

    transport_link_id: ObjectId = Field(
        description="Identifier of the transport link this fault is about.",
    )
    fault_id: int = Field(
        ge=0,
        description="Unique identifier of this fault report within the owning system.",
    )
    fault_type_id: TypeId = Field(
        description=(
            "Reference to a FaultTypeDefinition entry in the shared fault "
            "catalogue."
        ),
    )
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
    creation_date: Timestamp = Field(
        description="When the fault was first raised.",
    )
    last_updated: Timestamp | None = Field(
        default=None,
        description="When the fault record was last modified.",
    )
    cleared_date: Timestamp | None = Field(
        default=None,
        description="When the fault was cleared. If present, the fault is historical.",
    )
    description: str | None = Field(
        default=None,
        description="Free-text description of the fault.",
    )
    acknowledgement_state: AcknowledgementState | None = Field(
        default=None,
        description="Current acknowledgement state.",
    )
    acknowledgement_date: Timestamp | None = Field(
        default=None,
        description="When the fault was acknowledged.",
    )
    severity: int | None = Field(
        default=None,
        ge=0,
        description="Fault severity (higher = more severe).",
    )
    status: FaultStatus | None = Field(
        default=None,
        description="Whether this fault is still active or has been cleared.",
    )
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
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )
