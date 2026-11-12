# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_network-v1.0.xsd (NetworkZone)
# Changes:    Unordered set of link ids carried as list[ObjectId], since the XSD zone has no OrderId; optional type_id; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation; type_id optional where v1 required it; link_ids may not be empty or repeat
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.3
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Network Zone.

A geographic area defined by a set of links.

Field sources: utmc_network-v1.0.xsd NetworkZone.
"""

from typing import Any

from pydantic import Field, field_validator

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.geometry import GeoJsonLineString
from topas_3000_model.common.ids import ObjectId, TypeId


class NetworkZone(UTMCBaseModel):
    """A geographic area defined by an ordered set of links."""

    zone_id: ObjectId = Field(description="Unique identifier of this zone.")
    type_id: TypeId | None = Field(
        default=None,
        description="Zone type classification (e.g. electoral ward, incident area).",
    )
    description: str | None = Field(default=None, description="Human-readable name.")
    permanent: bool | None = Field(
        default=None,
        description=(
            "Whether this is a permanent or temporary zone. Absent means permanent "
            "(the v1 default for a zone)."
        ),
    )
    link_ids: list[ObjectId] | None = Field(
        default=None,
        min_length=1,
        description=(
            "The links that make up the zone, as a set: the order of the array carries "
            "no meaning (v1 describes a zone as an unordered sequence of links). "
            "Omit the field rather than send an empty list. Each link appears "
            "at most once."
        ),
    )
    geometry: GeoJsonLineString | None = Field(
        default=None, description="Zone boundary geometry as GeoJSON LineString.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None, description="Vendor or local extensions.",
    )

    # Python-only: this rule is not in the emitted schema, so other SDKs do not enforce it.
    @field_validator("link_ids")
    @classmethod
    def _link_ids_unique(cls, v: list[ObjectId] | None) -> list[ObjectId] | None:
        if v is not None and len(set(v)) != len(v):
            raise ValueError("link_ids must not contain duplicates")
        return v
