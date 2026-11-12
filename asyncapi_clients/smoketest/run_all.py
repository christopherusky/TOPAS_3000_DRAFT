"""Run all S1-S7 Python snippets end-to-end against an AMQP 1.0 broker.

File:        smoketest/run_all.py
Origin:      Hand-written for asyncapi-clients smoke testing
Changes:     Hand-edit allowed — this is a test harness, not generated output
Author:      Christopher Kettell, TRL Software
Date:        2026-05-03
License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)

Designed to drive the snippets and publishers we already have, so
both `make smoketest` (local) and the CI workflow run the same
exact commands.

Reads TOPAS_AMQP_URL from env (default amqp://localhost:5672).
Exits 0 if all scenarios pass, 1 otherwise.
"""
from __future__ import annotations

import os
import re
import select
import subprocess
import sys
import time
from dataclasses import dataclass, field
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parents[2]))
from bin.scope import load_scope  # noqa: E402

ROOT = Path(__file__).resolve().parents[2]
SNIP = "asyncapi_clients/output/python/snippets"
SMOKE = "asyncapi_clients/smoketest"

# S2 expects a fault from one of the live domains scope.toml names, the same list
# fault_publisher.py sprays to.
FAULT_FROM = (
    r"\[fault from ("
    + "|".join(re.escape(d) for d in load_scope().smoke["fault_domains"])
    + r")\]"
)
BROKER = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
PY = sys.executable
JAVA_CP = "asyncapi_clients/smoketest/java/target/topas-snippets-shaded.jar"
CSHARP_DLL = (
    "asyncapi_clients/smoketest/csharp/bin/Release/net8.0/publish/TopasSnippets.dll"
)

TS_DIST = "asyncapi_clients/smoketest/typescript/dist"


RUST_DIST = "asyncapi_clients/smoketest/rust/target/release"


@dataclass
class Scenario:
    name: str
    subscriber: list[str]
    publishers: list[list[str]]
    expected: str  # regex that must appear in subscriber stdout
    sub_env: dict = field(default_factory=dict)
    pub_env: dict = field(default_factory=dict)
    helpers: list[list[str]] = field(default_factory=list)
    helper_env: dict = field(default_factory=dict)
    sub_wait: float = 2.5  # fallback delay when ready_pattern is None
    # Wait for the subscriber to say it has attached its links, rather than
    # guessing with a sleep. Every publisher in this harness sends a fixed
    # burst and exits, so a message published before the receiver attaches is
    # discarded by the broker — AMQP multicast, no consumer, no retry — and
    # the subscriber then waits for messages that no longer exist. That is a
    # race against process startup, which is why it bit the JVM and CLR legs
    # first. Set ready_pattern=None to fall back to sub_wait.
    ready_pattern: str | None = r"\[ready\]"
    ready_timeout: float = 15.0
    timeout: float = 20.0  # max seconds to wait for expected pattern


