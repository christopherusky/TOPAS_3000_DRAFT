# ──────────────────────────────────────────────────────────────────────
# Origin:     Hand-written. Jinja snippet rendering.
# Changes:    n/a
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-05-02
# Version:    1.0
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Render Jinja2 snippet templates per language.

Each `(language, snippet_id)` pair has a template at
asyncapi_clients/templates/<language>/<snippet_id>.<ext>.j2.
"""
from __future__ import annotations

from pathlib import Path
from typing import Any

import jinja2


def render(
    *,
    templates_root: Path,
    language: str,
    snippet_id: str,
    ext: str,
    context: dict[str, Any],
) -> str:
    """Render one snippet template to a string."""
    template_path = templates_root / language / f"{snippet_id}.{ext}.j2"
    if not template_path.is_file():
        raise FileNotFoundError(template_path)
    env = jinja2.Environment(
        loader=jinja2.FileSystemLoader(templates_root / language),
        keep_trailing_newline=True,
        undefined=jinja2.StrictUndefined,
        autoescape=False,
    )
    template = env.get_template(template_path.name)
    return template.render(**context)
