# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_fault-v1.0.xsd (AbstractFaultTypeDefinitionGroup, AbstractFaultTypeDefinitionStructure)
# Changes:    Flattened Group+Structure into single FaultTypeDefinition class; shared across all domains via cross-file $ref; subsystemTypeId (SubSystemTypeRef) restored as an optional field
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Shared fault type definition base.

Each domain's FaultType (e.g. DetectorFaultType) references an identifier
from this shared catalogue. The FaultTypeDefinition itself carries the
description and severity; the domain-specific wrappers add device-type
specifics.

Field sources: utmc_device_fault-v1.0.xsd AbstractFaultTypeDefinitionGroup
(FaultDescription, FaultSeverity) and AbstractFaultTypeDefinitionStructure
(FaultTypeId).

SubSystemTypeRef (AbstractFaultTypeDefinitionGroup) was dropped without a record
and is restored as an optional field.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import TypeId
from topas_3000_model.fault_support.enums import FaultCategory


class FaultTypeDefinition(UTMCBaseModel):
    """Catalogue entry describing one kind of fault that can be reported."""

    fault_type_id: TypeId = Field(description="Identifier of this fault type.")
    description: str = Field(description="Human-readable description of the fault type.")
    severity: int | None = Field(
        default=None,
        ge=0,
        description="Numeric severity (higher = more severe). Optional.",
    )
    subsystem_type_id: TypeId | None = Field(
        default=None,
        description=(
            "Reference to a SubsystemDefinition entry: the subsystem this "
            "fault type belongs to."
        ),
    )
    # ── Catalogue extensions ────────────────────────────────────
    code: str | None = Field(
        default=None,
        min_length=1,
        max_length=32,
        description=(
            "The identifier this fault carries in its source document or "
            "vendor documentation — 'A1', 'F3', 'W12'. Distinct from "
            "faultTypeId, which is this catalogue's key: a deployment may "
            "hold two entries with the same code from different vendors."
        ),
    )
    category: FaultCategory | None = Field(
        default=None,
        description="Coarse grouping, for filtering rather than semantics.",
    )
    message_template: str | None = Field(
        default=None,
        min_length=1,
        description=(
            "Human-readable message with positional placeholders, e.g. "
            "'Minimum green not met: stage {0} ran {1}s against a minimum "
            "of {2}s'. A fault instance supplies the values in its "
            "parameters field. This is what lets a catalogue carry faults "
            "whose meaning depends on runtime values — the stuck-stage and "
            "interstage timing faults that a closed enum cannot express."
        ),
    )
    parameter_names: list[str] | None = Field(
        default=None,
        min_length=1,
        description=(
            "Names for the positional placeholders in messageTemplate, in "
            "order — ['stageNumber', 'actualSeconds', 'minimumSeconds']. "
            "Lets a consumer label the values rather than only interpolate "
            "them. Length SHOULD match the number of placeholders."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )
