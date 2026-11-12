# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_device_command-v1.0.xsd (DeviceCommandResponseGroup)
# Changes:    CommandResponseStatus is a TOPAS enum informed by, not copied from, the XSD ResponseResult (1 Invalid, 2 Valid, 3 Implemented, 4 Timed out): invalid, pending (accepted, interim), success and timedOut correspond; failed and notSupported are additions; separated command/response primitives; shared across all domains via cross-file $ref
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-05
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Shared command/response primitives.

CommandResponseStatus is the canonical enum for command outcomes, used by
every domain's command response model.
"""

from enum import Enum


class CommandResponseStatus(str, Enum):
    """Outcome of a command execution.

    Every value except ``PENDING`` is terminal. Consumers SHOULD test for
    "not pending" rather than enumerating the terminal values, so that a
    future profile adding one does not strand them.
    """

    SUCCESS = "success"
    FAILED = "failed"
    TIMED_OUT = "timedOut"
    INVALID = "invalid"
    #: The command is well-formed and understood, but this Data Publisher
    #: does not implement it. Distinct from `failed`, which means the
    #: publisher tried; and from `invalid`, which means the request was
    #: malformed. A publisher that answers `notSupported` is conformant —
    #: not every command in a domain's union is mandatory.
    NOT_SUPPORTED = "notSupported"
    PENDING = "pending"
