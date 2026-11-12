# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/a13_request_status.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-11-11
# Version:     1.0
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Conformance: ``requestStatus`` is acknowledged, then answered by a delta update.

Runs the exchange against ``status_responder.py`` and asserts the rules in
*Requesting status*:

  acknowledgement  exactly one command response, on the requester's reply node,
                   ``correlation-id`` mirroring the request's ``message-id``,
                   ``commandCorrelationId`` and ``trafficSignalId`` echoed,
                   status ``success``
  state            exactly one state message on ``traffic-signal.measurement``,
                   ``x-message-mode: delta``, no ``x-sync-*`` headers, a conformant
                   envelope, for the signal that was asked about, and valid as a
                   ``TrafficSignalDynamicState``
  declined         a publisher that does not implement it answers ``notSupported``
                   and publishes no state

Then, for each of nine ways the responder can break a rule, it requires the
checks to fail: a defective responder that passes is a failure of this gate.

    uv run python asyncapi_clients/smoketest/a13_request_status.py

Needs a broker at ``TOPAS_AMQP_URL``.
"""
from __future__ import annotations

import json
import os
import subprocess
import sys
import threading
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(ROOT))

from envelope import validate_data_headers  # noqa: E402
from topas_3000_model.traffic_signal.measurement import TrafficSignalDynamicState  # noqa: E402

PY = sys.executable
DATA_CHANNEL = "traffic-signal.measurement"
DEFECTS = [
    "no-state", "as-sync", "wrong-object", "state-to-reply", "ack-broadcast",
    "wrong-corr", "wrong-cmd-corr", "no-ack", "bad-status",
]


def _env(**extra: str) -> dict:
    env = dict(os.environ, PYTHONPATH=str(ROOT))
    env.update(extra)
    return env


def start_responder(**env: str) -> subprocess.Popen:
    proc = subprocess.Popen(
        [PY, str(HERE / "status_responder.py")], cwd=ROOT, env=_env(**env),
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, bufsize=1,
    )
    ready = threading.Event()

    def pump() -> None:
        for line in proc.stdout:  # type: ignore[union-attr]
            if line.startswith("[responder-ready]"):
                ready.set()
            elif line.startswith("[responder] FATAL"):
                print("   responder:", line.rstrip(), file=sys.stderr)

    threading.Thread(target=pump, daemon=True).start()
    if not ready.wait(30):
        proc.kill()
        raise RuntimeError("the status responder did not become ready")
    return proc


def run_requester() -> dict:
    out = subprocess.run(
        [PY, str(HERE / "status_requester.py")], cwd=ROOT, env=_env(),
        capture_output=True, text=True, timeout=60,
    ).stdout
    for line in out.splitlines():
        if line.startswith("[status-requester-result] "):
            return json.loads(line.split(" ", 1)[1])
    raise RuntimeError("the requester printed no result:\n" + out)


def check(res: dict, *, status: str = "success") -> list[str]:
    """Every way the exchange broke the specification."""
    problems: list[str] = []
    responses, states = res["responses"], res["states"]
    reply, mid = res["replyAddress"], res["messageId"]
    if not reply:
        return ["no reply node was assigned"]

    # --- acknowledgement --------------------------------------------------
    if len(responses) != 1:
        problems.append(f"expected exactly one command response on the reply node, received {len(responses)}")
    for r in responses:
        if r["to"] != reply:
            problems.append(f"response delivered with to={r['to']!r}, not the reply node {reply!r}")
        if r["correlation_id"] != mid:
            problems.append(f"correlation-id {r['correlation_id']!r} does not mirror message-id {mid!r}")
        body = r["body"]
        if body.get("commandCorrelationId") != res["commandCorrelationId"]:
            problems.append(
                f"commandCorrelationId {body.get('commandCorrelationId')!r} is not the command's "
                f"{res['commandCorrelationId']}"
            )
        if body.get("trafficSignalId") != res["signal"]:
            problems.append(f"response is for {body.get('trafficSignalId')!r}, not {res['signal']!r}")
        if body.get("status") != status:
            problems.append(f"response status is {body.get('status')!r}, expected {status!r}")

    # --- the state --------------------------------------------------------
    want = 0 if status != "success" else 1
    if len(states) != want:
        problems.append(f"expected {want} state message(s) on {DATA_CHANNEL}, received {len(states)}")
    for s in states:
        props = s["properties"]
        for err in validate_data_headers(props, DATA_CHANNEL):
            problems.append(f"envelope: {err}")
        if props.get("x-message-mode") != "delta":
            problems.append(f"state is x-message-mode {props.get('x-message-mode')!r}; it is an ordinary delta update")
        if any(k.startswith("x-sync-") for k in props):
            problems.append("a status update carries x-sync-* headers; it is not a sync")
        if s["body"].get("trafficSignalId") != res["signal"]:
            problems.append(f"state is for {s['body'].get('trafficSignalId')!r}, not {res['signal']!r}")
        try:
            TrafficSignalDynamicState.model_validate(s["body"])
        except Exception as exc:  # noqa: BLE001
            problems.append(f"state is not a valid TrafficSignalDynamicState: {exc}")
    return sorted(set(problems))


def run_one(*, status: str = "success", **env: str) -> list[str]:
    proc = start_responder(**env)
    try:
        res = run_requester()
    finally:
        proc.terminate()
        try:
            proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            proc.kill()
    return check(res, status=status)


def report(name: str, problems: list[str]) -> bool:
    print(f"  {'FAIL' if problems else 'PASS'}  {name}")
    for p in problems:
        print(f"        - {p}")
    return not problems


def main() -> int:
    print(f"[a13] broker = {os.environ.get('TOPAS_AMQP_URL', 'amqp://localhost:5672')}")
    ok = True
    print("\n=== requestStatus behaves as specified ===")
    ok &= report("supported: success on the reply node, then one delta update", run_one())
    ok &= report("declined: notSupported and no state",
                 run_one(status="notSupported", STATUS_UNSUPPORTED="1"))

    print("\n=== negative self-test: each defect must be caught ===")
    for defect in DEFECTS:
        problems = run_one(STATUS_DEFECT=defect)
        caught = bool(problems)
        ok &= caught
        print(f"  {'PASS' if caught else 'FAIL'}  {defect:16} "
              f"{'caught: ' + problems[0] if caught else 'NOT CAUGHT, the gate cannot fail'}")

    print("\nREQUESTSTATUS CONFORMANCE:", "PASS" if ok else "FAIL")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
