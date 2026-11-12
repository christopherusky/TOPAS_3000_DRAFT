# ──────────────────────────────────────────────────────────────────────
# Origin:     Cross-cutting scope configuration reader for TOPAS 3000A
#
# Author:     Christopher Kettell, TRL Software
# Date:       2026-10-05
# Version:    1.1
# License:    Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Read ``scope.toml``: which domains are live and which are deferred.

``bin/emit.py`` asks this module which domains to emit for a phase. A domain is
included when its phase is at or below the phase being emitted. The library
packages (``common``, ``fault_support``) are always included.

Usage: python bin/scope.py     # print the live and deferred domains
"""

from __future__ import annotations

import sys
import tomllib
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SCOPE_PATH = ROOT / "scope.toml"
MODEL_DIR = ROOT / "topas_3000_model"


class ScopeError(ValueError):
    """The scope file is malformed or disagrees with the packages on disk."""


@dataclass(frozen=True)
class Scope:
    live_phase: int
    domains: dict[str, int]
    libraries: tuple[str, ...]
    smoke: dict = field(default_factory=dict)

    def included(self, phase: int | None = None) -> list[str]:
        """Domains emitted for ``phase`` (default: the live phase), libraries first."""
        phase = self.live_phase if phase is None else phase
        live = sorted(d for d, p in self.domains.items() if p <= phase)
        return [*self.libraries, *live]

    def deferred(self, phase: int | None = None) -> list[str]:
        """Domains not emitted for ``phase`` (default: the live phase)."""
        phase = self.live_phase if phase is None else phase
        return sorted(d for d, p in self.domains.items() if p > phase)

    def is_included(self, domain: str, phase: int | None = None) -> bool:
        return domain in self.included(phase)


def load_scope(path: Path = SCOPE_PATH) -> Scope:
    """Parse and structurally validate a scope file."""
    try:
        raw = tomllib.loads(path.read_text())
    except (OSError, tomllib.TOMLDecodeError) as exc:
        raise ScopeError(f"cannot read {path}: {exc}") from exc

    live = raw.get("live_phase")
    if not isinstance(live, int) or isinstance(live, bool) or live < 1:
        raise ScopeError("live_phase must be an integer of 1 or more")
    domains = raw.get("domains")
    if not isinstance(domains, dict) or not domains:
        raise ScopeError("[domains] must list at least one domain")
    for name, phase in domains.items():
        if not isinstance(phase, int) or isinstance(phase, bool) or phase < 1:
            raise ScopeError(f"domain {name!r}: phase must be an integer of 1 or more")
    lib_table = raw.get("libraries", {})
    if not isinstance(lib_table, dict):
        raise ScopeError("[libraries] must be a table")
    libraries = lib_table.get("always", [])
    if not isinstance(libraries, list) or not all(isinstance(x, str) for x in libraries):
        raise ScopeError("[libraries] always must be a list of package names")
    overlap = set(libraries) & set(domains)
    if overlap:
        raise ScopeError(f"packages listed as both library and domain: {sorted(overlap)}")
    smoke = raw.get("smoke", {})
    if not isinstance(smoke, dict):
        raise ScopeError("[smoke] must be a table")
    _check_smoke(smoke, domains, live)
    return Scope(live, dict(domains), tuple(libraries), dict(smoke))


def _check_smoke(smoke: dict, domains: dict[str, int], live: int) -> None:
    """The smoke scenarios must name channels and fault domains that are live."""
    channel = smoke.get("sync_delta_channel")
    faults = smoke.get("fault_domains")
    if not isinstance(channel, str) or "." not in channel:
        raise ScopeError("[smoke] sync_delta_channel must be a channel name like 'detector.measurement'")
    if not isinstance(faults, list) or not faults or not all(isinstance(x, str) for x in faults):
        raise ScopeError("[smoke] fault_domains must be a non-empty list of channel domains")
    for name in (channel.split(".")[0], *faults):
        package = name.replace("-", "_")
        if package not in domains:
            raise ScopeError(f"[smoke] names {name!r}, which is not a domain in [domains]")
        if domains[package] > live:
            raise ScopeError(f"[smoke] names {name!r}, which is deferred (phase {domains[package]} > {live})")


def packages_on_disk(model_dir: Path = MODEL_DIR) -> set[str]:
    """Every package directory under ``topas_3000_model/``."""
    return {
        p.name
        for p in model_dir.iterdir()
        if p.is_dir() and not p.name.startswith("__") and (p / "__init__.py").exists()
    }


def check_against_disk(scope: Scope, model_dir: Path = MODEL_DIR) -> list[str]:
    """Problems between the scope file and the packages that exist."""
    on_disk = packages_on_disk(model_dir)
    listed = set(scope.domains) | set(scope.libraries)
    problems = [f"package {p!r} is not in scope.toml" for p in sorted(on_disk - listed)]
    problems += [f"scope.toml lists {p!r}, which is not a package" for p in sorted(listed - on_disk)]
    return problems


def main() -> int:
    try:
        scope = load_scope()
    except ScopeError as exc:
        print(f"FAIL: {exc}")
        return 1
    problems = check_against_disk(scope)
    if problems:
        print("FAIL:", *problems, sep="\n  ")
        return 1
    print(f"live phase {scope.live_phase}")
    print("emitted :", ", ".join(scope.included()))
    print("deferred:", ", ".join(scope.deferred()) or "none")
    return 0


if __name__ == "__main__":
    sys.exit(main())
