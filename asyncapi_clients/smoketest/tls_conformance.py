# ──────────────────────────────────────────────────────────────────────
# File:        smoketest/tls_conformance.py
# Origin:      Hand-written for asyncapi-clients conformance testing
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-05
# Version:     1.0
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Conformance: the Broker speaks TLS 1.3 with mutual authentication, and nothing less.

AC5-4 requires transport encryption to use TLS 1.3 and forbids a Broker
to offer TLS 1.2 or earlier; AC5 and *Mutual TLS* require every participant to
authenticate with a client certificate. The ordinary smoke broker is plain TCP,
so none of this was evidenced. This runs against a broker built to the
specification (``docker-compose.tls.yml``) and asserts, at the handshake and
then through AMQP:

  accepted    a TLS 1.3 client with a certificate the broker trusts completes
              the handshake, negotiates ``TLSv1.3``, and gets an AMQP header back
  tls12       a client that will only speak TLS 1.2 is refused
  no cert     a TLS 1.3 client with no certificate is refused
  untrusted   a TLS 1.3 client whose certificate comes from another CA is refused
  amqp        an AMQP message round-trips over the mutual-TLS 1.3 connection,
              so the broker works over this path and is not merely listening

"Refused" is judged by what the client can do afterwards, not by the handshake
alone: in TLS 1.3 a server may reject a client certificate *after* the client's
handshake has completed, so the probe sends an AMQP header and requires the
broker's answer.

A check that cannot fail is not a check, so it then runs the same
checks against in-process servers that are deliberately wrong (one that still
offers TLS 1.2, one that asks for no client certificate) and requires each
to be reported.

    asyncapi_clients/smoketest/tls/make_certs.sh asyncapi_clients/smoketest/tls/certs
    docker compose -f asyncapi_clients/smoketest/docker-compose.tls.yml up -d
    uv run python asyncapi_clients/smoketest/tls_conformance.py