PYTHON_SCENARIOS = [
    Scenario(
        name="S1 detector.measurement",
        subscriber=[PY, f"{SNIP}/s1.py"],
        sub_env={"TOPAS_MAX_MESSAGES": "5"},
        publishers=[[PY, f"{SMOKE}/publisher.py"]],
        expected=r"\[5\] \{",
    ),
    Scenario(
        name="S2 *.fault wildcard",
        subscriber=[PY, f"{SNIP}/s2.py"],
        sub_env={"TOPAS_MAX_MESSAGES": "3"},
        publishers=[[PY, f"{SMOKE}/fault_publisher.py"]],
        expected=FAULT_FROM,
    ),
    Scenario(
        name="S3 vms.command round-trip",
        subscriber=[PY, f"{SNIP}/s3.py"],
        sub_env={"TOPAS_ORIGINATOR": "s3-smoke-test"},
        helpers=[[PY, f"{SMOKE}/vms_responder.py"]],
        helper_env={
            "TOPAS_RESPONDER_PROGRESSION": "pending,pending,success",
            "TOPAS_RESPONDER_MAX_REPLIES": "1",  # one COMMAND -> one full progression
        },
        publishers=[],  # S3's snippet IS the publisher; helper handles the response
        expected=r"\[response\] (success|failed|timedOut|invalid)\b",
        sub_wait=0.5,  # responder needs head-start, not the subscriber
    ),
    Scenario(
        name="S4 sync batch + delta stream",
        subscriber=[PY, f"{SNIP}/s4.py"],
        publishers=[[PY, f"{SMOKE}/sync_delta_publisher.py"]],
        expected=r"\[sync [0-9a-f]{8}-[0-9a-f-]{27}\] complete",
        timeout=15.0,  # S4 never self-exits; we kill on pattern match
    ),
    Scenario(
        name="S5 traffic-signal telemetry",
        subscriber=[PY, f"{SNIP}/s5.py"],
        sub_env={"TOPAS_MAX_MESSAGES": "5"},
        publishers=[[PY, f"{SMOKE}/traffic_signal_publisher.py"]],
        expected=r"\[5\] signal=TS-\d+ mode=centralMode",
    ),
    Scenario(
        name="S6 transport-link discriminated union",
        subscriber=[PY, f"{SNIP}/s6.py"],
        sub_env={"TOPAS_MAX_MESSAGES": "3"},
        publishers=[[PY, f"{SMOKE}/transport_link_publisher.py"]],
        expected=r"\[3\] (SCOOT|GPS|ANPR)",
    ),
    Scenario(
        name="S7 network topology snapshot",
        subscriber=[PY, f"{SNIP}/s7.py"],
        publishers=[[PY, f"{SMOKE}/network_publisher.py"]],
        expected=r"\[topology\] node=1, link=1, zone=1, path=1, turn=1",
    ),
]

JAVA_SCENARIOS = [
    Scenario(
        name="S1 detector.measurement (java)",
        subscriber=["java", "-cp", JAVA_CP, "S1"],
        sub_env={"TOPAS_MAX_MESSAGES": "5"},
        publishers=[[PY, f"{SMOKE}/publisher.py"]],
        expected=r"\[5\] \{",
    ),
    Scenario(
        name="S2 *.fault wildcard (java)",
        subscriber=["java", "-cp", JAVA_CP, "S2"],
        sub_env={"TOPAS_MAX_MESSAGES": "3"},
        publishers=[[PY, f"{SMOKE}/fault_publisher.py"]],
        expected=FAULT_FROM,
    ),
    Scenario(
        name="S3 vms.command round-trip (java)",
        subscriber=["java", "-cp", JAVA_CP, "S3"],
        sub_env={"TOPAS_ORIGINATOR": "s3-smoke-test"},
        helpers=[[PY, f"{SMOKE}/vms_responder.py"]],
        helper_env={
            "TOPAS_RESPONDER_PROGRESSION": "pending,pending,success",
            "TOPAS_RESPONDER_MAX_REPLIES": "1",  # one COMMAND -> one full progression
        },
        publishers=[],  # Java S3 IS the publisher; helper handles the response
        expected=r"\[response\] (success|failed|timedOut|invalid)\b",
        sub_wait=0.5,
    ),
    Scenario(
        name="S4 sync batch + delta stream (java)",
        subscriber=["java", "-cp", JAVA_CP, "S4"],
        publishers=[[PY, f"{SMOKE}/sync_delta_publisher.py"]],
        expected=r"\[sync [0-9a-f]{8}-[0-9a-f-]{27}\] complete",
        timeout=15.0,
    ),
    Scenario(
        name="S5 traffic-signal telemetry (java)",
        subscriber=["java", "-cp", JAVA_CP, "S5"],
        sub_env={"TOPAS_MAX_MESSAGES": "5"},
        publishers=[[PY, f"{SMOKE}/traffic_signal_publisher.py"]],
        expected=r"\[5\] signal=TS-\d+ mode=centralMode",
    ),
    Scenario(
        name="S6 transport-link discriminated union (java)",
        subscriber=["java", "-cp", JAVA_CP, "S6"],
        sub_env={"TOPAS_MAX_MESSAGES": "3"},
        publishers=[[PY, f"{SMOKE}/transport_link_publisher.py"]],
        expected=r"\[3\] (SCOOT|GPS|ANPR)",
    ),
    Scenario(
        name="S7 network topology snapshot (java)",
        subscriber=["java", "-cp", JAVA_CP, "S7"],
        publishers=[[PY, f"{SMOKE}/network_publisher.py"]],
        expected=r"\[topology\] node=1, link=1, zone=1, path=1, turn=1",
    ),
]

