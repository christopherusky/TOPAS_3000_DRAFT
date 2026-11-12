# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/a11_idempotency.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-29
# Version:     1.1
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""A11.2 idempotency, made executable.

The isolation harness proves command *routing*. This proves the other
half: a command is executed once however many times it is attempted, and
every attempt is still answered on its own reply node.

Counters, not inference. `commandsExecuted` is instrumented in the
responder, because receiving the same answer twice cannot distinguish

    executed once and replayed correctly

from

    executed twice and coincidentally agreed.

The `disabled-store` scenario proves that distinction is live: it runs
the same duplicate with deduplication switched off and requires the run
to go red.
"""
from __future__ import annotations

import argparse
import json
import os
import re
import subprocess
import sys
import threading
import time
from dataclasses import dataclass, field
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
SMOKE = "asyncapi_clients/smoketest"
PY_EXE = sys.executable
BROKER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")

# The fixture's mapping. The marker is a lookup key established before the
# run, not an identity the sender asserts — an unmapped marker is refused.
PRINCIPAL_MAP = {"conn-a": "principal-a", "conn-b": "principal-b"}

SUMMARY_RE = re.compile(
    r"\[idempotency\] received=(?P<received>\d+) executed=(?P<executed>\d+) "
    r"produced=(?P<produced>\d+) delivered=(?P<delivered>\d+) "
    r"conflicts=(?P<conflicts>\d+)"
)
ATTEMPT_RE = re.compile(
    r"\[idem\] node=(?P<node>\S+) messageId=(?P<message_id>\S+) "
    r"statuses=(?P<statuses>\S+) misAddressed=(?P<mis_addressed>\d+) "
    r"misCorrelated=(?P<mis_correlated>\d+)"
)


@dataclass
class AttemptResult:
    label: str
    node: str | None = None
    message_id: str | None = None
    statuses: list[str] = field(default_factory=list)
    mis_addressed: int = 0
    mis_correlated: int = 0
    stdout: str = ""


def _env(extra: dict) -> dict:
    env = os.environ.copy()
    env["PYTHONPATH"] = "."
    env["TOPAS_AMQP_URL"] = BROKER
    env.update({k: str(v) for k, v in extra.items()})
    return env


def _start_responder(*, delay: float = 0.0, max_replies: int,
                     enabled: bool = True) -> subprocess.Popen:
    proc = subprocess.Popen(
        [PY_EXE, f"{SMOKE}/idempotent_responder.py"],
        cwd=str(ROOT),
        env=_env({
            "TOPAS_PRINCIPAL_MAP": json.dumps(PRINCIPAL_MAP),
            "TOPAS_RESPONDER_DELAY": delay,
            "TOPAS_RESPONDER_MAX_REPLIES": max_replies,
            "TOPAS_IDEMPOTENCY": "on" if enabled else "off",
        }),
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, bufsize=1,
    )
    proc._buf = []  # type: ignore[attr-defined]

    def pump():
        for line in proc.stdout:  # type: ignore[union-attr]
            proc._buf.append(line)  # type: ignore[attr-defined]

    t = threading.Thread(target=pump, daemon=True)
    t.start()
    proc._pump = t  # type: ignore[attr-defined]
    _await_ready(proc)
    return proc


def _await_ready(proc, timeout: float = 20.0) -> None:
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        if any(l.startswith("[responder-ready]") for l in proc._buf):
            return
        time.sleep(0.05)
    print("  [warn] responder did not signal readiness")


def _attempt(label: str, *, marker: str, correlation: int, plan: int = 4,
             expected: int = 2, originator: str = "") -> AttemptResult:
    proc = subprocess.run(
        [PY_EXE, f"{SMOKE}/idempotency_requester.py"],
        cwd=str(ROOT),
        env=_env({
            "TOPAS_CONNECTION_MARKER": marker,
            "TOPAS_COMMAND_CORRELATION_ID": correlation,
            "TOPAS_PLAN_NUMBER": plan,
            "TOPAS_EXPECTED_RESPONSES": expected,
            "TOPAS_ORIGINATOR": originator,
        }),
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=60,
    )
    result = AttemptResult(label=label, stdout=proc.stdout)
    m = ATTEMPT_RE.search(proc.stdout)
    if m:
        result.node = m.group("node")
        result.message_id = m.group("message_id")
        result.statuses = m.group("statuses").split(",")
        result.mis_addressed = int(m.group("mis_addressed"))
        result.mis_correlated = int(m.group("mis_correlated"))
    return result


def _stop(proc) -> dict:
    proc.terminate()
    try:
        proc.wait(timeout=10)
    except subprocess.TimeoutExpired:
        proc.kill()
        proc.wait(timeout=5)
    proc._pump.join(timeout=5)
    text = "".join(proc._buf)
    m = SUMMARY_RE.search(text)
    counters = {k: int(v) for k, v in m.groupdict().items()} if m else {}
    counters["_text"] = text
    return counters


def _report(name: str, attempts: list[AttemptResult], counters: dict,
            failures: list[str]) -> bool:
    print()
    for a in attempts:
        print(f"  {a.label}: node={a.node} statuses={','.join(a.statuses) or '-'} "
              f"misAddressed={a.mis_addressed} misCorrelated={a.mis_correlated}")
    print(f"  commands received:  {counters.get('received', '-')}")
    print(f"  commands executed:  {counters.get('executed', '-')}")
    print(f"  results produced:   {counters.get('produced', '-')}")
    print(f"  responses delivered:{counters.get('delivered', '-')}")
    print(f"  conflicts:          {counters.get('conflicts', '-')}")
    ok = not failures
    print(f"  result:             {'PASS' if ok else 'FAIL'}")
    for f in failures:
        print(f"    - {f}")
    return ok


def _envelope_failures(attempts: list[AttemptResult]) -> list[str]:
    """Every attempt must be answered on its own node with its own id."""
    out = []
    for a in attempts:
        if not a.statuses:
            out.append(f"{a.label} received nothing")
        if a.mis_addressed:
            out.append(f"{a.label}: {a.mis_addressed} response(s) on another node")
        if a.mis_correlated:
            out.append(f"{a.label}: {a.mis_correlated} response(s) with another "
                       "correlation-id — a stored envelope was replayed")
    nodes = [a.node for a in attempts if a.node]
    if len(set(nodes)) != len(nodes):
        out.append("attempts shared a reply node")
    return out


def scenario_duplicate_in_progress() -> bool:
    """A retry that lands while the first attempt is still executing.

    The valuable case: the retry has a different message-id and a
    different reply node, so the terminal result must reach a node the
    responder learned about *after* it began executing.
    """
    print("=== A11.2: duplicate while in progress ===")
    responder = _start_responder(delay=3.0, max_replies=4)
    results: list[AttemptResult] = []
    threads = []
    for label in ("attempt-1", "attempt-2"):
        t = threading.Thread(
            target=lambda l=label: results.append(
                _attempt(l, marker="conn-a", correlation=7777, expected=2)
            )
        )
        threads.append(t)
        t.start()
        time.sleep(1.0)  # the second lands inside the responder's delay
    for t in threads:
        t.join(timeout=60)
    counters = _stop(responder)

    failures = _envelope_failures(results)
    if counters.get("executed") != 1:
        failures.append(f"executed {counters.get('executed')} times, expected 1")
    for a in results:
        if "success" not in a.statuses:
            failures.append(f"{a.label} never received the terminal result")
    return _report("duplicate-in-progress", results, counters, failures)


def scenario_duplicate_completed() -> bool:
    """A retry after the command has already finished.

    The stored *result* is replayed; the transport envelope is rebuilt for
    the new attempt.
    """
    print("=== A11.2: duplicate after completion ===")
    responder = _start_responder(max_replies=3)
    first = _attempt("attempt-1", marker="conn-a", correlation=7777, expected=2)
    second = _attempt("attempt-2", marker="conn-a", correlation=7777, expected=1)
    counters = _stop(responder)

    failures = _envelope_failures([first, second])
    if counters.get("executed") != 1:
        failures.append(f"executed {counters.get('executed')} times, expected 1")
    if second.statuses != ["success"]:
        failures.append(f"retry got {second.statuses}, expected the stored success")
    return _report("duplicate-completed", [first, second], counters, failures)


def scenario_conflicting_command() -> bool:
    """Same key, materially different command."""
    print("=== A11.2: conflicting command on the same key ===")
    responder = _start_responder(max_replies=3)
    first = _attempt("setPlan-4", marker="conn-a", correlation=7777, plan=4,
                     expected=2)
    second = _attempt("setPlan-7", marker="conn-a", correlation=7777, plan=7,
                      expected=1)
    counters = _stop(responder)

    failures = _envelope_failures([first, second])
    if counters.get("executed") != 1:
        failures.append(f"executed {counters.get('executed')} times, expected 1")
    if second.statuses != ["invalid"]:
        # Returning the plan-4 result here would report success for a
        # command that was never run.
        failures.append(f"conflict got {second.statuses}, expected rejection")
    if counters.get("conflicts") != 1:
        failures.append("conflict was not recorded")
    return _report("conflicting-command", [first, second], counters, failures)


def scenario_principal_namespace() -> bool:
    """The same commandCorrelationId from two principals is two commands."""
    print("=== A11.2: commandCorrelationId is per-principal ===")
    responder = _start_responder(max_replies=4)
    a = _attempt("principal-a", marker="conn-a", correlation=7777, expected=2)
    b = _attempt("principal-b", marker="conn-b", correlation=7777, expected=2)
    counters = _stop(responder)

    failures = _envelope_failures([a, b])
    if counters.get("executed") != 2:
        failures.append(
            f"executed {counters.get('executed')} times, expected 2 — "
            "commandCorrelationId was treated as globally unique"
        )
    return _report("principal-namespace", [a, b], counters, failures)


def scenario_spoofed_originator() -> bool:
    """`x-originator` must not move a command between namespaces.

    Both attempts authenticate as principal-a; the second claims to be
    principal-b in the self-asserted audit field. If the responder keyed
    on that claim it would execute twice.
    """
    print("=== A11.2: x-originator does not select the namespace ===")
    responder = _start_responder(max_replies=3)
    honest = _attempt("honest", marker="conn-a", correlation=7777, expected=2)
    spoofed = _attempt("spoofed", marker="conn-a", correlation=7777, expected=1,
                       originator="principal-b")
    counters = _stop(responder)

    failures = _envelope_failures([honest, spoofed])
    if counters.get("executed") != 1:
        failures.append(
            f"executed {counters.get('executed')} times, expected 1 — "
            "a self-asserted x-originator changed the idempotency key"
        )
    return _report("spoofed-originator", [honest, spoofed], counters, failures)


def scenario_disabled_store() -> bool:
    """Prove the executed counter can move.

    With deduplication off the same duplicate must execute twice. A suite
    that cannot show this cannot claim the other scenarios mean anything.
    """
    print("=== A11.2: negative self-test — store disabled ===")
    responder = _start_responder(max_replies=4, enabled=False)
    first = _attempt("attempt-1", marker="conn-a", correlation=7777, expected=2)
    second = _attempt("attempt-2", marker="conn-a", correlation=7777, expected=2)
    counters = _stop(responder)

    failures = []
    if counters.get("executed") != 2:
        failures.append(
            f"executed {counters.get('executed')} times with the store "
            "disabled, expected 2 — the counter is not measuring execution"
        )
    print("  note: a duplicate execution here is the expected outcome; it is")
    print("        what shows the other scenarios are asserting something")
    return _report("disabled-store", [first, second], counters, failures)


SCENARIOS = {
    "duplicate-in-progress": scenario_duplicate_in_progress,
    "duplicate-completed": scenario_duplicate_completed,
    "conflicting-command": scenario_conflicting_command,
    "principal-namespace": scenario_principal_namespace,
    "spoofed-originator": scenario_spoofed_originator,
    "disabled-store": scenario_disabled_store,
}


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--only", choices=sorted(SCENARIOS))
    args = ap.parse_args()

    print(f"[a11] broker = {BROKER}")
    names = [args.only] if args.only else list(SCENARIOS)
    results = {name: SCENARIOS[name]() for name in names}
    print()
    print("=== SUMMARY ===")
    for name, ok in results.items():
        print(f"  {'PASS' if ok else 'FAIL'}  {name}")
    ok = all(results.values())
    print(f"\nA11 IDEMPOTENCY: {'PASS' if ok else 'FAIL'}")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
