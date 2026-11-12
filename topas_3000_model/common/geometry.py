# ──────────────────────────────────────────────────────────────────────
# Origin:     RFC 7946 (GeoJSON); no XSD source
# Changes:    GeoJsonPoint and GeoJsonLineString for geographic data; [longitude, latitude] coordinate order per RFC 7946; each position bounded to the WGS84 range (lon ±180, lat ±90) and finite, matching common.location.Coordinates and the XSD LongitudeType/LatitudeType; placed in common/ as shared primitives; positions are two-dimensional (no altitude), a LineString has at least two positions, and there is no Polygon
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.3
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""GeoJSON geometry primitives (RFC 7946).

GeoJsonPoint and GeoJsonLineString for geographic data. Added for the
Network domain but placed in common/ as shared primitives.

IMPORTANT: GeoJSON uses [longitude, latitude] coordinate order per
RFC 7946. This is the opposite of the existing Point model which has
separate latitude/longitude fields.
"""

from typing import Annotated, Literal

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel


Longitude = Annotated[float, Field(ge=-180.0, le=180.0)]
"""Longitude in decimal degrees (WGS84), -180 to 180. The bounds also reject NaN and infinity."""

Latitude = Annotated[float, Field(ge=-90.0, le=90.0)]
"""Latitude in decimal degrees (WGS84), -90 to 90. The bounds also reject NaN and infinity."""

Position = tuple[Longitude, Latitude]
"""A GeoJSON position: [longitude, latitude], each within the WGS84 range."""


class GeoJsonPoint(UTMCBaseModel):
    """A GeoJSON Point geometry (RFC 7946).

    Coordinates are [longitude, latitude] per GeoJSON specification.
    """

    type: Literal["Point"] = "Point"
    coordinates: Position = Field(
        description="[longitude, latitude] in WGS84 decimal degrees.",
    )


class GeoJsonLineString(UTMCBaseModel):
    """A GeoJSON LineString geometry (RFC 7946).

    An ordered sequence of two or more positions forming a polyline.
    Coordinates are [longitude, latitude] per GeoJSON specification.
    """

    type: Literal["LineString"] = "LineString"
    coordinates: list[Position] = Field(
        min_length=2,
        description="Ordered [longitude, latitude] pairs forming a polyline.",
    )