CSHARP_SCENARIOS = [
    Scenario(
        name="S1 detector.measurement (csharp)",
        subscriber=["dotnet", CSHARP_DLL, "S1"],
        sub_env={"TOPAS_MAX_MESSAGES": "5"},
        publishers=[[PY, f"{SMOKE}/publisher.py"]],
        expected=r"\[5\] \{",
    ),
    Scenario(
        name="S2 *.fault wildcard (csharp)",
        subscriber=["dotnet", CSHARP_DLL, "S2"],
        sub_env={"TOPAS_MAX_MESSAGES": "3"},
        publishers=[[PY, f"{SMOKE}/fault_publisher.py"]],
        expected=FAULT_FROM,
    ),
    Scenario(
        name="S3 vms.command round-trip (csharp)",
        subscriber=["dotnet", CSHARP_DLL, "S3"],
        sub_env={"TOPAS_ORIGINATOR": "s3-smoke-test"},
        helpers=[[PY, f"{SMOKE}/vms_responder.py"]],
        helper_env={
            "TOPAS_RESPONDER_PROGRESSION": "pending,pending,success",
            "TOPAS_RESPONDER_MAX_REPLIES": "1",  # one COMMAND -> one full progression
        },
        publishers=[],
        expected=r"\[response\] (success|failed|timedOut|invalid)\b",
        sub_wait=0.5,
    ),
    Scenario(
        name="S4 sync batch + delta stream (csharp)",
        subscriber=["dotnet", CSHARP_DLL, "S4"],
        publishers=[[PY, f"{SMOKE}/sync_delta_publisher.py"]],
        expected=r"\[sync [0-9a-f]{8}-[0-9a-f-]{27}\] complete",
        timeout=15.0,
    ),
    Scenario(
        name="S5 traffic-signal telemetry (csharp)",
        subscriber=["dotnet", CSHARP_DLL, "S5"],
        sub_env={"TOPAS_MAX_MESSAGES": "5"},
        publishers=[[PY, f"{SMOKE}/traffic_signal_publisher.py"]],
        expected=r"\[5\] signal=TS-\d+ mode=centralMode",
    ),
    Scenario(
        name="S6 transport-link discriminated union (csharp)",
        subscriber=["dotnet", CSHARP_DLL, "S6"],
        sub_env={"TOPAS_MAX_MESSAGES": "3"},
        publishers=[[PY, f"{SMOKE}/transport_link_publisher.py"]],
        expected=r"\[3\] (SCOOT|GPS|ANPR)",
    ),
    Scenario(
        name="S7 network topology snapshot (csharp)",
        subscriber=["dotnet", CSHARP_DLL, "S7"],
        publishers=[[PY, f"{SMOKE}/network_publisher.py"]],
        expected=r"\[topology\] node=1, link=1, zone=1, path=1, turn=1",
    ),
]

