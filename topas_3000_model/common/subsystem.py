# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_support_subsystem-v1.0.xsd (SubSystemTypeDefinition)
# Changes:    Pydantic model for subsystem definition; camelCase JSON output; shared across all domains via cross-file $ref; DataSourceTypeRef not carried
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Subsystem Type Definition for UTMC participant registry.

A subsystem definition describes a participating system or component in the
UTMC network. Each data source can have zero, one or many subsystems.
Vendor-allocated ID ranges partition the namespace:
  MOTTS 1000-1099, PEEK 1100-1199, TENET 1200-1299, etc.

Field sources: utmc_support_subsystem-v1.0.xsd SubSystemTypeDefinition,
which restricts TypeDefinitionStructure from utmc_support_typeID-v1.0.xsd.
"""

from typing import Any

from pydantic import AnyUrl, Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import TypeId


class SubsystemDefinition(UTMCBaseModel):
    """Catalogue entry for a UTMC subsystem type.

    Follows the standard TypeDefinitionStructure pattern: TypeId +
    TypeDescription + TypeNotes + SpecificationLink + Extensions.
    """

    type_id: TypeId = Field(description="Identifier of this subsystem type.")
    description: str = Field(description="Human-readable description of the subsystem.")
    notes: str | None = Field(
        default=None,
        description="Additional notes about the subsystem.",
    )
    specification_link: AnyUrl | None = Field(
        default=None,
        description="URL to an external specification document for this subsystem type.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )
