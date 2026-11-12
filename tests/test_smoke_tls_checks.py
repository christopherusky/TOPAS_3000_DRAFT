# ──────────────────────────────────────────────────────────────────────
#
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-05
# Version:     1.0
# Description: The TLS checks (smoketest/tls_conformance.py) against in-process
#              servers, with no broker: a server built to AC5-4 passes every
#              check and a server that is wrong in one way fails exactly that
#              check. Needs openssl and keytool to make throwaway certificates;
#              skipped where they are absent.
# License:     Apache-2.0 — https://opensource.org/license/Apache-2.0
# ──────────────────────────────────────────────────────────────────────
"""Tests for the TLS conformance checks."""

import shutil
import ssl
import subprocess
import sys
from pathlib import Path

import pytest

SMOKE = Path(__file__).resolve().parent.parent / "asyncapi_clients" / "smoketest"
sys.path.insert(0, str(SMOKE))

pytestmark = pytest.mark.skipif(
    not (shutil.which("openssl") and shutil.which("keytool")),
    reason="openssl and keytool are needed to make the test certificates",
)

import tls_conformance as tc  # noqa: E402


@pytest.fixture(scope="module")
def certs(tmp_path_factory):
    out = tmp_path_factory.mktemp("tlscerts")
    subprocess.run(["bash", str(SMOKE / "tls" / "make_certs.sh"), str(out)], check=True,
                   capture_output=True)
    previous = tc.CERTS
    tc.CERTS = out
    yield out
    tc.CERTS = previous


def serve(**kwargs):
    return tc.ControlServer(**kwargs)


CORRECT = dict(minimum=ssl.TLSVersion.TLSv1_3, require_client_cert=True)


def test_a_server_built_to_ac5_4_passes_every_check(certs):
    server = serve(**CORRECT)
    try:
        assert tc.check_tls13_mtls_accepted("127.0.0.1", server.port) == []
        assert tc.check_tls12_refused("127.0.0.1", server.port) == []
        assert tc.check_no_client_cert_refused("127.0.0.1", server.port) == []
        assert tc.check_untrusted_cert_refused("127.0.0.1", server.port) == []
    finally:
        server.stop()


def test_a_server_that_still_offers_tls12_fails_that_check_only(certs):
    server = serve(minimum=ssl.TLSVersion.TLSv1_2, require_client_cert=True)
    try:
        assert tc.check_tls12_refused("127.0.0.1", server.port)
        assert tc.check_tls13_mtls_accepted("127.0.0.1", server.port) == []
        assert tc.check_no_client_cert_refused("127.0.0.1", server.port) == []
    finally:
        server.stop()


def test_a_server_asking_for_no_client_certificate_fails_the_certificate_checks(certs):
    server = serve(minimum=ssl.TLSVersion.TLSv1_3, require_client_cert=False)
    try:
        assert tc.check_no_client_cert_refused("127.0.0.1", server.port)
        assert tc.check_untrusted_cert_refused("127.0.0.1", server.port)
        assert tc.check_tls12_refused("127.0.0.1", server.port) == []
    finally:
        server.stop()


def test_an_unreachable_server_is_a_refusal_not_a_pass(certs):
    """A closed port must not read as 'TLS 1.2 refused' for the right reason without a
    TLS 1.3 client also being turned away: the positive check fails there."""
    assert tc.check_tls13_mtls_accepted("127.0.0.1", 1)
