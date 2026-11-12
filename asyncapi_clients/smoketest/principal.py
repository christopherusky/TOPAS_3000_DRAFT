# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/principal.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-29
# Version:     1.1
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Where a command's authenticated principal comes from.

A11.2 keys idempotency on the **authenticated principal** together with
`commandCorrelationId`, and says specifically that `x-originator` must
not be used: it is self-asserted application data and cannot bound
anything. That makes "who is the principal" a security boundary rather
than a detail, so it gets an explicit seam instead of being inlined into
the responder.

    AuthenticatedPrincipalProvider
            |
            +-- BrokerValidatedUserProvider   production
            |     the identity the broker authenticated
            |
            +-- ConformancePrincipalProvider  this harness
                  a principal the fixture supplied, by prior arrangement

The distinction that matters:

    WRONG:              principal = message.properties["x-originator"]
    WRONG:              principal = remote container id
    OK for conformance: principal = trusted_fixture[connection marker]
    Production:         principal = broker-validated user

In the conformance case the marker is not an asserted identity. It is a
lookup key into a mapping the test fixture established before the run,
and a marker that is not in that mapping is **refused** rather than
trusted — which is what keeps this from being `x-originator` by another
name.
"""
from __future__ import annotations

from typing import Protocol


class PrincipalUnavailable(Exception):
    """No authenticated principal could be established for a command.

    Never fall back to something self-asserted. A11.2's idempotency
    guarantee is only as good as the identity it is keyed on, so a command
    whose principal cannot be established must be refused.
    """


class AuthenticatedPrincipalProvider(Protocol):
    """Resolves the authenticated principal responsible for a command."""

    def principal_for(self, message) -> str:  # pragma: no cover - protocol
        ...


class BrokerValidatedUserProvider:
    """The production shape: the identity the broker authenticated.

    Artemis stamps the authenticated user onto each message as
    `_AMQ_VALIDATED_USER` when `<populate-validated-user>true` is set, and
    a client cannot forge it — the broker overwrites whatever arrived.
    Other brokers expose an equivalent; the point is that the value is
    asserted by the authenticating party, not by the sender.

    **This does not work on the reference broker, and that is a finding
    rather than a to-do.** It was once recorded as unexercised plumbing;
    running it against an authenticated Artemis 2.31.2 showed something
    worse:

    * `populate-validated-user` is enabled and no validated user reaches
      an AMQP consumer — not in application properties, message
      annotations, delivery annotations, or anywhere else;
    * the AMQP `user-id` field arrives exactly as the sender set it. A
      client authenticated as `user-a` can send `user-id` of
      `principal-b` and it is delivered unmodified.

    So on that broker an AMQP consumer has **no** identity it did not take
    from the sender, and this provider would find nothing to read. It is
    kept because the shape is right and other brokers, other protocol
    hops (Artemis converts to core for a core consumer, which is where
    `_AMQ_VALIDATED_USER` and `JMSXUserID` appear) and mTLS-terminating
    deployments do supply one. `a11_authenticated.py` asserts the current
    behaviour, so if it ever changes the suite goes red and this decision
    gets re-read.
    """

    VALIDATED_USER_PROPERTY = "_AMQ_VALIDATED_USER"

    def principal_for(self, message) -> str:
        properties = getattr(message, "properties", None) or {}
        principal = properties.get(self.VALIDATED_USER_PROPERTY)
        if not principal:
            raise PrincipalUnavailable(
                "no broker-validated user on the command; refusing rather "
                "than falling back to self-asserted data"
            )
        return str(principal)


class ConformancePrincipalProvider:
    """A principal the test fixture supplied, by prior arrangement.

    Deliberately not called a mock: it stands in for a real security
    boundary, and naming it as a mock would invite someone to treat the
    boundary as incidental.

    The fixture starts each requester with a connection marker and tells
    this provider which principal that marker denotes. The marker is a
    lookup key, not a claim — an unknown one raises.
    """

    MARKER_PROPERTY = "x-conformance-connection"

    def __init__(self, mapping: dict[str, str]) -> None:
        self._mapping = dict(mapping)

    def principal_for(self, message) -> str:
        properties = getattr(message, "properties", None) or {}
        marker = properties.get(self.MARKER_PROPERTY)
        if marker is None:
            raise PrincipalUnavailable("command carried no conformance marker")
        try:
            return self._mapping[str(marker)]
        except KeyError:
            # Refused, not trusted. If an unrecognised marker were
            # accepted at face value this would be `x-originator` wearing
            # a different name.
            raise PrincipalUnavailable(
                f"marker {marker!r} is not in the fixture's mapping"
            ) from None
