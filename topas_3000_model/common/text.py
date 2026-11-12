# ──────────────────────────────────────────────────────────────────────
# Origin:     utmc_data_object-v1.0.xsd (DefinitionDescriptionGroup: ShortDescription, LongDescription)
# Changes:    The XSD length restrictions (ShortDescription 1 to 32, LongDescription 1 to 2000) restored as constrained Annotated types shared by every definition; they had been lost, leaving both unbounded
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-06
# Version:    1.0
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Descriptive text primitives shared across the domains.

Field sources: utmc_data_object-v1.0.xsd ``DefinitionDescriptionGroup``.

``ShortDescription`` is a label short enough for a display or a log line, and
``LongDescription`` is the free text behind it. v1 bounds them at 32 and 2,000
characters and forbids the empty string; a definition carrying either one is
held to the same limits, so a subscriber can size a column or a database field
from the specification instead of discovering the longest value in the wild.

The constraint is the type; the meaning of the field in a given definition (a
detector's label, a sign's name) stays in that field's own description.
"""

from typing import Annotated

from pydantic import Field

ShortDescription = Annotated[str, Field(min_length=1, max_length=32)]
LongDescription = Annotated[str, Field(min_length=1, max_length=2000)]
