# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_car_park-v1.0.xsd (CarParkDynamicGroup)
# Changes:    Single flat type (no discriminated union); camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Car Park dynamic (live state) payload.

Maps to the `carParkMeasurement` AsyncAPI channel. Unlike
DetectorMeasurement (which is a five-way discriminated union) and
TransportLinkMeasurement (which is a two-way discriminated union), a
car park dynamic is a single flat object carrying the current
occupancy, occupancy percentage, fill/exit rates and entrance queue
time.

Field sources: utmc_quantised_car_park-v1.0.xsd CarParkDynamicGroup
(Occupancy, OccupancyPercentage, FillRate, ExitRate, QueueTime). The
XSD's Prediction/Profile wrappers and Freeflow extensions are dropped.

QueueTime is an XSD duration in the original; we normalise to
seconds-as-float for consistency with the rest of the clean model,
following the TransportLink LinkTravelTime precedent.

CarParkStateTypeRef and OccupancyTrendTypeRef are dropped — these are
references into type catalogues, which in Stage 1 live on dedicated
type channels. Car Park has no type channel in Stage 1 scope, so the
references are out of scope here; consumers derive the ALMOST_FULL /
FULL state from the raw occupancy plus the configured thresholds in
CarParkConfiguration.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.quality import Quality
from topas_3000_model.common.time import Timestamp


class CarParkDynamic(UTMCBaseModel):
    """Live operational state for a car park."""

    car_park_id: ObjectId = Field(
        description="Identifier of the car park this reading is about.",
    )
    timestamp: Timestamp = Field(
        description="Time the state was observed.",
    )
    quality: Quality | None = Field(
        default=None,
        description="Optional quality statement for this reading.",
    )
    occupancy: int | None = Field(
        default=None,
        ge=0,
        description="Number of occupied spaces.",
    )
    occupancy_percent: float | None = Field(
        default=None,
        ge=0.0,
        le=100.0,
        description="Occupancy expressed as a percentage of total capacity.",
    )
    fill_rate_vehicles_per_minute: float | None = Field(
        default=None,
        ge=0.0,
        description="Inbound vehicle rate in vehicles per minute.",
    )
    exit_rate_vehicles_per_minute: float | None = Field(
        default=None,
        ge=0.0,
        description="Outbound vehicle rate in vehicles per minute.",
    )
    queue_time_seconds: float | None = Field(
        default=None,
        ge=0.0,
        description="Estimated time, in seconds, to queue at the entrance.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )
