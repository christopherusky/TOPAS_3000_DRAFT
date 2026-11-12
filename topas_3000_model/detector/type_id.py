# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_support_typeID-v1.0.xsd (TypeDefinitionGroup, TypeDefinitionStructure)
# Changes:    Catalogue class (not constrained enum) preserves vendor extensibility; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Detector Type Definition.

Catalogue entry describing one kind of detector (inductive loop, radar,
ANPR camera, etc.). Referenced by DetectorDefinition.type_ref.

Maps to the `detectorType` AsyncAPI channel.

Field sources: utmc_support_typeID-v1.0.xsd TypeDefinitionGroup and
TypeDefinitionStructure, specialised to Detector.
"""

from typing import Any

from pydantic import AnyUrl, Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import TypeId


class DetectorType(UTMCBaseModel):
    """Catalogue entry for one kind of detector."""

    type_id: TypeId = Field(description="Identifier of this detector type.")
    description: str = Field(description="Human-readable description of the detector kind.")
    notes: str | None = Field(
        default=None,
        description="Additional notes, e.g. deployment details.",
    )
    specification_link: AnyUrl | None = Field(
        default=None,
        description="URL to an external specification document for this detector type.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )
