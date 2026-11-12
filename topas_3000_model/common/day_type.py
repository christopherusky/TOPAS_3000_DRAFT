# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_support_day_type-v1.0.xsd (DayTypeDefinition, DateTypeDefinition)
# Changes:    Pydantic models for day/date type definitions; camelCase JSON output; shared across all domains via cross-file $ref
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-05
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Day Type and Date Type Definitions used for scheduling.

DayTypeDefinition classifies days (Monday, Saturday, Bank Holiday, Race Day)
so that profiles, quantised values and opening-hours can reference them by
type rather than by fixed calendar date.

DateTypeDefinition maps a specific calendar date to a day type, allowing
special dates (e.g. Easter Bank Holiday) to be categorised as a non-standard
day type.

Field sources: utmc_support_day_type-v1.0.xsd DayTypeDefinitionStructure
and DateTypeDefinitionStructure.
"""

from __future__ import annotations

import datetime
from typing import Annotated, Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import TypeId

DayTypeId = Annotated[
    int,
    Field(
        ge=1,
        le=100,
        description=(
            "Identifier of a day type. Predefined: 1=Monday, 2=Tuesday, "
            "3=Wednesday, 4=Thursday, 5=Friday, 6=Saturday, 7=Sunday. "
            "Values 8-100 available for custom day types."
        ),
    ),
]


class DayTypeDefinition(UTMCBaseModel):
    """Day type classification used for scheduling.

    Predefined day types 1-7 cover Monday to Sunday. Custom day types
    (8-100) can be added for special categories such as Bank Holidays
    or Race Days.
    """

    day_type_id: DayTypeId = Field(description="Identifier of this day type.")
    description: str = Field(description="Human-readable name, e.g. 'Monday', 'Bank Holiday'.")
    short_name: str | None = Field(
        default=None,
        description="Abbreviated name, e.g. 'Mon', 'BH'.",
    )
    pre_defined: bool | None = Field(
        default=None,
        description="Whether this is a predefined day type (1-7 for Monday to Sunday).",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


class DateTypeDefinition(UTMCBaseModel):
    """Maps a specific calendar date to a day type category.

    Used to classify special dates (e.g. 2025-04-18 as Easter Bank Holiday)
    so that profiles and schedules can apply the correct day type.
    """

    day_type_id: DayTypeId = Field(
        description="Reference to the day type this date is classified as.",
    )
    date: datetime.date = Field(
        description="The calendar date being classified.",
    )
    abnormal: bool = Field(
        description="Whether this date is an abnormal (non-standard) day.",
    )
    reason: str | None = Field(
        default=None,
        description="Reason for the abnormal classification, e.g. 'Easter Bank Holiday'.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )
