# ──────────────────────────────────────────────────────────────────────
# Origin:     UTMC-TS004.0064bis:2017 §3.4 (TrafficSignalConfiguration)
# XSD:        UTMCv1_xml/UTMC-TS004.0064bis-2017-traffic-signal-interface.xsd
#             complexType: TrafficSignalConfiguration
# Changes:    trafficSignalId added as payload-level key (container-object
#             flattening); extensions slot preserved (F9).
#             2026-05-07: descriptive fields restored from the 2010 profile
#             (configurationDate, subArea, userCodeNumber) — all optional
#             so the 2017 minimum contract is preserved.
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-05-07
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""TrafficSignalConfiguration — operational configuration for the 2017 profile.

All data-dict attributes are optional (0..1). The payload-level
`trafficSignalId` binds the configuration to its parent signal so the
schema is self-contained.
"""

from typing import Any

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp
from topas_3000_model.traffic_signal.enums import TrafficSignalTypeEnum


class TrafficSignalConfiguration(UTMCBaseModel):
    """Operational configuration for a traffic signal."""

    # Payload-level binder; PDF §3.2/§3.4 (implied by association from TrafficSignalStatus)
    traffic_signal_id: ObjectId = Field(
        description="Identifier of the traffic signal this configuration applies to.",
    )
    # XSD: TrafficSignalConfiguration/controllerType (String, minOccurs=0); PDF §3.4
    controller_type: str | None = Field(
        default=None,
        description="Textual description of controller type.",
    )
    # XSD: TrafficSignalConfiguration/supplier (String, minOccurs=0); PDF §3.4
    supplier: str | None = Field(
        default=None,
        description="Name of supplier.",
    )
    # XSD: TrafficSignalConfiguration/trafficSignalType (TrafficSignalTypeEnum, minOccurs=0); PDF §3.4, §4.2
    traffic_signal_type: TrafficSignalTypeEnum | None = Field(
        default=None,
        description="Type of the traffic signal.",
    )
    # 2010 carry-over (optional in 2017 profile): effectivity date + grouping/operator alias.
    configuration_date: Timestamp | None = Field(
        default=None,
        description="When this configuration record became effective.",
    )
    sub_area: str | None = Field(
        default=None,
        description="Group of signals to which this traffic signal belongs.",
    )
    user_code_number: str | None = Field(
        default=None,
        max_length=32,
        description="Operator's own reference for this traffic signal.",
    )
    # XSD: TrafficSignalConfiguration/trafficSignalConfigurationExtension (_ExtensionType, minOccurs=0) per F9
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or forward-compatibility extensions. Open-content object.",
    )
