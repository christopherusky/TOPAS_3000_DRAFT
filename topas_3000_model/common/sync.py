# ──────────────────────────────────────────────────────────────────────
# Origin:     New in TOPAS 3000A — no UTMCv1 equivalent. The CORBA
#             interface let a client call and get current state; a
#             publish-subscribe interface has to make that an explicit
#             request.
# Changes:    Request half of the sync protocol. The publish half already
#             exists in the AsyncAPI envelope (x-message-mode, x-sync-id,
#             x-sync-sequence, x-sync-total per A3/A5) and is exercised by
#             smoketest scenario S4; only the request was missing.
#             2026-10-03: SyncRequest filter lists reject duplicates; a rejected
#             SyncComplete must have object_count 0 and no sync_id; correlation
#             ids bounded to 2**53 - 1.
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Request the current state of a set of objects.

A Data Subscriber joining late, recovering from an outage longer than
Broker retention, or needing a filtered snapshot ("every active fault")
cannot reach a complete view by waiting for deltas. It sends a
``SyncRequest`` on the domain's ``sync-request`` channel, supplying an
AMQP ``reply-to``; the Data Publisher answers with an ordinary sync batch
addressed to that ``reply-to`` node, so it reaches the requester alone and
does not change what any other subscriber receives (A11, which replaced the
earlier behaviour of publishing the batch on the normal data channels).

