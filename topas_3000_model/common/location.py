# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_location-v1.0.xsd
# Changes:    Simplified from XSD (dropped OS grid references and projection metadata); WGS84 lat/lon/altitude as canonical form; shared across all domains via cross-file $ref
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Geographic primitives for UTMC objects.

Field sources: utmc_location-v1.0.xsd. Simplified from the XSD original
(which also carries OS grid references and projection metadata) — Stage 1
uses WGS84 lat/lon/altitude as the canonical form, because AMQP messages
are produced by GPS-timestamped sources and JSON consumers expect WGS84.
"""

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel


class Coordinates(UTMCBaseModel):
    """A WGS84 geographic coordinate pair (longitude, latitude)."""

    latitude: float = Field(
        ge=-90.0,
        le=90.0,
        description="Latitude in decimal degrees (WGS84).",
    )
    longitude: float = Field(
        ge=-180.0,
        le=180.0,
        description="Longitude in decimal degrees (WGS84).",
    )


class Point(UTMCBaseModel):
    """A geographic point: a coordinate plus optional altitude."""

    coordinates: Coordinates = Field(description="WGS84 coordinates.")
    altitude_meters: float | None = Field(
        default=None,
        ge=-1000.0,
        le=5000.0,
        description="Altitude in metres above mean sea level. Optional. "
                    "Bounds [-1000, 5000] inherited from utmc_location-v1.0.xsd AltitudeType.",
    )
