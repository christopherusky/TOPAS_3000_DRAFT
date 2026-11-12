# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/a11_matrix.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Run every requester against every responder.

The per-language gate proves each implementation satisfies A11 *against
itself* — in practice, against the Python responder it was written
alongside. That is weaker than it looks: a requester and a responder
written in the same session tend to share assumptions, and a shared
assumption is invisible to a test where both halves make it.

Pairing every language with every other is what exposes them. The first
run of this matrix found three, none of which any same-language test
could have caught:

  * Qpid JMS infers a *destination type* for a bare `reply-to` and got it
    wrong, sending responses to a multicast address nobody consumed.
  * The Python requester decoded only an AMQP value section, so a
    responder sending a data section crashed it.
  * AMQPNetLite exposes AMQP identifiers through both a lossy `string`
    property and a union-preserving accessor; the requester was reading
    the lossy one.

The diagonal is included deliberately. It is the control: those cells
are the per-language gate, so a diagonal failure means the harness
itself is wrong rather than the pairing.

`two-requesters` is the scenario used, because it exercises the whole
contract in one run — dynamic reply node, reply-to addressing,
correlation mirroring, commandCorrelationId carried through, `to`
present, isolation between two concurrent requesters, and a
pending->success progression.
"""
from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
HARNESS = "asyncapi_clients/smoketest/a11_isolation.py"
LANGUAGES = ["python", "java", "csharp", "typescript", "rust"]


def run_pair(requester: str, responder: str, scenario: str) -> tuple[bool, str]:
    proc = subprocess.run(
        [sys.executable, HARNESS, "--lang", requester,
         "--responder", responder, "--only", scenario],
        cwd=str(ROOT), stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
        text=True, timeout=180,
    )
    return proc.returncode == 0, proc.stdout


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--scenario", default="two-requesters",
                    help="which A11 scenario to run for each pairing")
    ap.add_argument("--requester", choices=LANGUAGES, action="append",
                    help="limit to these requesters (repeatable)")
    ap.add_argument("--responder", choices=LANGUAGES, action="append",
                    help="limit to these responders (repeatable)")
    args = ap.parse_args()

    requesters = args.requester or LANGUAGES
    responders = args.responder or LANGUAGES

    print(f"[matrix] scenario = {args.scenario}")
    print(f"[matrix] {len(requesters)} requesters x {len(responders)} responders "
          f"= {len(requesters) * len(responders)} pairings")
    print()

    results: dict[tuple[str, str], bool] = {}
    failures: list[tuple[str, str, str]] = []
    for requester in requesters:
        for responder in responders:
            ok, output = run_pair(requester, responder, args.scenario)
            results[(requester, responder)] = ok
            marker = "PASS" if ok else "FAIL"
            diagonal = " (control)" if requester == responder else ""
            print(f"  {marker}  {requester:<11} -> {responder}{diagonal}", flush=True)
            if not ok:
                failures.append((requester, responder, output))

    width = max(len(l) for l in LANGUAGES) + 2
    print()
    print("=== A11 CROSS-LANGUAGE MATRIX ===")
    print("rows = requester, columns = responder")
    print()
    header = " " * width + "".join(f"{r[:6]:<8}" for r in responders)
    print(header)
    for requester in requesters:
        row = f"{requester:<{width}}"
        for responder in responders:
            ok = results[(requester, responder)]
            row += f"{'PASS' if ok else 'FAIL':<8}"
        print(row)
    print()

    if failures:
        print(f"{len(failures)} pairing(s) failed:")
        for requester, responder, output in failures:
            print(f"\n--- {requester} -> {responder} ---")
            # The last lines carry the counters, which name the failure kind.
            print("\n".join(output.strip().splitlines()[-25:]))
        print()
        print("A11 MATRIX: FAIL")
        return 1

    print(f"A11 MATRIX: PASS ({len(results)} pairings)")
    return 0


if __name__ == "__main__":
    sys.exit(main())