Environment: ``TOPAS_TLS_HOST`` (localhost), ``TOPAS_TLS_PORT`` (5675),
``TOPAS_TLS_DIR`` (``tls/certs`` beside this file).
"""
from __future__ import annotations

import os
import socket
import ssl
import sys
import threading
import uuid
from dataclasses import dataclass
from pathlib import Path

HERE = Path(__file__).resolve().parent
HOST = os.environ.get("TOPAS_TLS_HOST", "localhost")
PORT = int(os.environ.get("TOPAS_TLS_PORT", "5675"))
CERTS = Path(os.environ.get("TOPAS_TLS_DIR", HERE / "tls" / "certs"))
AMQP_HEADER = b"AMQP\x00\x01\x00\x00"
TIMEOUT = 8.0


@dataclass
class Outcome:
    """What a client could do against a server."""

    accepted: bool
    version: str | None = None
    detail: str = ""


def client_context(
    *, minimum=ssl.TLSVersion.TLSv1_3, maximum=ssl.TLSVersion.TLSv1_3, cert: str | None = "client",
) -> ssl.SSLContext:
    ctx = ssl.SSLContext(ssl.PROTOCOL_TLS_CLIENT)
    ctx.load_verify_locations(CERTS / "ca.pem")
    ctx.minimum_version = minimum
    ctx.maximum_version = maximum
    if cert:
        ctx.load_cert_chain(CERTS / f"{cert}.pem", CERTS / f"{cert}.key")
    return ctx


def probe(ctx: ssl.SSLContext, host: str = HOST, port: int = PORT) -> Outcome:
    """Handshake, then ask for the AMQP header. Accepted only if both work."""
    try:
        with socket.create_connection((host, port), timeout=TIMEOUT) as raw:
            with ctx.wrap_socket(raw, server_hostname="localhost") as tls:
                tls.settimeout(TIMEOUT)
                version = tls.version()
                tls.sendall(AMQP_HEADER)
                answer = tls.recv(8)
                if answer[:4] != b"AMQP":
                    return Outcome(False, version, f"handshake completed but the broker answered {answer!r}")
                return Outcome(True, version)
    except (ssl.SSLError, OSError, socket.timeout) as exc:
        return Outcome(False, None, f"{type(exc).__name__}: {exc}")


# ── the specification's requirements, as checks that return problems ───────

def check_tls13_mtls_accepted(host=HOST, port=PORT) -> list[str]:
    out = probe(client_context(), host, port)
    if not out.accepted:
        return [f"a TLS 1.3 client with a trusted certificate was refused ({out.detail})"]
    if out.version != "TLSv1.3":
        return [f"negotiated {out.version}, not TLSv1.3"]
    return []


def check_tls12_refused(host=HOST, port=PORT) -> list[str]:
    ctx = client_context(minimum=ssl.TLSVersion.TLSv1_2, maximum=ssl.TLSVersion.TLSv1_2)
    out = probe(ctx, host, port)
    return [f"a TLS 1.2-only client was accepted (negotiated {out.version}); AC5-4 forbids offering it"] if out.accepted else []


def check_no_client_cert_refused(host=HOST, port=PORT) -> list[str]:
    out = probe(client_context(cert=None), host, port)
    return ["a client with no certificate was accepted; mutual TLS requires one"] if out.accepted else []


def check_untrusted_cert_refused(host=HOST, port=PORT) -> list[str]:
    out = probe(client_context(cert="rogue-client"), host, port)
    return ["a client certificate from an untrusted CA was accepted"] if out.accepted else []


def check_amqp_round_trip() -> list[str]:
    """Send a message to an address and receive it, over mutual TLS 1.3."""
    from proton import Message, SSLDomain
    from proton.handlers import MessagingHandler
    from proton.reactor import Container

    address = f"tls.conformance.{uuid.uuid4().hex[:8]}"
    token = uuid.uuid4().hex
    result: dict = {}

    class RoundTrip(MessagingHandler):
        def on_start(self, event):
            dom = SSLDomain(SSLDomain.MODE_CLIENT)
            dom.set_trusted_ca_db(str(CERTS / "ca.pem"))
            dom.set_peer_authentication(SSLDomain.VERIFY_PEER_NAME)
            dom.set_credentials(str(CERTS / "client.pem"), str(CERTS / "client.key"), "")
            conn = event.container.connect(f"amqps://{HOST}:{PORT}", ssl_domain=dom)
            self.receiver = event.container.create_receiver(conn, address)
            self.sender = event.container.create_sender(conn, address)
            event.container.schedule(TIMEOUT, self)

        def on_link_opened(self, event):
            # Send only once the receiver is attached, or a multicast address drops it.
            if event.receiver is not None:
                self.sender.send(Message(body=token))

        def on_message(self, event):
            result["body"] = event.message.body
            event.connection.close()

        def on_transport_error(self, event):
            # A transport error has no connection to close (event.connection is None).
            result["error"] = str(event.transport.condition)
            event.container.stop()

        def on_connection_error(self, event):
            result["error"] = str(event.connection.remote_condition)

        def on_timer_task(self, event):
            result.setdefault("error", "timed out")
            event.container.stop()

    try:
        Container(RoundTrip()).run()
    except Exception as exc:  # noqa: BLE001 - report any client failure as a problem
        return [f"AMQP client failed: {type(exc).__name__}: {exc}"]
    if result.get("body") != token:
        return [f"no AMQP round trip over mutual TLS 1.3 ({result.get('error', 'wrong body')})"]
    return []


REQUIREMENTS = [
    ("TLS 1.3 with a trusted client certificate is accepted", check_tls13_mtls_accepted),
    ("TLS 1.2 is refused (AC5-4)", check_tls12_refused),
    ("no client certificate is refused", check_no_client_cert_refused),
    ("a client certificate from an untrusted CA is refused", check_untrusted_cert_refused),
]


# ── control servers: deliberately wrong, to prove the checks can fail ──────

class ControlServer:
    """An in-process TLS server that speaks just enough AMQP to be probed."""

    def __init__(self, *, minimum, require_client_cert: bool) -> None:
        ctx = ssl.SSLContext(ssl.PROTOCOL_TLS_SERVER)
        ctx.load_cert_chain(CERTS / "broker.pem", CERTS / "broker.key")
        ctx.minimum_version = minimum
        ctx.maximum_version = ssl.TLSVersion.TLSv1_3
        if require_client_cert:
            ctx.load_verify_locations(CERTS / "ca.pem")
            ctx.verify_mode = ssl.CERT_REQUIRED
        else:
            ctx.verify_mode = ssl.CERT_NONE
        self.ctx = ctx
        self.sock = socket.socket()
        self.sock.bind(("127.0.0.1", 0))
        self.sock.listen(8)
        self.port = self.sock.getsockname()[1]
        self.stopping = False
        threading.Thread(target=self._serve, daemon=True).start()

    def _serve(self) -> None:
        while not self.stopping:
            try:
                conn, _ = self.sock.accept()
            except OSError:
                return
            threading.Thread(target=self._handle, args=(conn,), daemon=True).start()

    def _handle(self, conn) -> None:
        try:
            conn.settimeout(TIMEOUT)
            with self.ctx.wrap_socket(conn, server_side=True) as tls:
                if tls.recv(8)[:4] == b"AMQP":
                    tls.sendall(AMQP_HEADER)
        except (ssl.SSLError, OSError):
            pass

    def stop(self) -> None:
        self.stopping = True
        self.sock.close()


CONTROLS = [
    ("a server that still offers TLS 1.2", dict(minimum=ssl.TLSVersion.TLSv1_2, require_client_cert=True),
     check_tls12_refused),
    ("a server that asks for no client certificate", dict(minimum=ssl.TLSVersion.TLSv1_3, require_client_cert=False),
     check_no_client_cert_refused),
    ("a server that accepts any client certificate", dict(minimum=ssl.TLSVersion.TLSv1_3, require_client_cert=False),
     check_untrusted_cert_refused),
]


def main() -> int:
    if not (CERTS / "ca.pem").exists():
        print(f"FAIL: no certificates in {CERTS}; run tls/make_certs.sh {CERTS} first")
        return 1
    ok = True
    print(f"[tls] broker = {HOST}:{PORT}  certificates = {CERTS}")

    print("\n=== the Broker meets AC5-4 and mutual TLS ===")
    for name, check in REQUIREMENTS:
        problems = check()
        ok &= not problems
        print(f"  {'FAIL' if problems else 'PASS'}  {name}")
        for p in problems:
            print(f"        - {p}")
    problems = check_amqp_round_trip()
    ok &= not problems
    print(f"  {'FAIL' if problems else 'PASS'}  an AMQP message round-trips over mutual TLS 1.3")
    for p in problems:
        print(f"        - {p}")

    print("\n=== negative self-test: a wrong server must be reported ===")
    for name, kwargs, check in CONTROLS:
        server = ControlServer(**kwargs)
        try:
            problems = check("127.0.0.1", server.port)
        finally:
            server.stop()
        caught = bool(problems)
        ok &= caught
        print(f"  {'PASS' if caught else 'FAIL'}  {name:46} "
              f"{'reported: ' + problems[0] if caught else 'NOT REPORTED, the check cannot fail'}")

    print("\nTLS CONFORMANCE:", "PASS" if ok else "FAIL")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