The batch is not a new message type. Each message is the same payload
schema a delta uses, carrying ``x-message-mode: sync`` plus the
``x-sync-id`` / ``x-sync-sequence`` / ``x-sync-total`` triple from A5, so a
consumer knows the batch is complete when sequence equals total.
``x-sync-request-id`` echoes ``syncCorrelationId`` so the requester can tie
the batch to its request, and a ``SyncComplete`` closes it (A12).
"""

from enum import Enum
from typing import Any

from pydantic import Field, field_validator, model_validator

from topas_3000_model.common.base import UTMCBaseModel
from topas_3000_model.common.ids import ObjectId, TypeId
from topas_3000_model.common.time import Timestamp


class SyncCategoryEnum(str, Enum):
    """Data categories a sync may be restricted to.

    Mirrors the schema-per-category pattern: a subscriber that only holds
    a fault dashboard asks for ``fault`` alone rather than provoking a
    full republication of every definition and configuration it already
    has.
    """

    DEFINITION = "definition"
    CONFIGURATION = "configuration"
    MEASUREMENT = "measurement"
    FAULT = "fault"
    TYPE = "type"
    STATUS = "status"


class SyncRequest(UTMCBaseModel):
    """A Data Subscriber's request for current state.

    Every filter is optional and they combine with AND. A request with no
    filters asks for the complete current state of the domain, which is
    the common case on first connection.
    """

    sync_correlation_id: int = Field(
        ge=0,
        le=9007199254740991,  # 2**53 - 1: largest integer a JSON number carries exactly (I-JSON)
        description=(
            "Caller-assigned identifier correlating this request with the "
            "sync batch that answers it. The Data Publisher echoes it in "
            "the batch's x-sync-request-id header. Not an identifier of "
            "the sync itself — that is x-sync-id. Named to avoid collision "
            "with the AMQP 1.0 correlation-id message property, as for "
            "commandCorrelationId."
        ),
    )
    request_time: Timestamp = Field(
        description="When the request was issued.",
    )
    categories: list[SyncCategoryEnum] | None = Field(
        default=None,
        min_length=1,
        description=(
            "Restrict the sync to these data categories. Omitted means "
            "every category the domain publishes."
        ),
    )
    object_ids: list[ObjectId] | None = Field(
        default=None,
        min_length=1,
        description=(
            "Restrict the sync to these objects. Omitted means every "
            "object of the requested categories."
        ),
    )
    sub_area: str | None = Field(
        default=None,
        min_length=1,
        max_length=32,
        description=(
            "Restrict the sync to objects in this sub-area. Omitted means "
            "every sub-area."
        ),
    )
    type_id: TypeId | None = Field(
        default=None,
        description=(
            "Restrict the sync to objects of this type. Omitted means "
            "every type."
        ),
    )
    active_only: bool | None = Field(
        default=None,
        description=(
            "Restrict the sync to objects in an active state — for faults, "
            "those not yet cleared. Omitted is equivalent to false. This is "
            "the filter behind \"send me every active fault\": without it a "
            "subscriber would receive the full fault history it does not "
            "want."
        ),
    )
    since: Timestamp | None = Field(
        default=None,
        description=(
            "Restrict the sync to objects changed at or after this time. "
            "Intended for a subscriber recovering from a known-length "
            "outage; omitted means no lower bound."
        ),
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or forward-compatibility extensions.",
    )

    # Python-only: this rule is not in the emitted schema, so other SDKs do not enforce it.
    @field_validator("categories", "object_ids")
    @classmethod
    def _filter_values_unique(cls, v: list | None) -> list | None:
        """A repeated filter value adds nothing and is almost certainly a bug."""
        if v is not None and len(set(v)) != len(v):
            raise ValueError("filter list must not contain duplicates")
        return v


class SyncStatus(str, Enum):
    """Outcome of a sync request (A12)."""

    SUCCESS = "success"
    #: The request was understood but the publisher declined it — typically
    #: rate limiting. The subscriber may retry later.
    REJECTED = "rejected"
    #: The publisher began the batch but could not finish it. Whatever was
    #: delivered before this point is incomplete and should be discarded.
    FAILED = "failed"


class SyncComplete(UTMCBaseModel):
    """Terminal message ending a sync batch (A12).

    A5's ``x-sync-sequence``/``x-sync-total`` pair tells a subscriber when a
    batch is complete, but only once at least one message exists to carry
    them. A filtered sync that matches nothing — "every active fault", when
    there are none — would otherwise be indistinguishable from a request
    that was never answered, leaving silence as the protocol. This message
    is sent to the requester's reply node after the last object, or instead
    of any objects when none match.
    """

    sync_correlation_id: int = Field(
        ge=0,
        le=9007199254740991,  # 2**53 - 1: largest integer a JSON number carries exactly (I-JSON)
        description=(
            "Copied unchanged from the SyncRequest this completes, so a "
            "requester with several syncs in flight can tell them apart."
        ),
    )
    sync_id: str | None = Field(
        default=None,
        min_length=1,
        max_length=64,
        description=(
            "The x-sync-id shared by the messages of this batch. Normally "
            "absent when no objects matched, since there was no batch to "
            "identify. Only a rejected sync is required to omit it; a success "
            "or a failed sync may carry one (a failed sync began a batch)."
        ),
    )
    status: SyncStatus = Field(
        description="Outcome of the sync.",
    )
    object_count: int = Field(
        ge=0,
        description=(
            "Number of objects delivered in this batch. Zero is a valid "
            "answer and is the reason this message exists."
        ),
    )
    completed_time: Timestamp = Field(
        description="When the publisher finished the sync.",
    )
    message: str | None = Field(
        default=None,
        min_length=1,
        description="Human-readable explanation, for rejected or failed.",
    )
    extensions: dict[str, Any] | None = Field(
        default=None,
        description="Vendor or forward-compatibility extensions.",
    )

    # Python-only: this rule is not in the emitted schema, so other SDKs do not enforce it.
    @model_validator(mode="after")
    def _rejected_delivers_nothing(self) -> "SyncComplete":
        """A rejected sync never began a batch, so it has no batch to describe."""
        if self.status is SyncStatus.REJECTED and (
            self.object_count != 0 or self.sync_id is not None
        ):
            raise ValueError(
                "a rejected sync delivered no batch: object_count must be 0 "
                "and sync_id absent"
            )
        return self
