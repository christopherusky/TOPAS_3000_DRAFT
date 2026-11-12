# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.0
# Description: CommandResponseStatus: the full value set, that only pending is
#              non-terminal, and that notSupported is distinct from failed and
#              invalid.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.common.command."""

from topas_3000_model.common.command import CommandResponseStatus


def test_command_response_status_values():
    assert CommandResponseStatus.SUCCESS.value == "success"
    assert CommandResponseStatus.FAILED.value == "failed"
    assert CommandResponseStatus.TIMED_OUT.value == "timedOut"
    assert CommandResponseStatus.INVALID.value == "invalid"
    assert CommandResponseStatus.NOT_SUPPORTED.value == "notSupported"
    assert CommandResponseStatus.PENDING.value == "pending"


def test_only_pending_is_non_terminal():
    """Consumers test for "not pending" rather than enumerating terminal
    values, so a profile adding one does not strand them."""
    terminal = {s for s in CommandResponseStatus if s is not CommandResponseStatus.PENDING}
    assert {s.value for s in terminal} == {
        "success", "failed", "timedOut", "invalid", "notSupported",
    }


def test_not_supported_is_distinct_from_failed_and_invalid():
    """notSupported means the publisher does not implement the command;
    failed means it tried; invalid means the request was malformed."""
    assert CommandResponseStatus.NOT_SUPPORTED not in (
        CommandResponseStatus.FAILED,
        CommandResponseStatus.INVALID,
    )
