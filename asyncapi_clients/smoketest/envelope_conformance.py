# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/envelope_conformance.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-05
# Version:     1.0
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Conformance: every data message carries the envelope the specification requires.

Taps every data channel on a live broker, runs each smoke publisher, and
checks the application properties of every message that arrives against
``envelope.validate_data_headers`` (the ``topasEnvelope`` trait plus the rules
a JSON Schema cannot state). It then runs a publisher that breaks the envelope
in nine different ways and requires every one to be reported: a run in which
a defect passes is a failure of this checker.

    uv run python asyncapi_clients/smoketest/envelope_conformance.py

Needs a broker at ``TOPAS_AMQP_URL`` (default ``amqp://localhost:5672``).
Exit status 0 when every publisher is clean, every expected channel was seen,
and every defect was caught.
"""
from __future__ import annotations

import json
import os
import subprocess
import sys
import threading
import time
from dataclasses import dataclass, field
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(HERE))

from envelope import validate_data_headers  # noqa: E402
from envelope_defect_publisher import DEFECTS  # noqa: E402

PY = sys.executable
READY_TIMEOUT = 30.0
PUBLISHER_TIMEOUT = 40.0
DRAIN_SECONDS = 1.5


@dataclass
class Publisher:
    name: str
    script: str
    expects: set[str] | None = None  # channels that must receive at least one message
    modes: set[str] = field(default_factory=set)  # x-message-mode values that must be seen


PUBLISHERS = [
    Publisher("detector", "publisher.py", {"detector.measurement"}, {"delta"}),
    Publisher("fault", "fault_publisher.py",
              {"detector.fault", "vms.fault", "traffic-signal.fault"}, {"delta"}),
    Publisher("traffic-signal", "traffic_signal_publisher.py", {"traffic-signal.measurement"}, {"delta"}),
    Publisher("transport-link", "transport_link_publisher.py", {"transport-link.measurement"}, {"delta"}),
    Publisher("network", "network_publisher.py",
              {"network.node", "network.link", "network.zone", "network.path", "network.turn"}, {"sync"}),
    Publisher("sync-delta", "sync_delta_publisher.py", {"detector.measurement"}, {"sync", "delta"}),
]


class TapRun:
    """A running tap whose output is collected on a thread."""

    def __init__(self) -> None:
        env = dict(os.environ, PYTHONPATH=str(ROOT))
        self.proc = subprocess.Popen(
            [PY, str(HERE / "envelope_tap.py")], cwd=ROOT, env=env,
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, bufsize=1,
        )
        self.messages: list[dict] = []
        self.ready = threading.Event()
        self.lock = threading.Lock()
        threading.Thread(target=self._read, daemon=True).start()

    def _read(self) -> None:
        for line in self.proc.stdout:  # type: ignore[union-attr]
            if line.startswith("[tap-ready]"):
                self.ready.set()
            elif line.startswith("[tap] "):
                with self.lock:
                    self.messages.append(json.loads(line[6:]))
            else:
                print("   tap:", line.rstrip(), file=sys.stderr)

    def mark(self) -> int:
        with self.lock:
            return len(self.messages)

    def since(self, mark: int) -> list[dict]:
        with self.lock:
            return list(self.messages[mark:])

    def stop(self) -> None:
        self.proc.terminate()
        try:
            self.proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            self.proc.kill()


def run_publisher(script: str) -> tuple[int, str]:
    env = dict(os.environ, PYTHONPATH=str(ROOT))
    done = subprocess.run(
        [PY, str(HERE / script)], cwd=ROOT, env=env, capture_output=True, text=True,
        timeout=PUBLISHER_TIMEOUT,
    )
    return done.returncode, (done.stdout + done.stderr).strip().splitlines()[-1:] and \
        (done.stdout + done.stderr).strip().splitlines()[-1] or ""


def check_publisher(pub: Publisher, messages: list[dict]) -> list[str]:
    problems: list[str] = []
    seen_channels = {m["channel"] for m in messages}
    for channel in sorted(pub.expects or ()):
        if channel not in seen_channels:
            problems.append(f"no message reached {channel}")
    modes = {m["properties"].get("x-message-mode") for m in messages}
    for mode in sorted(pub.modes - modes):
        problems.append(f"no x-message-mode {mode!r} message was seen")
    for m in messages:
        for err in validate_data_headers(m["properties"], m["channel"]):
            problems.append(f"{m['channel']}: {err}")
    return sorted(set(problems))


def main() -> int:
    broker = os.environ.get("TOPAS_AMQP_URL", "amqp://localhost:5672")
    print(f"[envelope] broker = {broker}")
    tap = TapRun()
    failed = False
    try:
        if not tap.ready.wait(READY_TIMEOUT):
            print("FAIL: the tap did not attach to every data channel in time")
            return 1

        print("\n=== every smoke publisher emits a conformant envelope ===")
        for pub in PUBLISHERS:
            mark = tap.mark()
            code, last = run_publisher(pub.script)
            time.sleep(DRAIN_SECONDS)
            got = tap.since(mark)
            problems = check_publisher(pub, got)
            if code != 0:
                problems.insert(0, f"publisher exited {code}: {last}")
            status = "FAIL" if problems else "PASS"
            failed |= bool(problems)
            print(f"  {status}  {pub.name:15} {len(got)} messages on "
                  f"{len({m['channel'] for m in got})} channel(s)")
            for p in problems:
                print(f"        - {p}")

        print("\n=== negative self-test: each defect must be reported ===")
        mark = tap.mark()
        code, last = run_publisher("envelope_defect_publisher.py")
        time.sleep(DRAIN_SECONDS)
        got = tap.since(mark)
        if code != 0 or len(got) != len(DEFECTS):
            print(f"  FAIL  defect publisher sent {len(got)} of {len(DEFECTS)} messages (exit {code}): {last}")
            failed = True
        else:
            # Messages arrive in send order, so the nth message is the nth defect.
            for (name, _props), msg in zip(DEFECTS.items(), got):
                errs = validate_data_headers(msg["properties"], msg["channel"])
                ok = bool(errs)
                failed |= not ok
                print(f"  {'PASS' if ok else 'FAIL'}  {name:32} "
                      f"{'reported: ' + errs[0] if ok else 'NOT REPORTED, the checker cannot fail'}")
    finally:
        tap.stop()

    print("\nENVELOPE CONFORMANCE:", "FAIL" if failed else "PASS")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
