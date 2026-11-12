# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/a11_authenticated.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-08-29
# Version:     1.1
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""What the authentication boundary actually provides.

This leg exists to answer one question: can an AMQP consumer obtain an
identity it did not have to trust the sender for?

On the reference broker the answer is **no**, and that is the finding.
Authentication itself is enforced — anonymous and bad credentials are
both refused at connect — but Artemis 2.31.2's `populate-validated-user`
does not reach an AMQP consumer, and the AMQP `user-id` field arrives
exactly as the sender set it.

So these scenarios assert the *environment*, not the application:

    authentication-enforced      anonymous cannot connect
    credentials-enforced         a bad password cannot connect
    broker-identity-unavailable  no broker-asserted identity is delivered,
                                 and `user-id` is forgeable

The third is a regression test on an assumption rather than on our code.
If a future broker starts validating `user-id`, it goes red and someone
re-reads the identity — which is the outcome we want, because that would be the
moment `BrokerValidatedUserProvider` becomes usable.

Run against the authenticated broker (docker-compose.auth.yml, port
5674), not the anonymous one.
"""
from __future__ import annotations

import argparse
import os
import subprocess
import sys
import textwrap
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
PY_EXE = sys.executable
BROKER_HOST = os.environ.get("TOPAS_AUTH_BROKER_HOST", "localhost")
BROKER_PORT = os.environ.get("TOPAS_AUTH_BROKER_PORT", "5674")
MGMT_PORT = os.environ.get("TOPAS_AUTH_MGMT_PORT", "8162")


def _connect_script(url: str, extra: str = "") -> str:
    return textwrap.dedent(f"""
        from proton import Message
        from proton.handlers import MessagingHandler
        from proton.reactor import Container

        class P(MessagingHandler):
            def __init__(self):
                super().__init__(); self.sent = False
            def on_start(self, e):
                self.container = e.container
                # reconnect=False: proton otherwise retries an auth failure
                # forever, so a refused connection would look like a hang
                # rather than a refusal.
                c = e.container.connect({url!r}, reconnect=False)
                e.container.create_receiver(c, "auth.probe")
                e.container.create_sender(c, "auth.probe")
            def on_sendable(self, e):
                if not self.sent:
                    m = Message(body="x")
                    {extra}
                    e.sender.send(m); self.sent = True
            def on_message(self, e):
                m = e.message
                print("DELIVERED", flush=True)
                print("user_id=%r" % (m.user_id,), flush=True)
                print("properties=%r" % (m.properties,), flush=True)
                print("annotations=%r" % (m.annotations,), flush=True)
                e.connection.close()
            def on_connection_error(self, e):
                print("REFUSED %s" % (e.connection.remote_condition,), flush=True)
                e.connection.close()
                self.container.stop()
            def on_transport_error(self, e):
                print("REFUSED %s" % (e.transport.condition,), flush=True)
                self.container.stop()

        Container(P()).run()
    """)


def _run(script: str, timeout: int = 30) -> str:
    proc = subprocess.run(
        [PY_EXE, "-u", "-c", script],
        cwd=str(ROOT), stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
        text=True, timeout=timeout,
    )
    return proc.stdout


def _url(user: str | None = None, password: str | None = None) -> str:
    creds = f"{user}:{password}@" if user else ""
    return f"amqp://{creds}{BROKER_HOST}:{BROKER_PORT}"


def scenario_authentication_enforced() -> bool:
    """Anonymous must be refused, or nothing below means anything."""
    print("=== auth: anonymous connections are refused ===")
    out = _run(_connect_script(_url()))
    refused = "REFUSED" in out and "unauthorized-access" in out
    print(f"  anonymous: {'refused' if refused else 'ACCEPTED'}")
    if not refused:
        print("    - the broker is not enforcing authentication; with security")
        print("      disabled Artemis takes the username the client supplied,")
        print("      so nothing here would be broker-asserted")
    return refused


def scenario_credentials_enforced() -> bool:
    """A bad password must fail at the transport, not reach the application."""
    print("=== auth: invalid credentials are refused ===")
    out = _run(_connect_script(_url("user-a", "WRONG-PASSWORD")))
    refused = "REFUSED" in out and "unauthorized-access" in out
    delivered = "DELIVERED" in out
    print(f"  bad password: {'refused' if refused else 'ACCEPTED'}")
    print(f"  reached the application: {'yes' if delivered else 'no'}")
    ok = refused and not delivered
    if not ok:
        print("    - authentication failure must fail closed at the transport,")
        print("      a layer below the principal provider")
    return ok


def scenario_broker_identity_unavailable() -> bool:
    """Assert the environmental fact the design rests on.

    Both users authenticate for real. If the broker asserted an identity,
    a forged `user-id` would come back overwritten. It does not — so an
    AMQP consumer here has no identity it did not take from the sender,
    and `BrokerValidatedUserProvider` cannot be used on this broker.
    """
    print("=== auth: the broker asserts no identity to an AMQP consumer ===")
    forged = "principal-b-FORGED"
    out = _run(_connect_script(
        _url("user-a", "pw-a"), extra=f"m.user_id = {forged!r}.encode()"
    ))
    delivered = "DELIVERED" in out
    print(f"  authenticated as user-a and delivered: {'yes' if delivered else 'no'}")
    if not delivered:
        print("    - could not deliver as an authenticated user")
        return False

    survived = forged in out
    stamped = "_AMQ_VALIDATED_USER" in out
    print(f"  forged user-id survived unmodified: {'yes' if survived else 'no'}")
    print(f"  _AMQ_VALIDATED_USER present:        {'yes' if stamped else 'no'}")
    print()
    print("  This is the recorded state of the reference broker, not a defect")
    print("  in the SDKs. populate-validated-user is enabled and does not")
    print("  reach an AMQP consumer, and user-id is client-controlled — so a")
    print("  responder MUST NOT treat either as an authenticated identity.")
    print("  If this scenario ever fails, the environment has changed and")
    print("  the identity handling should be re-read: it would mean a broker-asserted")
    print("  principal has become available.")
    ok = survived and not stamped
    if not ok:
        print("    - the broker's identity behaviour has changed; re-read the identity handling")
    return ok


def scenario_broker_knows_principal() -> bool:
    """The broker *does* hold the authenticated identity.

    This is the scenario that makes the finding precise. Without it the
    result reads as "authentication tells you nothing", which is wrong and
    would point implementers at the wrong fix. The broker authenticates
    the producer and records who it is; what it does not do is put that
    identity into the message an AMQP consumer receives.

    So the deficiency is in **propagation**, not authentication — and the
    remedies are correspondingly different: a broker interceptor that
    stamps a trusted property, or a trusted ingress, rather than anything
    the client could be asked to send.
    """
    print("=== auth: the broker holds the authenticated principal ===")
    import base64
    import json
    import threading
    import urllib.request

    held = threading.Event()
    stop = threading.Event()

    def hold():
        from proton.handlers import MessagingHandler
        from proton.reactor import Container

        class H(MessagingHandler):
            def on_start(self, e):
                self.container = e.container
                c = e.container.connect(_url("user-a", "pw-a"), reconnect=False)
                e.container.create_receiver(c, "auth.knows")

            def on_link_opened(self, e):
                held.set()

            def on_timer_task(self, e):
                if stop.is_set():
                    self.container.stop()
                else:
                    e.reactor.schedule(0.2, self)

            def on_connection_opened(self, e):
                e.reactor.schedule(0.2, self)

        Container(H()).run()

    t = threading.Thread(target=hold, daemon=True)
    t.start()
    if not held.wait(timeout=30):
        print("  could not hold an authenticated session")
        return False

    try:
        auth = base64.b64encode(b"admin:admin").decode()
        url = (f"http://localhost:{MGMT_PORT}/console/jolokia/exec/"
               'org.apache.activemq.artemis:broker="0.0.0.0"/listAllSessionsAsJSON')
        req = urllib.request.Request(url, headers={
            "Origin": "http://localhost",
            "Authorization": f"Basic {auth}",
        })
        with urllib.request.urlopen(req, timeout=20) as resp:
            payload = json.load(resp)
        value = payload.get("value")
        sessions = json.loads(value) if isinstance(value, str) else value
        users = sorted({s.get("validatedUser") for s in (sessions or [])
                        if s.get("validatedUser")})
    except Exception as exc:  # noqa: BLE001 - reported, not swallowed
        print(f"  management query failed: {exc}")
        return False
    finally:
        stop.set()

    print(f"  sessions reported: {len(sessions or [])}")
    print(f"  validatedUser:     {users}")
    ok = "user-a" in users
    if ok:
        print()
        print("  The broker authenticated the producer and holds its identity.")
        print("  It simply does not place that identity in the message an AMQP")
        print("  consumer receives — so the gap is propagation, not")
        print("  authentication, and the fix belongs in the broker or an")
        print("  ingress, never in something the client sends.")
    else:
        print("    - the broker did not report a validated user for a live")
        print("      authenticated session; re-read the identity handling")
    return ok


SCENARIOS = {
    "authentication-enforced": scenario_authentication_enforced,
    "broker-knows-principal": scenario_broker_knows_principal,
    "credentials-enforced": scenario_credentials_enforced,
    "broker-identity-unavailable": scenario_broker_identity_unavailable,
}


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--only", choices=sorted(SCENARIOS))
    args = ap.parse_args()
    print(f"[auth] broker = {_url()}")
    names = [args.only] if args.only else list(SCENARIOS)
    results = {}
    for name in names:
        results[name] = SCENARIOS[name]()
        print()
    print("=== SUMMARY ===")
    for name, ok in results.items():
        print(f"  {'PASS' if ok else 'FAIL'}  {name}")
    ok = all(results.values())
    print(f"\nA11 AUTHENTICATED IDENTITY: {'PASS' if ok else 'FAIL'}")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
