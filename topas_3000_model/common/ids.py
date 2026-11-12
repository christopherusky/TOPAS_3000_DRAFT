# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_network_base-v1.0.xsd (ObjectIdType, ParticipantIdType); utmc_support_quality-v1.0.xsd (QualityIdType); utmc_support_typeID-v1.0.xsd (TypeIdType)
# Changes:    Constrained Annotated types replace XSD simpleType restrictions; shared across all domains via cross-file $ref
#             2026-05-08: alias names (TypeId, ObjectId, ParticipantId, QualityId)
#             surfaced as `title` so they survive in the emitted JSON Schema
#             at property level. Picked up by bin/generate_diagrams.py to
#             display semantic type names ("TypeId" rather than "integer")
#             on ER diagrams. quicktype ignores property-level titles, so
#             SDK output is unaffected.
#             2026-10-03: TypeId pattern allows an embedded newline, as its
#             comment always said.
#             2026-10-06: ObjectId and ParticipantId carry the ASCII subset of the
#             XSD's NMTOKEN as a pattern, which they had lost.
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.4
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Identifier primitives shared across every UTMC domain.

Field sources:
- ObjectId:      utmc_network_base-v1.0.xsd ObjectIdType (xsd:NMTOKEN, minLength 1, maxLength 32)
- ParticipantId: utmc_network_base-v1.0.xsd ParticipantIdType (xsd:NMTOKEN, minLength 1, maxLength 32)
- QualityId:     utmc_support_quality-v1.0.xsd QualityIdType (integer)
- TypeId:        utmc_support_typeID-v1.0.xsd TypeIdType (integer)
"""

from typing import Annotated

from pydantic import Field

# v1 declares ObjectIdType and ParticipantIdType as xsd:NMTOKEN: no whitespace and
# only name characters. A JSON Schema pattern cannot portably say "any Unicode name
# character", so the ASCII subset of NMTOKEN is used: letters, digits and . _ : -
# Every identifier in use is within it, and any NMTOKEN made only of those
# characters is accepted.
_NMTOKEN_ASCII = r"^[A-Za-z0-9._:-]+$"

ObjectId = Annotated[
    str,
    Field(
        title="ObjectId",
        min_length=1,
        max_length=32,
        pattern=_NMTOKEN_ASCII,
        description=(
            "Unique identifier of an object. String of 1-32 characters from "
            "A-Z, a-z, 0-9 and . _ : - (no spaces), unique within object type "
            "within system."
        ),
    ),
]

ParticipantId = Annotated[
    str,
    Field(
        title="ParticipantId",
        min_length=1,
        max_length=32,
        pattern=_NMTOKEN_ASCII,
        description=(
            "Unique identifier of a system participating in UTMC data exchange. "
            "String of 1-32 characters from A-Z, a-z, 0-9 and . _ : - (no spaces)."
        ),
    ),
]

QualityId = Annotated[
    int,
    Field(title="QualityId", description="Identifier of a Quality Statement."),
]

TypeId = Annotated[
    str,
    Field(
        title="TypeId",
        min_length=1,
        max_length=64,
        # No leading or trailing whitespace; anything else, including an
        # embedded newline, is allowed. `[\s\S]` rather than `.`, which does
        # not match a newline.
        pattern=r"^\S(?:[\s\S]*\S)?$",
        description=(
            "Unique identifier for an object type definition, referencing a "
            "domain-specific Type object. String of 1-64 characters. "
            "Values SHOULD be stable and human-recognisable; a "
            "dot-namespaced form (e.g. 'yunex.lampFault') is RECOMMENDED "
            "where a vendor or deployment defines its own types, so they "
            "cannot collide with catalogue entries defined by this "
            "specification. The format is deliberately not constrained "
            "further."
        ),
    ),
]
