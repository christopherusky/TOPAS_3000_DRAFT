# ──────────────────────────────────────────────────────────────────────
# Origin:     DS003 Strategy Interface Specification v1.2 (StrategyDefinition schema)
# Changes:    WGS84 via GeoJsonPoint, dropped easting/northing; definition separate from status; camelCase JSON output via Pydantic alias_generator
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Strategy Definition.

Static metadata for a traffic management strategy. A strategy is a
coordinated set of traffic management actions (signal plans, VMS
messages, access control changes) that can be activated as a unit.

Maps to the `strategyDefinition` AsyncAPI channel.

Field sources: DS003 Strategy Interface Specification v1.2,
StrategyDefinition schema.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.geometry import GeoJsonPoint


class StrategyDefinition(UTMCBaseModel):
    """Static metadata for a traffic management strategy."""

    strategy_id: str = Field(
        min_length=1,
        description="Strategy identifier (e.g. 'STR00000001').",
    )
    strategy_name: str = Field(
        min_length=1,
        description="Human-readable name of the strategy.",
    )
    strategy_description: str | None = Field(
        default=None,
        description="Longer description of what the strategy does.",
    )
    location: GeoJsonPoint | None = Field(
        default=None,
        description="Geographic centre of the strategy area as WGS84 GeoJSON Point.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )
