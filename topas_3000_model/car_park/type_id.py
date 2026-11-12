# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_support_typeID-v1.0.xsd (TypeDefinitionGroup, TypeDefinitionStructure)
# Changes:    Catalogue class (not constrained enum) preserves vendor extensibility; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Car Park Type Definition.

Catalogue entry describing one kind of car park (multi-storey, surface,
park-and-ride, etc.). Referenced by CarParkDefinition.type_id.

Maps to the `carParkType` AsyncAPI channel.

Field sources: utmc_support_typeID-v1.0.xsd TypeDefinitionGroup and
TypeDefinitionStructure, specialised to Car Park.
"""

from typing import Any

from pydantic import AnyUrl, Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import TypeId


class CarParkType(UTMCBaseModel):
    """Catalogue entry for one kind of car park."""

    type_id: TypeId = Field(description="Identifier of this car park type.")
    description: str = Field(description="Human-readable description of the car park kind.")
    notes: str | None = Field(
        default=None,
        description="Additional notes, e.g. opening hours or restrictions.",
    )
    specification_link: AnyUrl | None = Field(
        default=None,
        description="URL to an external specification document for this car park type.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )
