# ──────────────────────────────────────────────────────────────────────
# Origin:     Cross-cutting; no single XSD source
# Changes:    Provides UTMCBaseModel with camelCase alias_generator, extra='forbid', and serialize_by_alias=True; omits None-valued fields on output, matching the schemas' absent-means-absent contract; not present in XSD (XSD has no shared base config)
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-03
# Version:    1.2
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Base Pydantic model for all UTMC types.

Every domain class in topas_3000_model/ inherits from UTMCBaseModel, which fixes
three conventions globally:

1. Python attribute names are snake_case (PEP 8).
2. Emitted JSON property names are camelCase (JSON idiom).
3. Unknown fields are rejected at parse time (no silent drop).

The translation between snake_case and camelCase is handled by Pydantic's
alias_generator. populate_by_name=True lets callers use either form when
constructing an instance programmatically.
"""

from typing import Any

from pydantic import BaseModel, ConfigDict, SerializerFunctionWrapHandler, model_serializer
from pydantic.alias_generators import to_camel


class UTMCBaseModel(BaseModel):
    """Base class for every UTMC domain type."""

    model_config = ConfigDict(
        alias_generator=to_camel,
        populate_by_name=True,
        serialize_by_alias=True,
        extra="forbid",
    )

    @model_serializer(mode="wrap")
    def _omit_none(self, handler: SerializerFunctionWrapHandler) -> dict[str, Any]:
        """Leave an unset optional field out of the output rather than emit ``null``.

        The emitted schemas type every optional field as its value type alone,
        with no ``null`` alternative ("absent means absent"), so a ``null`` in
        the output is a schema violation. Without this, ``model_dump_json()``
        produced one for every unset optional unless the caller remembered
        ``exclude_none=True``. No field in the package is both required and
        nullable, so omitting a ``None`` never drops a value a reader needs.
        Only a model's own fields are filtered: the contents of an
        ``extensions`` dict are the caller's and pass through untouched.
        """
        return {k: v for k, v in handler(self).items() if v is not None}
