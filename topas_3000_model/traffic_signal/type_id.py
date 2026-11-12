# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_support_typeID-v1.0.xsd (TypeDefinitionGroup, TypeDefinitionStructure)
#             via UTMC-TS004.0061:2010 Annex D. TS004.0064bis:2017 defers to
#             TS004.0061 for concepts it does not redefine (§1.1.2).
# Changes:    Standalone copy — the 2017 profile previously re-exported from
#             topas_3000_model.traffic_signal_2010.type_id but now duplicates the
#             class locally so the package is self-contained.
#             Catalogue class (not constrained enum) preserves vendor
#             extensibility.
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-05-07
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Traffic Signal Type Definition.

Open catalogue describing one *control method* (fixed-time, MOVA,
SCOOT, vehicle-actuated, etc.). Referenced by
``TrafficSignalDefinition.control_method_id``.

This is intentionally distinct from
``TrafficSignalConfiguration.trafficSignalType`` (a closed enum
covering junction kinds: intersection, pelican, puffin, toucan,
pegasus, tramCrossing, wigWag). The two axes are orthogonal — a
single junction kind can run any control method, and a single
control method classifies many junction kinds.

Open catalogue (rather than closed enum) so operators can publish
vendor-specific or in-house control methods alongside the
industry-standard ones.

Maps to the ``trafficSignalType`` AsyncAPI channel.

Field sources: utmc_support_typeID-v1.0.xsd TypeDefinitionGroup and
TypeDefinitionStructure, specialised to TrafficSignal.
"""

from typing import Any

from pydantic import AnyUrl, Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import TypeId


class TrafficSignalType(UTMCBaseModel):
    """Open catalogue entry for one traffic signal *control method*."""

    type_id: TypeId = Field(
        description=(
            "Identifier of this control method. Referenced by "
            "TrafficSignalDefinition.controlMethodId."
        ),
    )
    description: str = Field(
        description=(
            "Human-readable description of the control method "
            "(e.g. 'MOVA', 'SCOOT', 'fixed-time', 'vehicle-actuated')."
        ),
    )
    notes: str | None = Field(
        default=None,
        description="Additional notes, e.g. deployment details.",
    )
    specification_link: AnyUrl | None = Field(
        default=None,
        description="URL to an external specification document for this control method.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )
