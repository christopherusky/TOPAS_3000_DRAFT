# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object_access_control-v1.0.xsd (AbstractDeviceCommandGroup)
# Changes:    Enable/disable command variants; discriminated union with 'kind' field; separated command and response into distinct schemas; camelCase JSON output via Pydantic alias_generator; correlation id bounded to 2**53 - 1
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Access Control commands.

Maps to the `accessControlCommand` AsyncAPI channel. Two command
variants: enable and disable.

Field sources: utmc_data_object_access_control-v1.0.xsd
AbstractDeviceCommandGroup. The XSD defines a generic command base;
we specialise to enable/disable.
"""

from typing import Annotated, Any, Literal, Union

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp


class _AccessControlCommandBase(UTMCBaseModel):
    """Shared fields for every access control command variant."""

    access_control_id: ObjectId = Field(
        description="Identifier of the device receiving the command.",
    )
    command_correlation_id: int = Field(
        ge=0,
        le=9007199254740991,  # 2**53 - 1: largest integer a JSON number carries exactly (I-JSON)
        description=(
            "Caller-assigned identifier correlating a command request with its "
            "response. Not an identifier of the command itself. Named to avoid "
            "collision with the AMQP 1.0 correlation-id message property."
        ),
    )
    request_time: Timestamp = Field(
        description="When the command was issued.",
    )


class EnableAccessCommand(_AccessControlCommandBase):
    """Enable an access control device (raise bollard, open barrier)."""

    kind: Literal["enable"] = Field(
        default="enable",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"enable" selects the EnableAccessCommand payload; it is not free-text '
            "command content."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


class DisableAccessCommand(_AccessControlCommandBase):
    """Disable an access control device (lower bollard, close barrier)."""

    kind: Literal["disable"] = Field(
        default="disable",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"disable" selects the DisableAccessCommand payload; it is not free-text '
            "command content."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


AccessControlCommand = Annotated[
    Union[EnableAccessCommand, DisableAccessCommand],
    Field(discriminator="kind"),
]
"""Canonical payload type for the `accessControlCommand` AsyncAPI channel."""
