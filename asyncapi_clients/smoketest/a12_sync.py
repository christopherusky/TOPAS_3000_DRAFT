# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/a12_sync.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-05
# Version:     1.0
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""A12 conformance: a requested sync is addressed, ordered and closed.

*State Synchronization* makes ``sync-request`` a MUST and states how the
exchange behaves. Until now only the model classes were tested; no broker
exchange was. This runs the exchange and asserts each rule the specification
gives:

  addressing     the batch and the ``SyncComplete`` reach the requester's
                 reply node, ``correlation-id`` mirrors the request's
                 ``message-id``, and nothing is published to a data channel
  envelope       every batch message carries a conformant data envelope in
                 ``sync`` mode, one ``x-sync-id``, a 1..N ``x-sync-sequence``,
                 ``x-sync-total`` = N, and ``x-sync-request-id`` echoing the
                 request's ``syncCorrelationId``
  ending         exactly one ``SyncComplete``, after the last object, echoing
                 the request, carrying the batch's ``syncId`` and the right
                 ``objectCount``; nothing arrives after it
  filters        ``objectIds`` selects a subset; one matching nothing still
                 gets a ``SyncComplete`` (count 0, no ``syncId``)
  declined       a ``rejected`` ``SyncComplete`` and no batch
  isolation      two concurrent requesters each receive only their own batch

Then, for each of ten ways the responder can break a rule, it requires the
checks above to fail: a defective responder that passes is a failure of this
gate.

    uv run python asyncapi_clients/smoketest/a12_sync.py

Needs a broker at ``TOPAS_AMQP_URL``. Exit 0 only if every scenario passes and
every defect is caught. The reply-to authorisation MUST (the responder must
refuse a ``reply-to`` the requester may not receive from) needs a broker with
security enabled and is not exercised here.
"""
from __future__ import annotations

import json
import os
import subprocess
import sys
import threading
import time
from pathlib import Path

HERE = Path(__file__).resolve().parent
ROOT = HERE.parents[1]
sys.path.insert(0, str(HERE))

from envelope import validate_data_headers  # noqa: E402
from envelope_conformance import TapRun  # noqa: E402

PY = sys.executable
DATA_CHANNEL = "detector.measurement"
ALL = ["DET-0001", "DET-0002", "DET-0003"]
DEFECTS = [
    "broadcast", "wrong-total", "no-request-id", "wrong-request-id", "wrong-corr",
    "bad-sequence", "complete-early", "wrong-count", "wrong-sync-id", "no-complete",
]


def _env(**extra: str) -> dict:
    env = dict(os.environ, PYTHONPATH=str(ROOT))
    env.update(extra)
    return env


class Responder:
    def __init__(self, **env: str) -> None:
        self.proc = subprocess.Popen(
            [PY, str(HERE / "sync_responder.py")], cwd=ROOT, env=_env(**env),
            stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True, bufsize=1,
        )
        ready = threading.Event()

        def pump() -> None:
            for line in self.proc.stdout:  # type: ignore[union-attr]
                if line.startswith("[responder-ready]"):
                    ready.set()
                elif line.startswith("[responder] FATAL"):
                    print("   responder:", line.rstrip(), file=sys.stderr)

        threading.Thread(target=pump, daemon=True).start()
        if not ready.wait(30):
            self.stop()
            raise RuntimeError("the sync responder did not become ready")

    def stop(self) -> None:
        self.proc.terminate()
        try:
            self.proc.wait(timeout=5)
        except subprocess.TimeoutExpired:
            self.proc.kill()


def request(label: str = "requester", timeout: int = 10, **env: str) -> subprocess.Popen:
    return subprocess.Popen(
        [PY, str(HERE / "sync_requester.py")], cwd=ROOT,
        env=_env(SYNC_LABEL=label, SYNC_TIMEOUT=str(timeout), **env),
        stdout=subprocess.PIPE, stderr=subprocess.STDOUT, text=True,
    )


def result_of(proc: subprocess.Popen) -> dict:
    out, _ = proc.communicate(timeout=60)
    for line in out.splitlines():
        if line.startswith("[sync-requester-result] "):
            return json.loads(line.split(" ", 1)[1])
    raise RuntimeError("the requester printed no result:\n" + out)


def check(res: dict, expect_ids: list[str], *, status: str = "success") -> list[str]:
    """Every way one requester's exchange broke the specification."""
    problems: list[str] = []
    frames, completes = res["frames"], res["completes"]
    reply, mid, cid = res["replyAddress"], res["messageId"], res["syncCorrelationId"]
    n = len(expect_ids)

    if not reply:
        return ["no reply node was assigned"]
    if len(completes) != 1:
        problems.append(f"expected exactly one SyncComplete, received {len(completes)}")
    if len(frames) != n:
        problems.append(f"expected {n} batch message(s), received {len(frames)}")

    # --- addressing -----------------------------------------------------
    for item in frames + completes:
        if item["to"] != reply:
            problems.append(f"delivered with to={item['to']!r}, not the reply node {reply!r}")
            break
    for item in frames + completes:
        if item["correlation_id"] != mid:
            problems.append(f"correlation-id {item['correlation_id']!r} does not mirror message-id {mid!r}")
            break

    # --- the batch ------------------------------------------------------
    sync_ids = {f["properties"].get("x-sync-id") for f in frames}
    if len(sync_ids) > 1:
        problems.append(f"the batch carries {len(sync_ids)} different x-sync-id values")
    for f in frames:
        for err in validate_data_headers(f["properties"], DATA_CHANNEL):
            problems.append(f"envelope: {err}")
        if f["properties"].get("x-message-mode") != "sync":
            problems.append("a batch message is not x-message-mode 'sync'")
        if f["properties"].get("x-sync-request-id") != cid:
            problems.append(
                f"x-sync-request-id {f['properties'].get('x-sync-request-id')!r} does not echo "
                f"syncCorrelationId {cid}"
            )
    seqs = [f["properties"].get("x-sync-sequence") for f in frames]
    if seqs != list(range(1, n + 1)):
        problems.append(f"x-sync-sequence is {seqs}, expected {list(range(1, n + 1))}")
    totals = {f["properties"].get("x-sync-total") for f in frames}
    if frames and totals != {n}:
        problems.append(f"x-sync-total is {sorted(map(str, totals))}, expected {n}")
    got_ids = sorted(f["body"].get("detectorId") for f in frames)
    if got_ids != sorted(expect_ids):
        problems.append(f"objects received {got_ids}, expected {sorted(expect_ids)}")

    # --- the ending -----------------------------------------------------
    for done in completes[:1]:
        body = done["body"]
        if body.get("syncCorrelationId") != cid:
            problems.append(f"SyncComplete echoes syncCorrelationId {body.get('syncCorrelationId')}, not {cid}")
        if body.get("status") != status:
            problems.append(f"SyncComplete status is {body.get('status')!r}, expected {status!r}")
        if body.get("objectCount") != n:
            problems.append(f"SyncComplete objectCount is {body.get('objectCount')}, expected {n}")
        if n and body.get("syncId") not in sync_ids:
            problems.append(f"SyncComplete syncId {body.get('syncId')!r} is not the batch's x-sync-id")
        if not n and body.get("syncId") is not None:
            problems.append("an empty or declined sync's SyncComplete must not carry a syncId")
    # `after_complete` is True for anything received once a SyncComplete had arrived, so a batch
    # message with it set came after the end of the batch.
    if any(f["after_complete"] for f in frames):
        problems.append("a batch message arrived after the SyncComplete")
    return sorted(set(problems))


