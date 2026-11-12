# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_support_quality-v1.0.xsd (AbstractQualityGroup, AbstractQualityStructure)
# Changes:    Flattened Group+Structure into single class; shared across all domains via cross-file $ref; DataSourceTypeRef not carried, source_name required and non-empty; Quality sits on measurements only, and v1's definition QualityRef and Reality are not carried
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.3
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Data Quality Statement used across all UTMC domains.

Field sources: utmc_support_quality-v1.0.xsd AbstractQualityGroup and
AbstractQualityStructure, flattened. The XSD original factors this across
a Group and a Structure for XSD reuse — we collapse the two into a single
clean class.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ParticipantId, QualityId
from topas_3000_model.common.time import Timestamp


class Quality(UTMCBaseModel):
    """Quality statement attached to a UTMC data object or measurement."""

    quality_id: QualityId = Field(description="Identifier of this Quality statement.")
    source_name: str = Field(
        min_length=1,
        description="Name of the data source (e.g. 'SCOOT', 'MIDAS'). Required: a quality statement naming no source cannot be interpreted.",
    )
    source_type: str | None = Field(
        default=None,
        description="Free-text description of the source kind.",
    )
    participant_id: ParticipantId | None = Field(
        default=None,
        description="Identifier of the UTMC participant system that owns this quality statement.",
    )
    last_updated: Timestamp | None = Field(
        default=None,
        description="When this quality statement was last updated.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions. Open-content object.",
    )
