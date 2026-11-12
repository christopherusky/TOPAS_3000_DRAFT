# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_detector-v1.0.xsd (DetectorDynamicGroup and sub-groups)
# Changes:    Five-variant discriminated union with 'kind' field replaces XSD substitution groups; measurement values and periods optional as in the XSD for flow and for every period, with the single value of the occupancy, speed and headway variants kept required; the flow at-least-one rule is Python-only; class counts are sparse and non-negative; camelCase JSON output via Pydantic alias_generator; extra='forbid' enforces strict validation
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.3
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Detector measurement payloads.

Maps to the `detectorMeasurement` AsyncAPI channel. A measurement payload
is one of five variants keyed by the `kind` discriminator: flow, occupancy,
queue, speed, or headway. Each variant is a flat, self-contained object
with the common base fields (detector id, timestamp, optional quality)
plus its variant-specific measurement fields.

Field sources: utmc_quantised_detector-v1.0.xsd DetectorFlowDynamicGroup,
DetectorHeadwayDynamicGroup, DetectorOccupancyDynamicGroup, Queue inline
group, and DetectorSpeedDynamicGroup. The XSD original wraps these under
Prediction/Profile structures; we flatten to discrete measurement variants
suitable as AMQP message payloads.

Optionality. In the XSD every measurement value and every interval
is optional. A detector often reports one quantity without the others (speed
with no interval, class counts with no total), so every ``period_seconds`` is
optional and a flow reading may carry class counts without a total. The
occupancy, speed and headway variants each have a single value field, and a
reading with no value says nothing, so that field stays required: making it
optional would only have moved the rejection out of the schema and into a
Python validator that other languages' SDKs do not run. A flow reading needs
``total_flow`` or at least one class count. That rule runs in Python only: it
can be written in JSON Schema as an ``anyOf``, but ``cargo-typify`` turns each
``anyOf`` branch into a type and would reshape the Rust ``FlowDynamic``.

Status-ref fields (FlowStatusTypeRef, OccupancyStatusTypeRef, etc.) from
the XSD are dropped — they were Freeflow-extension Normal/Suspect flags
that the spec §Open items #2 earmarked for dropping unless shown to be in
the UML.
"""

from typing import Annotated, Any, Literal, Self, Union

from pydantic import Field, model_validator

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.quality import Quality
from topas_3000_model.common.time import Timestamp


class _DetectorDynamicBase(UTMCBaseModel):
    """Fields shared by every detector measurement variant."""

    detector_id: ObjectId = Field(
        description="Identifier of the detector this reading is about.",
    )
    timestamp: Timestamp = Field(
        description="Time the measurement was taken.",
    )
    quality: Quality | None = Field(
        default=None,
        description="Optional quality statement for this measurement.",
    )


class FlowDynamic(_DetectorDynamicBase):
    """Vehicle flow count over a measurement period."""

    kind: Literal["flow"] = Field(
        default="flow",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"flow" selects the FlowDynamic payload; it is not free-text '
            "command content."
        ),
    )
    total_flow: int | None = Field(
        default=None,
        ge=0,
        description="Total vehicle count over the period. Omitted when only class counts are reported.",
    )
    period_seconds: int | None = Field(
        default=None,
        gt=0,
        description="Duration of the measurement period in seconds.",
    )
    class_counts: list[Annotated[int, Field(ge=0)] | None] = Field(
        default_factory=list,
        max_length=8,
        description=(
            "Per-class vehicle counts in positional order — index 0 is Class1, "
            "index 7 is Class8. May be shorter if trailing classes are absent. "
            "A null entry means that class was not reported, as distinct from a "
            "count of zero."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )

    @model_validator(mode="after")
    def _requires_a_value(self) -> Self:
        if self.total_flow is None and all(c is None for c in self.class_counts):
            raise ValueError("a flow reading needs total_flow or at least one class count")
        return self


class OccupancyDynamic(_DetectorDynamicBase):
    """Detector occupancy (percentage of period occupied)."""

    kind: Literal["occupancy"] = Field(
        default="occupancy",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"occupancy" selects the OccupancyDynamic payload; it is not free-text '
            "command content."
        ),
    )
    occupancy_percent: float = Field(
        ge=0.0,
        le=100.0,
        description="Percentage occupancy over the period.",
    )
    period_seconds: int | None = Field(
        default=None,
        gt=0,
        description="Duration of the measurement period in seconds.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


QueueSeverity = Literal["normal", "dense", "delayed", "congested", "stopAndGo", "severe"]
"""Queue severity levels.

Mapped from the legacy XSD QueueSeverityTypeIdType:
- 0 -> normal
- 1 -> dense
- 2 -> delayed
- 3 -> congested
- 4 -> stopAndGo

The numeric mapping above and the value 'severe' are this model's own: the
XSD carries severity as a TypeId catalogue reference with no fixed value set,
so neither is sourced from it. The alias is the single source of
truth — QueueDynamic.severity references it directly.
"""


class QueueDynamic(_DetectorDynamicBase):
    """Queue presence and severity reported by the detector."""

    kind: Literal["queue"] = Field(
        default="queue",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"queue" selects the QueueDynamic payload; it is not free-text '
            "command content."
        ),
    )
    queue_present: bool = Field(description="Whether a queue is currently detected.")
    severity: QueueSeverity | None = Field(
        default=None,
        description="Queue severity when a queue is present.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


class SpeedDynamic(_DetectorDynamicBase):
    """Mean vehicle speed over a measurement period."""

    kind: Literal["speed"] = Field(
        default="speed",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"speed" selects the SpeedDynamic payload; it is not free-text '
            "command content."
        ),
    )
    mean_speed_kph: float = Field(
        ge=0.0,
        allow_inf_nan=False,
        description="Mean vehicle speed in kilometres per hour.",
    )
    period_seconds: int | None = Field(
        default=None,
        gt=0,
        description="Duration of the measurement period in seconds.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


class HeadwayDynamic(_DetectorDynamicBase):
    """Mean inter-vehicle headway over a measurement period."""

    kind: Literal["headway"] = Field(
        default="headway",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"headway" selects the HeadwayDynamic payload; it is not free-text '
            "command content."
        ),
    )
    mean_headway_seconds: float = Field(
        ge=0.0,
        allow_inf_nan=False,
        description="Mean inter-vehicle headway in seconds.",
    )
    period_seconds: int | None = Field(
        default=None,
        gt=0,
        description="Duration of the measurement period in seconds.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )


DetectorMeasurement = Annotated[
    Union[FlowDynamic, OccupancyDynamic, QueueDynamic, SpeedDynamic, HeadwayDynamic],
    Field(discriminator="kind"),
]
"""Canonical payload type for the `detectorMeasurement` AsyncAPI channel.

A discriminated union over the five measurement variants, keyed by the
`kind` property. This is the file-level $ref target when emit.py produces
detector_measurement-v1.0.json.
"""