TYPESCRIPT_SCENARIOS = [
    Scenario(
        name="S1 detector.measurement (typescript)",
        subscriber=["node", f"{TS_DIST}/snippets/s1.js"],
        sub_env={"TOPAS_MAX_MESSAGES": "5"},
        publishers=[[PY, f"{SMOKE}/publisher.py"]],
        expected=r"\[5\] \{",
    ),
    Scenario(
        name="S2 *.fault wildcard (typescript)",
        subscriber=["node", f"{TS_DIST}/snippets/s2.js"],
        sub_env={"TOPAS_MAX_MESSAGES": "3"},
        publishers=[[PY, f"{SMOKE}/fault_publisher.py"]],
        expected=FAULT_FROM,
    ),
    Scenario(
        name="S3 vms.command round-trip (typescript)",
        subscriber=["node", f"{TS_DIST}/snippets/s3.js"],
        sub_env={"TOPAS_ORIGINATOR": "s3-smoke-test"},
        helpers=[[PY, f"{SMOKE}/vms_responder.py"]],
        helper_env={
            "TOPAS_RESPONDER_PROGRESSION": "pending,pending,success",
            "TOPAS_RESPONDER_MAX_REPLIES": "1",  # one COMMAND -> one full progression
        },
        publishers=[],  # TS S3 IS the publisher; helper handles the response
        expected=r"\[response\] (success|failed|timedOut|invalid)\b",
        sub_wait=0.5,
    ),
    Scenario(
        name="S4 sync batch + delta stream (typescript)",
        subscriber=["node", f"{TS_DIST}/snippets/s4.js"],
        publishers=[[PY, f"{SMOKE}/sync_delta_publisher.py"]],
        expected=r"\[sync [0-9a-f]{8}-[0-9a-f-]{27}\] complete",
        timeout=15.0,
    ),
    Scenario(
        name="S5 traffic-signal telemetry (typescript)",
        subscriber=["node", f"{TS_DIST}/snippets/s5.js"],
        sub_env={"TOPAS_MAX_MESSAGES": "5"},
        publishers=[[PY, f"{SMOKE}/traffic_signal_publisher.py"]],
        expected=r"\[5\] signal=TS-\d+ mode=centralMode",
    ),
    Scenario(
        name="S6 transport-link discriminated union (typescript)",
        subscriber=["node", f"{TS_DIST}/snippets/s6.js"],
        sub_env={"TOPAS_MAX_MESSAGES": "3"},
        publishers=[[PY, f"{SMOKE}/transport_link_publisher.py"]],
        expected=r"\[3\] (SCOOT|GPS|ANPR)",
    ),
    Scenario(
        name="S7 network topology snapshot (typescript)",
        subscriber=["node", f"{TS_DIST}/snippets/s7.js"],
        publishers=[[PY, f"{SMOKE}/network_publisher.py"]],
        expected=r"\[topology\] node=1, link=1, zone=1, path=1, turn=1",
    ),
]


