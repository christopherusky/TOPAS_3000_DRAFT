# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_command-v1.0.xsd (AbstractDeviceCommandGroup)
# Changes:    Separated command and response into distinct schemas; discriminated union with 'kind' field replaces XSD generic command string; camelCase JSON output via Pydantic alias_generator; correlation id bounded to 2**53 - 1
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Transport Link commands.

Maps to the `transportLinkCommand` AsyncAPI channel. A command payload is
one of several variants keyed by the `kind` discriminator, each with its
own parameters.

Field sources: utmc_device_command-v1.0.xsd AbstractDeviceCommandGroup
and DeviceCommandResponseGroup. The legacy XSD uses generic
Command/Response strings; we specialise to a discriminated union with
typed payloads for SCOOT STOC changes and operational commands.
"""

from typing import Annotated, Any, Literal, Union

from pydantic import Field

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId
from topas_3000_model.common.time import Timestamp


class _TransportLinkCommandBase(UTMCBaseModel):
    """Shared fields for every transport link command variant."""

    transport_link_id: ObjectId = Field(
        description="Identifier of the transport link receiving the command.",
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


class SetStocCommand(_TransportLinkCommandBase):
    """Set the SCOOT STOC threshold value on a transport link."""

    kind: Literal["setStoc"] = Field(
        default="setStoc",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"setStoc" selects the SetStocCommand payload; it is not free-text '
            "command content."
        ),
    )
    stoc_value: str = Field(
        description="STOC value to set.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


class RecalibrateTransportLinkCommand(_TransportLinkCommandBase):
    """Trigger a recalibration on a transport link."""

    kind: Literal["recalibrate"] = Field(
        default="recalibrate",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"recalibrate" selects the RecalibrateTransportLinkCommand payload; it is not free-text '
            "command content."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


class RequestTransportLinkStatusCommand(_TransportLinkCommandBase):
    """Request the current status of a transport link."""

    kind: Literal["requestStatus"] = Field(
        default="requestStatus",
        description=(
            "Discriminator for the tagged union. The fixed value "
            '"requestStatus" selects the RequestTransportLinkStatusCommand payload; it is not free-text '
            "command content."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or local extensions.",
    )


TransportLinkCommand = Annotated[
    Union[SetStocCommand, RecalibrateTransportLinkCommand, RequestTransportLinkStatusCommand],
    Field(discriminator="kind"),
]
"""Canonical payload type for the `transportLinkCommand` AsyncAPI channel."""
