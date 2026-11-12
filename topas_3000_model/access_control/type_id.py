# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_support_typeID-v1.0.xsd; XSD reference values: 1=Bollard, 2=Barrier, 3=Gate, 4=Sign, 5=Other
# Changes:    Catalogue class (not constrained enum) preserves vendor extensibility; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Access Control Type Definition.

Catalogue entry describing one kind of access control device.
Referenced by AccessControlDefinition.type_id.

Maps to the `accessControlType` AsyncAPI channel.

XSD reference values: 1=Bollard, 2=Barrier, 3=Gate, 4=Sign, 5=Other.
"""

from typing import Any

from pydantic import AnyUrl, Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import TypeId


class AccessControlType(UTMCBaseModel):
    """Catalogue entry for one kind of access control device."""

    type_id: TypeId = Field(description="Identifier of this access control type.")
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
