# ──────────────────────────────────────────────────────────────────────
# Origin:     Hand-written. Per-language file-header generation.
# Changes:    n/a
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-05-02
# Version:    1.0
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Per-language file-header generation.

File headers for the asyncapi-clients output. The author line is fixed to
the repo convention and the comment syntax is chosen per language.
"""
from __future__ import annotations

from dataclasses import dataclass
from datetime import date


@dataclass(frozen=True)
class _CommentSyntax:
    open: str
    close: str
    line_prefix: str


_SYNTAX = {
    "python": _CommentSyntax('"""', '"""', ""),
    "java": _CommentSyntax("/*", " */", " * "),
    "csharp": _CommentSyntax("/*", " */", " * "),
    "cpp": _CommentSyntax("/*", " */", " * "),
    "typescript": _CommentSyntax("/*", " */", " * "),
    "go": _CommentSyntax("/*", " */", " * "),
    "rust": _CommentSyntax("/*", " */", " * "),
    "hash": _CommentSyntax("#", "#", "# "),
}


def build_header(
    *,
    relative_path: str,
    language: str,
    gen_date: date,
    sdk_generator: str,
    generator_version: str,
) -> str:
    """Build the header block for one emitted file.

    sdk_generator is a short label naming the upstream tool that
    produced the SDK class file (e.g. 'datamodel-code-generator@0.56'),
    or 'n/a' for non-SDK files (snippets, READMEs).

    """
    if language not in _SYNTAX:
        raise ValueError(f"unknown language {language!r}")
    s = _SYNTAX[language]
    own = f"asyncapi_clients/generator v{generator_version}"
    if sdk_generator == "asyncapi_clients/generator":
        # Non-SDK file (snippet, README) — drop the redundant prefix.
        generator_field = own
    else:
        generator_field = f"{sdk_generator} + {own}"
    body_lines = [
        f"File:        {relative_path}",
        "Origin:      Generated from asyncapi.yaml by asyncapi_clients/generator/generate.py",
        "Changes:     Do not hand-edit; re-run `make asyncapi-clients` to regenerate",
        "Author:      Christopher Kettell, TRL Software",
        f"Date:        {gen_date.isoformat()}",
        f"Generator:   {generator_field}",
        "License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)",
    ]
    inner = "\n".join(s.line_prefix + line for line in body_lines)
    return f"{s.open}\n{inner}\n{s.close}\n"
