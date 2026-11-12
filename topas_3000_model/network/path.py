# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_network-v1.0.xsd (NetworkPath)
# Changes:    Ordered list[ObjectId] replaces XSD OrderId; optional type_id; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation; type_id optional where v1 required it, trim fields float >= 0 where v1 had an unbounded integer; link_ids may not be empty
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Network Path.

An ordered route through the network (e.g. a bus route, diversion route).

Field sources: utmc_network-v1.0.xsd NetworkPath.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId, TypeId


class NetworkPath(UTMCBaseModel):
    """An ordered route through the network."""

    path_id: ObjectId = Field(description="Unique identifier of this path.")
    type_id: TypeId | None = Field(
        default=None,
        description="Path type classification (e.g. bus route, diversion).",
    )
    description: str | None = Field(default=None, description="Human-readable name.")
    permanent: bool | None = Field(
        default=None, description="Whether this is a permanent or temporary path.",
    )
    link_ids: list[ObjectId] | None = Field(
        default=None,
        min_length=1,
        description=(
            "Ordered links defining the path. List order is the route sequence. "
            "Omit the field rather than send an empty list. A link may appear "
            "more than once, since a route can traverse the same link twice."
        ),
    )
    initial_trim_meters: float | None = Field(
        default=None, ge=0.0,
        description="Distance to trim from the start of the first link.",
    )
    final_trim_meters: float | None = Field(
        default=None, ge=0.0,
        description="Distance to trim from the end of the last link.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None, description="Vendor or local extensions.",
    )
