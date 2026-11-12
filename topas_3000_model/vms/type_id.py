# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_support_typeID-v1.0.xsd (TypeDefinitionGroup, TypeDefinitionStructure)
# Changes:    Catalogue class (not constrained enum) preserves vendor extensibility; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""VMS Type Definition.

Catalogue entry describing one kind of variable message sign.
Referenced by VmsDefinition.type_id.

Maps to the `vmsType` AsyncAPI channel.
"""

from typing import Any

from pydantic import AnyUrl, Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import TypeId


class VmsType(UTMCBaseModel):
    """Catalogue entry for one kind of variable message sign."""

    type_id: TypeId = Field(description="Identifier of this VMS type.")
    description: str = Field(description="Human-readable description.")
    notes: str | None = Field(
        default=None,
        description="Additional notes.",
    )
    specification_link: AnyUrl | None = Field(
        default=None,
        description="URL to an external specification document.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )
