# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_cctv-v1.0.xsd (various enumeration types)
# Changes:    String enum values replace XSD integer codes; inline in domain module
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-04-23
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""CCTV domain-specific enumerations (inline)."""

from enum import Enum


class ImageType(str, Enum):
    """Image format type."""

    JPEG = "jpeg"
    PNG = "png"
    BMP = "bmp"
    GIF = "gif"
    TIFF = "tiff"
    OTHER = "other"


class VehicleDirection(str, Enum):
    """Direction of vehicle travel relative to the camera."""

    TOWARDS = "towards"
    AWAY = "away"
