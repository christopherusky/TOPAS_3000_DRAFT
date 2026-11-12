# ──────────────────────────────────────────────────────────────────────
# Origin:     UTMC-TS004.0064bis:2017 §3.3 (TrafficSignalFault) +
#             DATEX II Fault base class (inherited, flattened per F6)
# XSD:        UTMCv1_xml/UTMC-TS004.0064bis-2017-traffic-signal-interface.xsd
#             complexTypes: TrafficSignalFault, Fault (base)
# Changes:    DATEX II Fault base-class fields flattened onto TrafficSignalFault
#             (F6); faultLastUpdateTime is the only inherited required field;
#             two XSD extension slots (trafficSignalFaultExtension +
#             inherited faultExtension) collapse to a single `extensions`
#             field per F6.
#             2026-10-06: faultTypeId optional; at least one of faultTypeId
#             and trafficSignalFaultType is required, so a fault written to
#             the 2017 profile alone is accepted (review B12 c).
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""TrafficSignalFault — fault record for the 2017 profile.

In the DATEX II XSD TrafficSignalFault extends a `Fault` base class that
carries faultIdentifier, faultDescription, faultCreationTime,
faultLastUpdateTime (required), and faultSeverity. Per F6 those
inherited fields are flattened onto this class.

A fault is classified by `faultTypeId` (the catalogue), by the 2017
profile's `trafficSignalFaultType`, or by both, and at least one of the two
must be present. The rule is also emitted into the JSON Schema, so every
SDK and schema validator applies it, not only Python.
"""

from typing import Any, Self

from pydantic import ConfigDict, Field, model_validator

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId, TypeId
from topas_3000_model.common.time import Timestamp
from topas_3000_model.traffic_signal.enums import (
    FaultSeverityEnum,
    TrafficSignalFaultEnum,
    UrgencyEnum,
)


def _one_classification_required(schema: dict[str, Any]) -> None:
    """At least one of faultTypeId and trafficSignalFaultType.

    ``anyOf`` with two ``required`` clauses, not ``if``/``then``: cargo-typify
    0.7.0 rejects ``if``/``then`` (see traffic_event/checks.py).
    """
    schema["anyOf"] = [{"required": ["faultTypeId"]}, {"required": ["trafficSignalFaultType"]}]


class TrafficSignalFault(UTMCBaseModel):
    """A fault record for a traffic signal."""

    model_config = ConfigDict(json_schema_extra=_one_classification_required)

    # Payload-level binder; PDF §3.3 (implied by association from TrafficSignalStatus)
    traffic_signal_id: ObjectId = Field(
        description="Identifier of the traffic signal this fault applies to.",
    )
    # Catalogue reference. traffic_signal was the only domain still
    # classifying faults by a closed enum; every other one references a
    # FaultTypeDefinition by id. That enum buckets 36 of the 46 published
    # signal fault codes into `other`, which is why timing faults such as a
    # stuck stage could not be expressed at all.
    # Optional since the 2017 profile was accepted: a 2017-profile publisher may classify by
    # trafficSignalFaultType alone. At least one of the two is required.
    fault_type_id: TypeId | None = Field(
        default=None,
        description=(
            "Reference to a FaultTypeDefinition entry. The catalogue carries "
            "the code, category and message template; new fault types are "
            "added as catalogue entries, with no schema change. Required "
            "unless trafficSignalFaultType is given; when both are present "
            "this is the authoritative classification."
        ),
    )
    # XSD: TrafficSignalFault/trafficSignalFaultType (enum, minOccurs=1); PDF §3.3, §4.1
    # Relaxed to optional: retained as the coarse DATEX II bucket for
    # interoperability, superseded by faultTypeId for classification. It
    # may be the only classification, as in the 2017 profile.
    traffic_signal_fault_type: TrafficSignalFaultEnum | None = Field(
        default=None,
        description=(
            "Coarse DATEX II fault bucket, as in TS004.0064bis:2017. Required "
            "unless faultTypeId is given; when both are present faultTypeId "
            "is the authoritative classification."
        ),
    )
    # XSD: Fault/faultLastUpdateTime (DateTime, minOccurs=1) — flattened per F6; PDF §3.3 base
    fault_last_update_time: Timestamp = Field(
        description="Time fault was last updated.",
    )
    # XSD: Fault/faultIdentifier (String, minOccurs=0) — flattened per F6; PDF §3.3 base
    fault_identifier: str | None = Field(
        default=None,
        description="Unique identifier for this fault.",
    )
    # XSD: Fault/faultDescription (String, minOccurs=0) — flattened per F6; PDF §3.3 base
    fault_description: str | None = Field(
        default=None,
        description="Human-readable fault description.",
    )
    # XSD: Fault/faultCreationTime (DateTime, minOccurs=0) — flattened per F6; PDF §3.3 base
    fault_creation_time: Timestamp | None = Field(
        default=None,
        description="Time the fault was first recorded.",
    )
    # XSD: Fault/faultSeverity (FaultSeverityEnum, minOccurs=0) — flattened per F6; PDF §3.3 base
    fault_severity: FaultSeverityEnum | None = Field(
        default=None,
        description="Severity of the fault.",
    )
    # XSD: TrafficSignalFault/trafficSignalFaultClearanceTime (DateTime, minOccurs=0); PDF §3.3
    traffic_signal_fault_clearance_time: Timestamp | None = Field(
        default=None,
        description="Time fault was cleared.",
    )
    # XSD: TrafficSignalFault/trafficSignalFaultUrgency (UrgencyEnum, minOccurs=0); PDF §3.3
    traffic_signal_fault_urgency: UrgencyEnum | None = Field(
        default=None,
        description="The urgency of this fault.",
    )
    # XSD: TrafficSignalFault/trafficSignalManufacturerFaultCode (String, minOccurs=0); PDF §3.3
    traffic_signal_manufacturer_fault_code: str | None = Field(
        default=None,
        description="A supplier unique identifier for this specific fault.",
    )
    # XSD: TrafficSignalFault/trafficSignalFaultExtension + inherited Fault/faultExtension
    #      (two _ExtensionType slots collapsed per F6)
    # Values for the placeholders in the catalogue entry's messageTemplate
    # Positional and ordered; a consumer with the catalogue can render
    # the message, one without it still has the raw values.
    parameters: list[str] | None = Field(
        default=None,
        min_length=1,
        description=(
            "Values for the positional placeholders in the referenced "
            "FaultTypeDefinition's messageTemplate, in order. Rendered as "
            "text so a value carries whatever form the fault reports it in."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or forward-compatibility extensions. Open-content object.",
    )

    @model_validator(mode="after")
    def _classified(self) -> Self:
        if self.fault_type_id is None and self.traffic_signal_fault_type is None:
            raise ValueError("a traffic signal fault needs faultTypeId, trafficSignalFaultType or both")
        return self
