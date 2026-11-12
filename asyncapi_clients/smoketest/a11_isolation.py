# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/a11_isolation.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-28
# Version:     1.1
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""A11 conformance: prove command responses are addressed, not broadcast.

The ordinary smoke scenarios ask "did the happy path work". This asks the
architectural question instead: **can a requester receive a response that
was not addressed to it?** Under A4 the answer was yes — every subscriber
received every response and was trusted to discard the ones that did not
correlate. Under A11 it must be no.

A private reply node therefore has a stronger invariant than an ordinary
consumer, and this harness encodes it:

    expected correlation    -> valid
    unexpected correlation  -> FAILURE
    foreign response        -> FAILURE
    short delivery          -> FAILURE
    timeout                 -> FAILURE

Failures are reported by kind rather than as one total, because the three
causes need different fixes and a red CI run should say which it is:

    routing         addressed elsewhere but delivered here (`to` != our node)
    absence         a MUST-carry property is missing
    transformation  addressed here, but an identifier was not carried through

`to` is what separates routing from transformation: a response bearing
another requester's correlation-id *and* another node's address was
mis-delivered; one bearing our address did reach the right place but lost
the correlation on the way.

There is deliberately no "ignore what isn't mine" path anywhere in it.
That behaviour belongs to the architecture A11 replaced, and tolerating it
here would let fan-out be reintroduced without the suite noticing — which
is the actual risk now that a working implementation exists.
"""
from __future__ import annotations

import argparse
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
SNIPPETS = "asyncapi_clients/output/python/snippets"
PY = sys.executable
JAVA_CP = "asyncapi_clients/smoketest/java/target/topas-snippets-shaded.jar"
CS_DLL = "asyncapi_clients/smoketest/csharp/bin/Release/net8.0/publish/TopasSnippets.dll"
TS_DIST = "asyncapi_clients/smoketest/typescript/dist"
RUST_BIN = "asyncapi_clients/smoketest/rust/target/release"

# The requester under test. Every converted language must satisfy the same
# invariants, so the harness is the acceptance gate for a conversion rather
# than a Python-specific script.
REQUESTERS: dict[str, list[str]] = {
    "python": [PY, f"{SNIPPETS}/s3.py"],
    "java": ["java", "-cp", JAVA_CP, "S3"],
    "csharp": ["dotnet", CS_DLL, "S3"],
    "typescript": ["node", f"{TS_DIST}/snippets/s3.js"],
    "rust": [f"{RUST_BIN}/s3"],
}

# The responder side of the same contract. Every entry consumes
# vms.command, answers the requester's reply node, mirrors message-id onto
# correlation-id and carries commandCorrelationId through unchanged — so
# any requester can be run against any responder.
RESPONDERS: dict[str, list[str]] = {
    "python": [PY, f"{SMOKE}/vms_responder.py"],
    "java": ["java", "-cp", JAVA_CP, "Responder"],
    "csharp": ["dotnet", CS_DLL, "Responder"],
    "typescript": ["node", f"{TS_DIST}/responder.js"],
    "rust": [f"{RUST_BIN}/responder"],
}
RESPONDER_LANG = os.environ.get("TOPAS_A11_RESPONDER", "python")
LANG = os.environ.get("TOPAS_A11_LANG", "python")

BROKER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
# Every counter is reported separately rather than rolled into a pass/fail
# total. When a language first fails in CI the useful question is *which kind*
# of failure it is — absence, transformation, or routing — and a single
# "transportErrors=2" cannot answer it.
SUMMARY_RE = re.compile(
    r"\[a11\] node=(?P<node>\S+) expected=(?P<expected>\d+) "
    r"received=(?P<received>\d+) foreign=(?P<foreign>\d+) "
    r"missingTo=(?P<missing_to>\d+|n/a) missingCorr=(?P<missing_corr>\d+) "
    r"wrongCorr=(?P<wrong_corr>\d+) "
    r"missingApp=(?P<missing_app>\d+) wrongApp=(?P<wrong_app>\d+)"
)


REQUEST_RE = re.compile(
    r"\[a11-request\] node=(?P<node>\S+) messageId=(?P<message_id>\S+)"
)


@dataclass
class Requester:
    """One concurrent requester under test."""

    label: str
    originator: str
    command_correlation_id: int
    expected_responses: int
    proc: subprocess.Popen | None = None
    pump: threading.Thread | None = None
    lock: threading.Lock = field(default_factory=threading.Lock)
    stdout: str = ""
    node: str | None = None
    request_message_id: str | None = None
    received: int = 0
    foreign: int = 0          # delivery isolation
    missing_to: int = 0       # transport: absence
    # Some client APIs cannot observe an absent property at all. That is a
    # third state, not a pass: a check that cannot fail must not be reported
    # as one that did not fail.
    missing_to_observable: bool = True
    missing_corr: int = 0     # transport: absence
    wrong_corr: int = 0       # transport: transformation
    missing_app: int = 0      # application: absence
    wrong_app: int = 0        # application: transformation
    failures: list[str] = field(default_factory=list)

    @property
    def ok(self) -> bool:
        return not self.failures


def _env(extra: dict[str, str]) -> dict[str, str]:
    env = os.environ.copy()
    env["PYTHONPATH"] = "."
    env["TOPAS_AMQP_URL"] = BROKER
    env.update(extra)
    return env


def _start_responder(
    progression: str, max_replies: int, delay: float | None = None
) -> subprocess.Popen:
    env = {
        "TOPAS_RESPONDER_PROGRESSION": progression,
        "TOPAS_RESPONDER_MAX_REPLIES": str(max_replies),
    }
    if delay is not None:
        env["TOPAS_RESPONDER_DELAY"] = str(delay)
    proc = subprocess.Popen(
        RESPONDERS[RESPONDER_LANG],
        cwd=str(ROOT), env=_env(env),
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, bufsize=1,
    )
    _await_responder_ready(proc)
    return proc


def _await_responder_ready(proc: subprocess.Popen, timeout: float = 30.0) -> None:
    """Block until the responder says it is consuming.

    Replaces a fixed sleep. A JVM or .NET responder takes appreciably
    longer to reach its first receive than Proton does, and a command
    published into that gap is discarded by the broker — which would look
    like a routing failure rather than a startup race.
    """
    deadline = time.monotonic() + timeout
    assert proc.stdout is not None
    while time.monotonic() < deadline:
        line = proc.stdout.readline()
        if not line:
            break
        if line.startswith("[responder-ready]"):
            return
    print(f"  [warn] responder did not signal readiness within {timeout:.0f}s")


def _pump(r: Requester) -> None:
    """Accumulate the requester's output as it arrives.

    ``communicate()`` only yields at exit, which is too late to aim a
    defective response at a node the requester is currently listening on.
    """
    assert r.proc is not None and r.proc.stdout is not None
    for line in r.proc.stdout:
        with r.lock:
            r.stdout += line


def _await_request(r: Requester, timeout: float = 20.0) -> bool:
    """Block until the requester reports the node and message-id it used."""
    deadline = time.monotonic() + timeout
    while time.monotonic() < deadline:
        with r.lock:
            m = REQUEST_RE.search(r.stdout)
        if m:
            r.node = m.group("node")
            r.request_message_id = m.group("message_id")
            return True
        if r.proc is not None and r.proc.poll() is not None:
            return False
        time.sleep(0.1)
    return False


def _start_requester(r: Requester) -> None:
    r.proc = subprocess.Popen(
        REQUESTERS[LANG],
        cwd=str(ROOT),
        env=_env({
            "TOPAS_ORIGINATOR": r.originator,
            "TOPAS_COMMAND_CORRELATION_ID": str(r.command_correlation_id),
            "TOPAS_EXPECTED_RESPONSES": str(r.expected_responses),
        }),
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, bufsize=1,
    )
    r.pump = threading.Thread(target=_pump, args=(r,), daemon=True)
    r.pump.start()


def _collect(r: Requester, timeout: float) -> None:
    try:
        r.proc.wait(timeout=timeout)
    except subprocess.TimeoutExpired:
        r.proc.kill()
        r.failures.append(f"timed out after {timeout:.0f}s")
    if r.pump is not None:
        r.pump.join(timeout=5)
    with r.lock:
        text = r.stdout

    m = SUMMARY_RE.search(text)
    if not m:
        r.failures.append("no [a11] summary — requester did not complete")
        return
    r.node = m.group("node")
    r.received = int(m.group("received"))
    r.foreign = int(m.group("foreign"))
    raw_missing_to = m.group("missing_to")
    r.missing_to_observable = raw_missing_to != "n/a"
    r.missing_to = 0 if not r.missing_to_observable else int(raw_missing_to)
    r.missing_corr = int(m.group("missing_corr"))
    r.wrong_corr = int(m.group("wrong_corr"))
    r.missing_app = int(m.group("missing_app"))
    r.wrong_app = int(m.group("wrong_app"))

    if r.received != r.expected_responses:
        r.failures.append(
            f"expected {r.expected_responses} responses, received {r.received}"
        )
    # Asserted separately so one cannot mask another, and named by failure
    # kind so a red CI run identifies itself without a local repro.
    if r.foreign:
        r.failures.append(
            f"{r.foreign} response(s) for another requester reached its reply "
            "node — routing"
        )
    if r.missing_to_observable and r.missing_to:
        r.failures.append(f"{r.missing_to} response(s) carried no `to` — absence")
    if r.missing_corr:
        r.failures.append(
            f"{r.missing_corr} response(s) carried no correlation-id — absence"
        )
    if r.wrong_corr:
        r.failures.append(
            f"{r.wrong_corr} response(s) addressed here but bearing another "
            "correlation-id — transformation"
        )
    if r.missing_app:
        r.failures.append(
            f"{r.missing_app} response(s) carried no commandCorrelationId — absence"
        )
    if r.wrong_app:
        r.failures.append(
            f"{r.wrong_app} response(s) altered commandCorrelationId — transformation"
        )


def _report(requesters: list[Requester]) -> bool:
    print()
    for r in requesters:
        print(f"Requester {r.label}  ({r.node or 'no reply node assigned'})")
        print( "  delivery:")
        print(f"    expected:             {r.expected_responses}")
        print(f"    received:             {r.received}")
        print(f"    foreign:              {r.foreign}")
        print( "  transport:")
        print(f"    missing-to:           "
              f"{r.missing_to if r.missing_to_observable else 'n/a (not observable via this client API)'}")
        print(f"    missing-correlation:  {r.missing_corr}")
        print(f"    wrong-correlation:    {r.wrong_corr}")
        print( "  application:")
        print(f"    missing-correlation:  {r.missing_app}")
        print(f"    wrong-correlation:    {r.wrong_app}")
        print(f"  result:                 {'PASS' if r.ok else 'FAIL'}")
        for f in r.failures:
            print(f"    - {f}")
        print()

    nodes = [r.node for r in requesters if r.node]
    distinct = len(set(nodes)) == len(nodes) and len(nodes) == len(requesters)
    if not distinct:
        print("  - reply nodes were not distinct across requesters")

    ok = distinct and all(r.ok for r in requesters)
    print(f"A11 ISOLATION: {'PASS' if ok else 'FAIL'}")
    return ok


def scenario_two_requesters() -> bool:
    """Two concurrent requesters, one with an interim response.

    A also carries a `pending` frame, so the run proves a reply node serves
    a progression rather than a single-use callback while isolation holds.
    """
    print("=== A11: two concurrent requesters, distinct reply nodes ===")
    responder = _start_responder("pending,success", max_replies=2)

    requesters = [
        Requester("A", "subscriber-a", 1111, expected_responses=2),
        Requester("B", "subscriber-b", 2222, expected_responses=2),
    ]
    for r in requesters:          # started together, not in sequence —
        _start_requester(r)       # sequential runs let broken routing pass
    for r in requesters:
        _collect(r, timeout=30)

    responder.kill()
    responder.communicate()
    return _report(requesters)


def scenario_shared_correlation_id() -> bool:
    """Two principals using the *same* commandCorrelationId.

    Proves the correlation namespace is per-requester rather than global:
    both must execute independently and each receive its own result. If the
    namespace were global, one would capture the other's response.
    """
    print("=== A11: same commandCorrelationId from two principals ===")
    responder = _start_responder("success", max_replies=2)

    requesters = [
        Requester("A", "subscriber-a", 7777, expected_responses=1),
        Requester("B", "subscriber-b", 7777, expected_responses=1),
    ]
    for r in requesters:
        _start_requester(r)
    for r in requesters:
        _collect(r, timeout=30)

    responder.kill()
    responder.communicate()
    return _report(requesters)


def scenario_foreign_traffic() -> bool:
    """Traffic aimed at another reply node while A waits.

    Under A4 the equivalent test showed "A receives everything and ignores
    what does not correlate". Under A11 it must show A is never delivered
    it at all — so the assertion is foreign == 0, not "A still worked".
    """
    print("=== A11: foreign traffic to an unrelated reply node ===")
    # The responder is deliberately slow between `pending` and `success` so A
    # is provably still listening when the noise is sent. Without that the
    # test passes vacuously: A completes and closes first, and foreign == 0
    # says nothing at all. Verified by aiming the same noise at A's own node,
    # which does raise foreign to 1 — so a zero here is meaningful.
    responder = _start_responder("pending,success", max_replies=1, delay=6.0)

    a = Requester("A", "subscriber-a", 3333, expected_responses=2)
    _start_requester(a)
    time.sleep(2.0)  # A's dynamic node is open and awaiting the terminal frame

    noise = subprocess.Popen(
        [PY, f"{SMOKE}/reply_node_noise.py"],
        cwd=str(ROOT), env=_env({}),
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
    )
    noise_out = ""
    try:
        noise_out = noise.communicate(timeout=15)[0] or ""
    except subprocess.TimeoutExpired:
        noise.kill()
        noise_out = noise.communicate()[0] or ""

    _collect(a, timeout=40)
    responder.kill()
    responder.communicate()

    # If the noise never actually sent, the scenario proved nothing.
    if "[noise] sent" not in noise_out:
        a.failures.append("foreign traffic was never sent — test is vacuous")
    return _report([a])


# Defect -> the counter it must move. Each entry is one assertion proving
# the corresponding check in every language's S3 can actually fail.
DEFECT_COUNTERS = [
    ("mis-delivered", "foreign"),
    ("mis-correlated", "wrong_corr"),
    ("missing-correlation", "missing_corr"),
    ("missing-to", "missing_to"),
    ("missing-app", "missing_app"),
    ("wrong-app", "wrong_app"),
]


def scenario_negative_self_test() -> bool:
    """Prove every A11 counter can be made to fire.

    The other scenarios assert that nothing goes wrong. On their own they
    cannot distinguish a conformant requester from one whose checks are
    unreachable — which this suite has shipped twice. So
    each counter gets a response engineered to trigger it, and a requester
    that reports a clean run here has failed, not passed.
    """
    print("=== A11: negative self-test — every counter must be reachable ===")
    # Slow between `pending` and `success` so the defects land while the
    # requester is still listening rather than after it has closed.
    responder = _start_responder("pending,success", max_replies=1, delay=10.0)

    # Four defects are counted as delivered (they reached the right node and
    # were merely malformed); the two mis-routed ones are rejected before
    # the delivery count, so they are not expected responses.
    counted_defects = sum(
        1 for d, _ in DEFECT_COUNTERS if d not in ("mis-delivered", "mis-correlated")
    )
    a = Requester("A", "subscriber-a", 4444,
                  expected_responses=2 + counted_defects)
    _start_requester(a)

    if not _await_request(a):
        a.failures.append("requester never reported its node and message-id")
        return _report([a])

    defects = ",".join(d for d, _ in DEFECT_COUNTERS)
    injector = subprocess.Popen(
        [PY, f"{SMOKE}/a11_defect_injector.py"],
        cwd=str(ROOT),
        env=_env({
            "TOPAS_TARGET_NODE": a.node,
            "TOPAS_TARGET_MESSAGE_ID": a.request_message_id,
            "TOPAS_TARGET_APP_ID": str(a.command_correlation_id),
            "TOPAS_DEFECTS": defects,
        }),
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
    )
    try:
        injector_out = injector.communicate(timeout=20)[0] or ""
    except subprocess.TimeoutExpired:
        injector.kill()
        injector_out = injector.communicate()[0] or ""

    _collect(a, timeout=45)
    responder.kill()
    responder.communicate()

    if "[inject] sent" not in injector_out:
        a.failures.append("defects were never injected — self-test is vacuous")

    # Invert the verdict: here a zero counter is the failure.
    a.failures = [f for f in a.failures if "response(s)" not in f
                  and not f.startswith("expected ")]
    for defect, attr in DEFECT_COUNTERS:
        if attr == "missing_to" and not a.missing_to_observable:
            # Declared unobservable by the client rather than silently zero.
            # Recorded on every run so the gap stays visible instead of
            # decaying into an assumption.
            print("  note: `to` absence is not observable through this "
                  "client API; the defect was injected and the check "
                  "is declared n/a rather than passed")
            continue
        if getattr(a, attr) < 1:
            a.failures.append(
                f"{defect} did not move `{attr}` — that check is unreachable"
            )
    if a.received != a.expected_responses:
        a.failures.append(
            f"expected {a.expected_responses} delivered responses, "
            f"received {a.received}"
        )
    return _report([a])


def scenario_identifier_types() -> bool:
    """Round-trip all four AMQP message-id representations.

    A11 requires a response's `correlation-id` to equal the request's
    `message-id`, and AMQP defines that identifier as a union of ulong,
    uuid, binary and string — so "equal" is a claim about type as well as
    value. Every language's transport assertion rests on the broker
    preserving the representation; this is where that is actually checked.

    Rust only, and deliberately so: it is the one client here whose type
    system makes the union explicit, so it is the only one that can assert
    the property rather than approximate it. Reported as `n/a` elsewhere
    rather than skipped silently.
    """
    print("=== A11: AMQP identifier type fidelity ===")
    if LANG != "rust":
        print("  n/a — this client API does not expose the AMQP identifier")
        print("        union as a distinguishable type; checked under `rust`")
        print("A11 IDENTIFIER TYPES: n/a")
        # None, not True. Reporting this as a pass would put a green mark
        # against a check that did not run — the same mistake as letting an
        # unobservable property read as a satisfied one.
        return None

    proc = subprocess.run(
        [f"{RUST_BIN}/a11_ids"],
        cwd=str(ROOT), env=_env({}),
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, timeout=60,
    )
    print(proc.stdout.rstrip())
    ok = "A11 IDENTIFIER TYPES: PASS" in proc.stdout
    if not ok:
        print("  - an identifier representation was not preserved end to end")
    return ok


SCENARIOS = {
    "two-requesters": scenario_two_requesters,
    "shared-correlation-id": scenario_shared_correlation_id,
    "foreign-traffic": scenario_foreign_traffic,
    "negative-self-test": scenario_negative_self_test,
    "identifier-types": scenario_identifier_types,
}


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--only", choices=sorted(SCENARIOS), help="run one scenario")
    ap.add_argument("--lang", choices=sorted(REQUESTERS), default=LANG,
                    help="which language's S3 requester to hold to the invariants")
    ap.add_argument("--responder", choices=sorted(RESPONDERS), default=RESPONDER_LANG,
                    help="which language's responder answers it (default python)")
    args = ap.parse_args()

    globals()["LANG"] = args.lang
    globals()["RESPONDER_LANG"] = args.responder

    print(f"[a11] broker = {BROKER}")
    print(f"[a11] requester = {LANG}")
    print(f"[a11] responder = {RESPONDER_LANG}")
    names = [args.only] if args.only else list(SCENARIOS)
    results = {}
    for name in names:
        results[name] = SCENARIOS[name]()
        print()

    print("=== SUMMARY ===")
    for name, ok in results.items():
        verdict = "n/a " if ok is None else ("PASS" if ok else "FAIL")
        print(f"  {verdict}  {name}")
    # `n/a` is neither a pass nor a failure: it does not fail the run, and
    # it is not counted as evidence either.
    return 0 if all(ok is not False for ok in results.values()) else 1


if __name__ == "__main__":
    sys.exit(main())
