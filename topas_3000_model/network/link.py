# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_network-v1.0.xsd (NetworkLink)
# Changes:    GeoJSON LineString geometry; ObjectId reuse; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation; FeatureRef, path/zone/turn lists and the shared-geometry choice not carried
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Network Link.

A directed road segment connecting two nodes.

Field sources: utmc_network-v1.0.xsd NetworkLink.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.geometry import GeoJsonLineString
from topas_3000_model.common.ids import ObjectId


class NetworkLink(UTMCBaseModel):
    """A directed road segment connecting two nodes."""

    link_id: ObjectId = Field(description="Unique identifier of this link.")
    from_node_id: ObjectId = Field(description="Start node reference.")
    to_node_id: ObjectId = Field(description="End node reference.")
    description: str | None = Field(default=None, description="Human-readable name.")
    geometry: GeoJsonLineString | None = Field(
        default=None, description="Polyline shape of the link as GeoJSON LineString.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None, description="Vendor or local extensions.",
    )
