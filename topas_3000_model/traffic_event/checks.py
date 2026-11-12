# ──────────────────────────────────────────────────────────────────────
# Origin:     Cross-cutting; no single XSD source
# Changes:    Shared consistency checks for the four traffic event subtypes: time ordering, external cross-reference completeness and lane counts that fit the lane pattern; the version-needs-id rule is also emitted into the schema, the rest stay Python-only; not present in the XSD
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Consistency checks shared by every traffic event subtype.

Each rule rejects a payload that contradicts itself and nothing else. None
requires a field that the XSD leaves optional, so a partial report remains
valid: a rule only fires when the fields it compares are both present.

Deliberately not checked: a ``cleared`` event with no end time. The clearance
time is often unknown when an event is cleared, and requiring it would reject
the report that says so.
"""

from typing import Any, Self

from pydantic import ConfigDict, model_validator

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.traffic_event.common import LanesAffectedType

#: (start, end) field pairs. A pair is checked only for the fields a subtype has.
_TIME_PAIRS = (
    ("accident_time", "end_time"),
    ("incident_time", "end_time"),
    ("planned_start", "planned_end"),
    ("actual_start", "actual_end"),
)


def _checks_schema_extra(schema: dict[str, Any]) -> None:
    """State, in the emitted JSON Schema, the one check that can be stated there.

    The validators below run only in Python. ``dependentRequired`` carries the
    version-needs-its-id rule to the SDKs in the other languages and to any
    schema-based validator. The others stay Python-only:

    * a time that runs backwards cannot be expressed in JSON Schema;
    * the lane rules (counts bounded by the pattern's length, no restricted
      lane with ``allOpen``) can be, as ``if``/``then`` pairs, but
      ``cargo-typify`` 0.7.0 panics on ``if``/``then`` ("not supported"), so
      emitting them would break the Rust SDK.
    """
    schema["dependentRequired"] = {"externalSituationVersion": ["externalSituationId"]}


class TrafficEventChecks(UTMCBaseModel):
    """Base class adding the shared traffic event consistency checks."""

    model_config = ConfigDict(json_schema_extra=_checks_schema_extra)

    @model_validator(mode="after")
    def _times_do_not_run_backwards(self) -> Self:
        for start_name, end_name in _TIME_PAIRS:
            start = getattr(self, start_name, None)
            end = getattr(self, end_name, None)
            if start is not None and end is not None and end < start:
                raise ValueError(f"{end_name} must not be earlier than {start_name}")
        return self

    @model_validator(mode="after")
    def _external_version_needs_an_id(self) -> Self:
        version = getattr(self, "external_situation_version", None)
        if version is not None and not getattr(self, "external_situation_id", None):
            raise ValueError("external_situation_version requires a non-empty external_situation_id")
        return self

    @model_validator(mode="after")
    def _lane_counts_fit_the_pattern(self) -> Self:
        pattern = getattr(self, "lanes_affected", None)
        if pattern is None:
            return self
        restricted = getattr(self, "lanes_restricted", None)
        operational = getattr(self, "lanes_operational", None)
        if pattern is LanesAffectedType.ALL_OPEN:
            if restricted not in (None, 0):
                raise ValueError("lanes_restricted must be 0 when lanes_affected is allOpen")
            return self
        lanes = len(pattern.value)
        for name, count in (("lanes_restricted", restricted), ("lanes_operational", operational)):
            if count is not None and count > lanes:
                raise ValueError(
                    f"{name}={count} exceeds the {lanes} lane(s) in lanes_affected={pattern.value!r}"
                )
        return self