RUST_SCENARIOS = [
    Scenario(
        name="S1 detector.measurement (rust)",
        subscriber=[f"{RUST_DIST}/s1"],
        sub_env={"TOPAS_MAX_MESSAGES": "5"},
        publishers=[[PY, f"{SMOKE}/publisher.py"]],
        # Rust serde_json prints `Value` objects as `{...}` with quoted keys.
        expected=r"\[5\] \{",
    ),
    Scenario(
        name="S2 *.fault wildcard (rust)",
        subscriber=[f"{RUST_DIST}/s2"],
        sub_env={"TOPAS_MAX_MESSAGES": "3"},
        publishers=[[PY, f"{SMOKE}/fault_publisher.py"]],
        expected=FAULT_FROM,
    ),
    Scenario(
        name="S3 vms.command round-trip (rust)",
        subscriber=[f"{RUST_DIST}/s3"],
        sub_env={"TOPAS_ORIGINATOR": "s3-smoke-test"},
        helpers=[[PY, f"{SMOKE}/vms_responder.py"]],
        helper_env={
            "TOPAS_RESPONDER_PROGRESSION": "pending,pending,success",
            "TOPAS_RESPONDER_MAX_REPLIES": "1",  # one COMMAND -> one full progression
        },
        publishers=[],  # Rust S3 IS the publisher; helper handles the response
        expected=r"\[response\] (success|failed|timedOut|invalid)\b",
        sub_wait=0.5,
    ),
    Scenario(
        name="S4 sync batch + delta stream (rust)",
        subscriber=[f"{RUST_DIST}/s4"],
        publishers=[[PY, f"{SMOKE}/sync_delta_publisher.py"]],
        expected=r"\[sync [0-9a-f]{8}-[0-9a-f-]{27}\] complete",
        timeout=15.0,
    ),
    Scenario(
        name="S5 traffic-signal telemetry (rust)",
        subscriber=[f"{RUST_DIST}/s5"],
        sub_env={"TOPAS_MAX_MESSAGES": "5"},
        publishers=[[PY, f"{SMOKE}/traffic_signal_publisher.py"]],
        expected=r"\[5\] signal=TS-\d+ mode=centralMode",
    ),
    Scenario(
        name="S6 transport-link discriminated union (rust)",
        subscriber=[f"{RUST_DIST}/s6"],
        sub_env={"TOPAS_MAX_MESSAGES": "3"},
        publishers=[[PY, f"{SMOKE}/transport_link_publisher.py"]],
        expected=r"\[3\] (SCOOT|GPS|ANPR)",
    ),
    Scenario(
        name="S7 network topology snapshot (rust)",
        subscriber=[f"{RUST_DIST}/s7"],
        publishers=[[PY, f"{SMOKE}/network_publisher.py"]],
        expected=r"\[topology\] node=1, link=1, zone=1, path=1, turn=1",
        # S7 is the only scenario whose publisher sends exactly one message
        # per address and then exits: `self.sent < len(self.channels)`. Every
        # other scenario publishes N messages to a single address, so a
        # subscriber that attaches late still catches a later message. Here a
        # message published to an address whose receiver has not yet attached
        # is discarded — AMQP multicast with no consumer, no retention, no
        # retry — and `seen` can never reach five.
        #
        # The Rust snippet attaches its five receivers sequentially on one
        # session, awaiting the peer's attach frame each time, so it needs the
        # longest head start of the six languages. The 2.5s default was enough
        # locally (attaches complete in <0.2s) but not on a loaded CI runner,
        # where this failed reproducibly. Verified locally: at sub_wait=0.05 it
        # fails having seen only node/link/zone — the last two receivers in
        # CHANNELS order lose their single message.
        sub_wait=6.0,
        # The snippet carries its own TOTAL_DEADLINE of 30s — the only S7 that
        # does. On the orchestrator's 20s default we killed it before it could
        # reach its own timeout branch, so failures surfaced as an empty stderr
        # tail with nothing to diagnose. Outlive it so its diagnostics show.
        timeout=35.0,
    ),
]


LANG_SCENARIOS: dict[str, list[Scenario]] = {
    "python":     PYTHON_SCENARIOS,
    "java":       JAVA_SCENARIOS,
    "csharp":     CSHARP_SCENARIOS,
    "typescript": TYPESCRIPT_SCENARIOS,
    "rust":       RUST_SCENARIOS,
}

LANG_PREFIX: dict[str, str] = {
    "python":     "PY",
    "java":       "JV",
    "csharp":     "CS",
    "typescript": "TS",
    "rust":       "RU",
}

LANG_TOOLCHAIN_PROBE: dict[str, list[str]] = {
    "python":     [PY, "--version"],
    "java":       ["java", "-version"],
    "csharp":     ["dotnet", "--version"],
    "typescript": ["node", "--version"],
    "rust":       ["cargo", "--version"],
}


# Scenarios in this set may fail without failing CI. An xfail entry
# that passes is reported as "UNEXPECTED PASS — promote out of xfail"
# but does not fail the build. Currently empty: all seven Java
# scenarios pass end-to-end after the createTopic fix (2a51e30) and
# the AmqpJmsMessageFacade fix for hyphenated AMQP property names
# (4cc2acb). Add scenario names here if a future template change
# introduces a regression we want to land flagged-but-not-failing.
XFAIL: set[str] = set()


