# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_support_typeID-v1.0.xsd; XSD reference values: 1=Colour CCD, 2=B/W CCD, 3=ANPR, 4=Infra-red, 999=Other
# Changes:    Catalogue class (not constrained enum) preserves vendor extensibility; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""CCTV Type Definition.

XSD reference values: 1=Colour CCD, 2=B/W CCD, 3=ANPR,
4=Infra-red, 999=Other.

Maps to the `cctvType` AsyncAPI channel.
"""

from typing import Any

from pydantic import AnyUrl, Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import TypeId


class CctvType(UTMCBaseModel):
    """Catalogue entry for one kind of CCTV camera."""

    type_id: TypeId = Field(description="Identifier of this CCTV type.")
    description: str = Field(description="Human-readable description.")
    notes: str | None = Field(default=None, description="Additional notes.")
    specification_link: AnyUrl | None = Field(default=None, description="URL to specification.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")