def run_one(label: str, expect: list[str], *, status="success", req_env=None, resp_env=None,
            tap: TapRun | None = None) -> list[str]:
    """One requester against one fresh responder. Returns problems (tap leaks included)."""
    mark = tap.mark() if tap else 0
    responder = Responder(**(resp_env or {}))
    try:
        res = result_of(request(label, 6 if resp_env else 10, **(req_env or {})))
        time.sleep(0.5)
    finally:
        responder.stop()
    problems = check(res, expect, status=status)
    if tap:
        leaked = [m for m in tap.since(mark) if "x-sync-request-id" in m["properties"]]
        if leaked:
            problems.append(
                f"{len(leaked)} requested-sync message(s) were published to a data channel "
                f"({sorted({m['channel'] for m in leaked})}); a requested sync is addressed to the requester"
            )
    return problems


def report(name: str, problems: list[str]) -> bool:
    print(f"  {'FAIL' if problems else 'PASS'}  {name}")
    for p in problems:
        print(f"        - {p}")
    return not problems


def main() -> int:
    print(f"[a12] broker = {os.environ.get('TOPAS_AMQP_URL', 'amqp://localhost:5672')}")
    tap = TapRun()
    ok = True
    try:
        if not tap.ready.wait(30):
            print("FAIL: the data-channel tap did not attach")
            return 1

        print("\n=== a requested sync behaves as specified ===")
        ok &= report("complete: no filter returns every object, closed by SyncComplete",
                     run_one("all", ALL, tap=tap))
        ok &= report("filtered: objectIds selects a subset",
                     run_one("subset", ["DET-0002"], req_env={"SYNC_OBJECT_IDS": "DET-0002"}, tap=tap))
        ok &= report("empty: a filter matching nothing still gets a SyncComplete (count 0, no syncId)",
                     run_one("empty", [], req_env={"SYNC_OBJECT_IDS": "NO-SUCH"}, tap=tap))
        ok &= report("category: a category the publisher does not hold returns an empty sync",
                     run_one("category", [], req_env={"SYNC_CATEGORIES": "fault"}, tap=tap))
        ok &= report("declined: a rejected SyncComplete and no batch",
                     run_one("declined", [], status="rejected", resp_env={"SYNC_REJECT": "1"}, tap=tap))

        # Isolation: two requesters at once against one responder.
        responder = Responder()
        mark = tap.mark()
        try:
            a = request("A", 10, SYNC_OBJECT_IDS="DET-0001")
            b = request("B", 10, SYNC_OBJECT_IDS="DET-0003")
            ra, rb = result_of(a), result_of(b)
            time.sleep(0.5)
        finally:
            responder.stop()
        problems = [f"A: {p}" for p in check(ra, ["DET-0001"])] + [f"B: {p}" for p in check(rb, ["DET-0003"])]
        if ra["replyAddress"] == rb["replyAddress"]:
            problems.append("both requesters were given the same reply node")
        if any(m for m in tap.since(mark) if "x-sync-request-id" in m["properties"]):
            problems.append("a requested sync reached a data channel")
        ok &= report("isolation: two concurrent requesters each receive only their own batch", problems)

        print("\n=== negative self-test: each defect must be caught ===")
        for defect in DEFECTS:
            problems = run_one(f"defect {defect}", ALL, resp_env={"SYNC_DEFECT": defect}, tap=tap)
            caught = bool(problems)
            ok &= caught
            print(f"  {'PASS' if caught else 'FAIL'}  {defect:18} "
                  f"{'caught: ' + problems[0] if caught else 'NOT CAUGHT, the gate cannot fail'}")
    finally:
        tap.stop()

    print("\nA12 SYNC CONFORMANCE:", "PASS" if ok else "FAIL")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
