# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_cctv-v1.0.xsd (CctvDynamicGroup and sub-groups)
# Changes:    6-variant discriminated union; image binary field dropped, URL only; ANPR-VRN fields carry GDPR annotation; camelCase JSON output via Pydantic alias_generator
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-08-26
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""CCTV dynamic measurement payload.

6-variant discriminated union. Image variant uses URL only.
ANPR-VRN fields carry GDPR annotation.

Maps to the `cctvMeasurement` AsyncAPI channel.
"""

from typing import Annotated, Any, Literal, Union

from pydantic import AnyUrl, Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId, TypeId
from topas_3000_model.common.quality import Quality
from topas_3000_model.common.time import Timestamp
from topas_3000_model.cctv.enums import ImageType, VehicleDirection


class _CctvDynamicBase(UTMCBaseModel):
    """Shared fields for every CCTV measurement variant."""

    cctv_id: ObjectId = Field(description="Identifier of the camera.")
    timestamp: Timestamp = Field(description="Time of the observation.")
    quality: Quality | None = Field(default=None, description="Optional quality statement.")


class CctvImageDynamic(_CctvDynamicBase):
    """A camera image capture."""

    kind: Literal["image"] = Field(
        default="image",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"image" selects the CctvImageDynamic payload; it is not free-text '
            "command content."
        ),
    )
    preset: int | None = Field(default=None, description="Camera preset number.")
    image_type: ImageType | None = Field(default=None, description="Image format.")
    image_url: AnyUrl = Field(description="URL of the captured image (no inline binary).")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")


class CctvAnprDynamic(_CctvDynamicBase):
    """Aggregate ANPR vehicle count."""

    kind: Literal["anpr"] = Field(
        default="anpr",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"anpr" selects the CctvAnprDynamic payload; it is not free-text '
            "command content."
        ),
    )
    count_period_seconds: int = Field(gt=0, description="Measurement period in seconds.")
    vehicle_count: int = Field(ge=0, description="Number of vehicles detected.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")


class CctvAnprVrnDynamic(_CctvDynamicBase):
    """Individual ANPR vehicle registration number read.

    Contains GDPR-sensitive personal data fields: vrn, tag,
    vehicle make/model/colour. Processors must handle these fields
    in accordance with data protection legislation.
    """

    kind: Literal["anprVrn"] = Field(
        default="anprVrn",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"anprVrn" selects the CctvAnprVrnDynamic payload; it is not free-text '
            "command content."
        ),
    )
    serial_number: str | None = Field(default=None, description="Camera serial number.")
    lane_id: int | None = Field(default=None, description="Lane identifier.")
    lane_direction_type_id: TypeId | None = Field(default=None, description="Lane direction type.")
    # VRN fields (GDPR-sensitive)
    vrn: str | None = Field(
        default=None,
        description="Vehicle registration number. GDPR-sensitive personal data.",
    )
    vrn_confidence: float | None = Field(
        default=None, ge=0.0, le=100.0,
        description="Confidence of VRN read (0-100%).",
    )
    tag: int | None = Field(
        default=None,
        description="Integer hash of VRN. GDPR-sensitive personal data.",
    )
    tag_confidence: float | None = Field(
        default=None, ge=0.0, le=100.0,
        description="Confidence of tag read (0-100%).",
    )
    country: str | None = Field(
        default=None,
        description="Vehicle country of registration (ISO 3166).",
    )
    # Vehicle fields
    vehicle_direction: VehicleDirection | None = Field(
        default=None, description="Direction of travel relative to camera.",
    )
    speed_kph: float | None = Field(default=None, ge=0.0, description="Vehicle speed in km/h.")
    vehicle_classification: str | None = Field(
        default=None, description="Vehicle classification (e.g. Euro6).",
    )
    vehicle_make: str | None = Field(default=None, description="Vehicle make. GDPR-sensitive.")
    vehicle_model: str | None = Field(default=None, description="Vehicle model. GDPR-sensitive.")
    vehicle_colour: str | None = Field(default=None, description="Vehicle colour.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")


class CctvBroadcastDynamic(_CctvDynamicBase):
    """Camera broadcast status."""

    kind: Literal["broadcast"] = Field(
        default="broadcast",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"broadcast" selects the CctvBroadcastDynamic payload; it is not free-text '
            "command content."
        ),
    )
    broadcast: bool = Field(description="Whether the camera is currently broadcasting.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")


class CctvFlowDynamic(_CctvDynamicBase):
    """Vehicle flow count from CCTV."""

    kind: Literal["flow"] = Field(
        default="flow",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"flow" selects the CctvFlowDynamic payload; it is not free-text '
            "command content."
        ),
    )
    count_period_seconds: int = Field(gt=0, description="Measurement period in seconds.")
    vehicle_type_id: TypeId | None = Field(default=None, description="Vehicle type category.")
    vehicle_count: int = Field(ge=0, description="Number of vehicles detected.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")


class CctvQueueDynamic(_CctvDynamicBase):
    """Queue detection from CCTV."""

    kind: Literal["queue"] = Field(
        default="queue",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"queue" selects the CctvQueueDynamic payload; it is not free-text '
            "command content."
        ),
    )
    vehicle_count: int = Field(ge=0, description="Number of vehicles in queue.")
    vehicle_type_id: TypeId | None = Field(default=None, description="Vehicle type category.")
    estimated_speed_kph: float | None = Field(default=None, ge=0.0, description="Estimated queue speed.")
    count_period_seconds: int | None = Field(default=None, gt=0, description="Measurement period.")
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")


CctvMeasurement = Annotated[
    Union[
        CctvImageDynamic,
        CctvAnprDynamic,
        CctvAnprVrnDynamic,
        CctvBroadcastDynamic,
        CctvFlowDynamic,
        CctvQueueDynamic,
    ],
    Field(discriminator="kind"),
]
"""Canonical payload type for the `cctvMeasurement` AsyncAPI channel."""
