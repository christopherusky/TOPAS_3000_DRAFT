# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# Description: RecordStatus: its value set and parsing.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for topas_3000_model.common.status."""

from topas_3000_model.common.status import RecordStatus


def test_record_status_values():
    assert RecordStatus.ACTIVE.value == "active"
    assert RecordStatus.CLEARED.value == "cleared"
    assert RecordStatus.HISTORIC.value == "historic"


def test_record_status_parse():
    assert RecordStatus("cleared") == RecordStatus.CLEARED