def _env(extra: dict) -> dict:
    e = os.environ.copy()
    e["PYTHONPATH"] = str(ROOT)
    e["TOPAS_AMQP_URL"] = BROKER
    e.update(extra)
    return e


def _kill(proc: subprocess.Popen) -> None:
    if proc.poll() is None:
        proc.terminate()
        try:
            proc.wait(timeout=2)
        except subprocess.TimeoutExpired:
            proc.kill()
            proc.wait(timeout=2)


def _read_until(proc: subprocess.Popen, pattern: re.Pattern, deadline: float) -> str:
    """Drain proc.stdout (line-buffered) until pattern matches or deadline.

    Uses select.select with the remaining-deadline as the timeout so a
    silent subprocess does NOT block the orchestrator past the deadline.
    The previous blocking-readline implementation would hang indefinitely
    if the subprocess was alive but quiet (observed in CI when
    fe2o3-amqp's session/connection close path stalls on slow runners).
    """
    chunks: list[str] = []
    fd = proc.stdout.fileno() if proc.stdout else None
    if fd is None:
        return ""
    pending = ""  # partial line carried between reads
    while True:
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            break
        if proc.poll() is not None:
            # Process exited — drain remaining stdout.
            rest = proc.stdout.read() if proc.stdout else ""
            if rest:
                chunks.append(rest)
            break
        # Wait up to `remaining` seconds for data to be readable, with a
        # cap of 0.5 s per iteration so we still re-check process state
        # responsively (no blocking past the deadline).
        slice_timeout = min(remaining, 0.5)
        ready, _, _ = select.select([fd], [], [], slice_timeout)
        if not ready:
            continue
        # Read whatever is available without blocking. readline() would
        # still block waiting for a newline after partial data; use a
        # raw os.read and split lines ourselves.
        try:
            raw = os.read(fd, 4096)
        except OSError:
            break
        if not raw:
            # EOF.
            break
        pending += raw.decode("utf-8", errors="replace")
        # Process complete lines; keep any trailing partial in `pending`.
        *complete, pending = pending.split("\n")
        for line in complete:
            chunks.append(line + "\n")
        if pattern.search("".join(chunks)):
            break
    if pending:
        chunks.append(pending)
    return "".join(chunks)


def run_scenario(s: Scenario) -> tuple[bool, str]:
    helpers: list[subprocess.Popen] = []
    sub: subprocess.Popen | None = None
    pattern = re.compile(s.expected)
    try:
        for h in s.helpers:
            helpers.append(subprocess.Popen(
                h, env=_env(s.helper_env), cwd=str(ROOT),
                stdout=subprocess.DEVNULL, stderr=subprocess.STDOUT,
            ))
        if s.helpers:
            time.sleep(0.5)  # let helpers attach links

        sub = subprocess.Popen(
            s.subscriber, env=_env(s.sub_env), cwd=str(ROOT),
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
            text=True, bufsize=1,
        )

        # Anything consumed while waiting for readiness still belongs to the
        # scenario's output, so keep it and prepend it below.
        preamble = ""
        if s.ready_pattern:
            ready_re = re.compile(s.ready_pattern)
            preamble = _read_until(
                sub, ready_re, time.monotonic() + s.ready_timeout
            )
            if not ready_re.search(preamble):
                preamble += (
                    f"\n[harness] subscriber did not signal readiness within "
                    f"{s.ready_timeout}s; publishing anyway\n"
                )
        else:
            time.sleep(s.sub_wait)

        # Capture publisher output rather than discarding it. When a
        # subscriber reports never having seen a channel, the first question
        # is whether the publisher actually sent it — and with DEVNULL there
        # was no way to tell a lost message from one never published.
        pub_out: list[str] = []
        for p in s.publishers:
            try:
                res = subprocess.run(
                    p, env=_env(s.pub_env), cwd=str(ROOT),
                    timeout=10, check=False,
                    stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                    text=True,
                )
                if res.stdout:
                    pub_out.append(res.stdout.strip())
            except subprocess.TimeoutExpired:
                return False, f"publisher {p[-1]} timed out"

        deadline = time.monotonic() + s.timeout
        out = preamble + _read_until(sub, pattern, deadline)
        matched = bool(pattern.search(out))
        if not matched and pub_out:
            out = "--- publisher output ---\n" + "\n".join(pub_out) + "\n--- subscriber output ---\n" + out
        return matched, out
    finally:
        if sub is not None:
            _kill(sub)
        for h in helpers:
            _kill(h)


