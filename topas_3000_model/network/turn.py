# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_network-v1.0.xsd (NetworkTurn)
# Changes:    Faithful model with from/to link IDs and allowed flag; GeoJSON LineString geometry; camelCase JSON output via Pydantic alias_generator
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Network Turn.

An allowed or prohibited movement between two links at a junction.

Field sources: utmc_network-v1.0.xsd NetworkTurn.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.geometry import GeoJsonLineString
from topas_3000_model.common.ids import ObjectId


class NetworkTurn(UTMCBaseModel):
    """An allowed or prohibited movement between two links at a junction."""

    turn_id: ObjectId = Field(description="Unique identifier of this turn.")
    from_link_id: ObjectId = Field(description="Approach link.")
    to_link_id: ObjectId = Field(description="Departure link.")
    allowed: bool = Field(description="Whether this turn is permitted.")
    description: str | None = Field(default=None, description="Human-readable description.")
    geometry: GeoJsonLineString | None = Field(
        default=None, description="Turn path geometry as GeoJSON LineString.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None, description="Vendor or local extensions.",
    )
