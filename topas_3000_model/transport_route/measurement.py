# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_transport_route-v1.0.xsd (TransportRouteDynamicGroup)
# Changes:    Single flat type; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Transport Route dynamic payload.

Maps to the `transportRouteMeasurement` AsyncAPI channel.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.quality import Quality
from topas_3000_model.common.time import Timestamp
from topas_3000_model.transport_route.enums import TransportRouteStatus


class TransportRouteDynamic(UTMCBaseModel):
    """Live journey time and status for a transport route."""

    transport_route_id: ObjectId = Field(description="Identifier of the route.")
    timestamp: Timestamp = Field(description="Time of the observation.")
    quality: Quality | None = Field(default=None, description="Optional quality statement.")
    current_journey_time_seconds: float | None = Field(
        default=None, ge=0.0, description="Current journey time in seconds.",
    )
    estimated_speed_kph: float | None = Field(
        default=None, ge=0.0, description="Estimated speed along the route in km/h.",
    )
    route_status: TransportRouteStatus | None = Field(
        default=None, description="Route status (inRange/outOfRange).",
    )
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")
