# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_vms-v1.0.xsd (VmsCommandGroup)
# Changes:    4-variant discriminated union (DisplayMessage, SetBrightness, ClearSign, etc.); separated command and response into distinct schemas; camelCase JSON output via Pydantic alias_generator; graphics are named by URL only, GraphicToDisplay by name or reference is not carried; correlation id bounded to 2**53 - 1
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""VMS commands.

Maps to the `vmsCommand` AsyncAPI channel. Four command variants.
"""

from typing import Annotated, Any, Literal, Union

from pydantic import AnyUrl, Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp


class _VmsCommandBase(UTMCBaseModel):
    """Shared fields for every VMS command variant."""

    vms_id: ObjectId = Field(description="Identifier of the VMS receiving the command.")
    command_correlation_id: int = Field(
        ge=0,
        le=9007199254740991,  # 2**53 - 1: largest integer a JSON number carries exactly (I-JSON)
        description="Caller-assigned identifier.",
    )
    request_time: Timestamp = Field(description="When the command was issued.")


class DisplayMessageCommand(_VmsCommandBase):
    """Display a text message on the sign."""

    kind: Literal["displayMessage"] = Field(
        default="displayMessage",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"displayMessage" selects the DisplayMessageCommand payload; it is not free-text '
            "command content."
        ),
    )
    message: str = Field(description="Message text to display.")
    message_lines: list[str] | None = Field(default=None, description="Message split by lines.")
    display_duration_seconds: int | None = Field(
        default=None, gt=0, description="How long to display the message.",
    )
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")


class DisplayGraphicCommand(_VmsCommandBase):
    """Display a graphic on the sign."""

    kind: Literal["displayGraphic"] = Field(
        default="displayGraphic",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"displayGraphic" selects the DisplayGraphicCommand payload; it is not free-text '
            "command content."
        ),
    )
    graphic_url: AnyUrl = Field(description="URL of the graphic to display.")
    display_duration_seconds: int | None = Field(
        default=None, gt=0, description="How long to display the graphic.",
    )
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")


class SetBrightnessCommand(_VmsCommandBase):
    """Set the brightness of the sign."""

    kind: Literal["setBrightness"] = Field(
        default="setBrightness",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"setBrightness" selects the SetBrightnessCommand payload; it is not free-text '
            "command content."
        ),
    )
    brightness_percent: float = Field(
        ge=0.0, le=100.0, description="Brightness percentage to set.",
    )
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")


class ClearSignCommand(_VmsCommandBase):
    """Clear the sign (blank the display)."""

    kind: Literal["clearSign"] = Field(
        default="clearSign",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"clearSign" selects the ClearSignCommand payload; it is not free-text '
            "command content."
        ),
    )
    extensions: dict[str, Any] | None = Field(default=None, description="Vendor or local extensions.")


VmsCommand = Annotated[
    Union[DisplayMessageCommand, DisplayGraphicCommand, SetBrightnessCommand, ClearSignCommand],
    Field(discriminator="kind"),
]
"""Canonical payload type for the `vmsCommand` AsyncAPI channel."""
