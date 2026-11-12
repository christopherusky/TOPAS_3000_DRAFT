# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_network-v1.0.xsd (NetworkNode)
# Changes:    GeoJSON Point geometry; ObjectId reuse; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Network Node.

A junction or intersection point on the road network.

Field sources: utmc_network-v1.0.xsd NetworkNode.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.geometry import GeoJsonPoint
from topas_3000_model.common.ids import ObjectId


class NetworkNode(UTMCBaseModel):
    """A junction or intersection point on the road network."""

    node_id: ObjectId = Field(description="Unique identifier of this node.")
    description: str | None = Field(default=None, description="Human-readable name.")
    location: GeoJsonPoint | None = Field(
        default=None, description="Geographic position as GeoJSON Point.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None, description="Vendor or local extensions.",
    )