def _toolchain_present(lang: str) -> bool:
    probe = LANG_TOOLCHAIN_PROBE[lang]
    try:
        subprocess.run(
            probe, check=True, stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL,
            timeout=10,
        )
        return True
    except (FileNotFoundError, subprocess.CalledProcessError, subprocess.TimeoutExpired):
        return False


def _parse_args(argv: list[str]) -> str:
    import argparse
    choices = ["all", *LANG_SCENARIOS.keys()]
    p = argparse.ArgumentParser(
        description="Run the asyncapi-clients S1-S7 smoke harness."
    )
    p.add_argument(
        "--lang", default="all", choices=choices,
        help=f"language to run (default: all = {', '.join(LANG_SCENARIOS)})",
    )
    return p.parse_args(argv).lang


def main(argv: list[str] | None = None) -> int:
    requested = _parse_args(argv if argv is not None else sys.argv[1:])
    print(f"[smoketest] broker = {BROKER}", flush=True)

    if requested == "all":
        run_langs: list[str] = []
        for lang in LANG_SCENARIOS:
            if _toolchain_present(lang):
                run_langs.append(lang)
            else:
                print(
                    f"[smoketest] skipping {lang}: "
                    f"`{LANG_TOOLCHAIN_PROBE[lang][0]}` not on PATH",
                    flush=True,
                )
        if not run_langs:
            print("[smoketest] no languages have a working toolchain — nothing to do.",
                  flush=True)
            return 1
    else:
        if not _toolchain_present(requested):
            print(
                f"[smoketest] requested lang '{requested}' but toolchain "
                f"`{LANG_TOOLCHAIN_PROBE[requested][0]}` is not on PATH.",
                flush=True,
            )
            return 1
        run_langs = [requested]

    print(f"[smoketest] running languages: {', '.join(run_langs)}", flush=True)

    results: list[tuple[str, str, str, float]] = []  # (lang, name, status, elapsed)
    real_failures: list[tuple[str, str]] = []

    for lang in run_langs:
        prefix = LANG_PREFIX[lang]
        for s in LANG_SCENARIOS[lang]:
            t0 = time.monotonic()
            passed, out = run_scenario(s)
            elapsed = time.monotonic() - t0
            is_xfail = s.name in XFAIL
            if passed and not is_xfail:
                status = "PASS"
            elif passed and is_xfail:
                status = "UNEXPECTED PASS"
            elif not passed and is_xfail:
                status = "XFAIL"
            else:
                status = "FAIL"
                real_failures.append((s.name, out))
            print(f"[{status:15s}] {prefix} {s.name}  ({elapsed:.1f}s)", flush=True)
            results.append((prefix, s.name, status, elapsed))

    print("\n=== SUMMARY ===", flush=True)
    for prefix, name, status, elapsed in results:
        print(f"  {prefix}  {status:15s}  {name}  ({elapsed:.1f}s)", flush=True)

    if real_failures:
        print("\n=== FAILURES ===", flush=True)
        for name, out in real_failures:
            print(f"\n--- {name} ---\n{out}", flush=True)
        return 1

    unexpected_passes = [r for r in results if r[2] == "UNEXPECTED PASS"]
    if unexpected_passes:
        print(
            f"\n[notice] {len(unexpected_passes)} XFAIL scenarios actually passed "
            "— consider promoting them out of XFAIL.", flush=True
        )

    suffix = " (xfails honoured)" if any(r[2] == "XFAIL" for r in results) else ""
    print(f"\nAll {len(results)} scenarios passed{suffix}.", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
