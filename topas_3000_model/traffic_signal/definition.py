# ──────────────────────────────────────────────────────────────────────
# Origin:     UTMC-TS004.0064bis:2017 §3.2 (TrafficSignalStatus identity + Point)
# XSD:        UTMCv1_xml/UTMC-TS004.0064bis-2017-traffic-signal-interface.xsd
#             complexType: TrafficSignalStatus (identifier + location slice)
# Changes:    DATEX II Point wrapper (TPEG/AlertC/linear-referencing/coordinates); ShortDescription (1 to 32) and LongDescription (1 to 2000) limits restored
#             collapsed to GeoJsonPoint per F1 + F7; DATEX II publication
#             envelope dropped (F1); extensions slot preserved (F9).
#             2026-05-07: descriptive + provenance fields restored from the
#             2010 profile (creationDate, shortDescription,
#             longDescription, participantId, originatingParticipantId,
#             originatingObjectId) — all optional so the 2017 minimum
#             contract is preserved. typeId remains excluded (its 2017
#             home is TrafficSignalConfiguration.trafficSignalType).
#             2026-08-29: historicDate removed with every other domain
# — a Freeflow XSD soft-delete extension, not core UTMC.
#             2026-05-07 (later): controlMethodId added as an optional
#             reference into the TrafficSignalType catalogue. Distinct
#             axis from TrafficSignalConfiguration.trafficSignalType:
#             trafficSignalType (closed enum) classifies the junction
#             *kind* (intersection / pelican / puffin / ...);
#             controlMethodId (open catalogue) identifies the *control
#             method* running it (MOVA / SCOOT / fixed-time / ...).
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.4
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""TrafficSignalDefinition — identity slice of the 2017 TrafficSignalStatus.

The 2017 UML folds identity (`trafficSignalIdentifier`) and location
(`Point [0..1]`) into the `TrafficSignalStatus` container, alongside
Configuration/DynamicState/Fault. We extract identity + location into a
standalone `TrafficSignalDefinition` payload so it maps to the
`trafficSignalDefinition` AsyncAPI channel, consistent with every
other domain in the project.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.geometry import GeoJsonPoint
from topas_3000_model.common.ids import ObjectId, ParticipantId, TypeId
from topas_3000_model.common.time import Timestamp
from topas_3000_model.common.text import LongDescription, ShortDescription


class TrafficSignalDefinition(UTMCBaseModel):
    """Static identity + location for a traffic signal."""

    # XSD: TrafficSignalStatus/trafficSignalIdentifier (String, minOccurs=1); PDF §3.2
    traffic_signal_id: ObjectId = Field(
        description="Human readable identifier for the Traffic Signal.",
    )
    # XSD: TrafficSignalStatus/point (Point, minOccurs=0) — flattened to GeoJsonPoint per F7; PDF §3.2, §2 UML
    location: GeoJsonPoint | None = Field(
        default=None,
        description="Location of traffic signal.",
    )
    # Reference into the TrafficSignalType catalogue identifying the
    # *control method* (MOVA / SCOOT / fixed-time / vehicle-actuated /
    # ...). Orthogonal to TrafficSignalConfiguration.trafficSignalType
    # (closed enum), which classifies the junction kind.
    control_method_id: TypeId | None = Field(
        default=None,
        description=(
            "Optional reference into the TrafficSignalType catalogue identifying "
            "the operational control method (MOVA, SCOOT, fixed-time, ...). "
            "Orthogonal to TrafficSignalConfiguration.trafficSignalType, which "
            "classifies the junction kind (intersection, pelican, puffin, ...)."
        ),
    )
    # 2010 carry-over (optional in 2017 profile): provenance + descriptive fields.
    creation_date: Timestamp | None = Field(
        default=None,
        description="When this definition record was created.",
    )
    short_description: ShortDescription | None = Field(
        default=None,
        description="Short human-readable name (e.g. 'Main St / High St junction').",
    )
    long_description: LongDescription | None = Field(
        default=None,
        description="Longer description (context, commissioning notes, etc.).",
    )
    participant_id: ParticipantId | None = Field(
        default=None,
        description="System that owns this traffic signal definition. Often implicit from context.",
    )
    originating_participant_id: ParticipantId | None = Field(
        default=None,
        description=(
            "Identifier of the upstream system that originally created this object "
            "(comparable to the UML DataSourceTypeId)."
        ),
    )
    originating_object_id: ObjectId | None = Field(
        default=None,
        description=(
            "Identifier of this object on the originating (upstream) system. "
            "Enables cross-system traceability."
        ),
    )
    # XSD: TrafficSignalStatus/trafficSignalStatusExtension (_ExtensionType, minOccurs=0) per F9
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or forward-compatibility extensions. Open-content object.",
    )
