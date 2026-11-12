#!/usr/bin/env bash
# File:        smoketest/tls/make_certs.sh
# Origin:      Hand-written for asyncapi-clients conformance testing
# Author:      Christopher Kettell, TRL Software
# Date:        2026-10-05
# License:     Apache-2.0  (https://opensource.org/license/Apache-2.0)
#
# Throwaway certificates for the TLS conformance broker. TEST ONLY: the
# password is public and the keys are never to be reused anywhere.
#
#   make_certs.sh <output-dir>
#
# Writes into <output-dir>:
#   ca.pem                   the CA that signs the broker and the good client
#   broker.p12               the broker key and certificate (CN=localhost,
#                            SAN localhost and 127.0.0.1)
#   truststore.p12           the broker trust store: the CA only
#   broker.pem / broker.key  the same broker key and certificate as PEM, for the in-process control server
#   client.pem / client.key  a client certificate signed by that CA
#   rogue-client.pem / .key  a client certificate from a CA the broker does NOT trust
#
# Needs openssl and keytool. Files are world-readable so a container user can
# read the bind mount.
set -euo pipefail

out="${1:?usage: make_certs.sh <output-dir>}"
pass=changeit
mkdir -p "$out"
cd "$out"
rm -f ./*.pem ./*.key ./*.p12 ./*.csr ./*.srl ./*.cnf

mkca() {  # mkca <name>
  openssl req -x509 -newkey rsa:2048 -nodes -days 2 -subj "/CN=$1" \
    -keyout "$1.key" -out "$1.pem" 2>/dev/null
}
mkleaf() {  # mkleaf <name> <ca> <cn> [san]
  local name=$1 ca=$2 cn=$3 san=${4:-}
  openssl req -newkey rsa:2048 -nodes -subj "/CN=$cn" -keyout "$name.key" -out "$name.csr" 2>/dev/null
  printf 'basicConstraints=CA:FALSE\n%s\n' "${san:+subjectAltName=$san}" > "$name.cnf"
  openssl x509 -req -in "$name.csr" -CA "$ca.pem" -CAkey "$ca.key" -CAcreateserial \
    -days 2 -extfile "$name.cnf" -out "$name.pem" 2>/dev/null
}

mkca ca
mkca rogue-ca
mkleaf broker ca localhost "DNS:localhost,IP:127.0.0.1"
mkleaf client ca topas-client
mkleaf rogue-client rogue-ca topas-rogue

openssl pkcs12 -export -inkey broker.key -in broker.pem -certfile ca.pem \
  -name broker -passout "pass:$pass" -out broker.p12
keytool -importcert -noprompt -alias ca -file ca.pem -keystore truststore.p12 \
  -storetype PKCS12 -storepass "$pass" >/dev/null 2>&1

rm -f ./*.csr ./*.srl ./*.cnf ca.key rogue-ca.key rogue-ca.pem
chmod 644 ./*
ls -1
