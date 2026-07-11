#!/usr/bin/env bash
# Generate the internal mTLS certificate chain for tpt-gov-nz.
#
# Produces a private internal CA plus a server certificate and a client
# certificate, all signed by that CA. Every internal service presents either
# the server or client certificate and verifies peers against `ca.pem`.
#
# Output is written to scripts/mtls-certs/ (gitignored). Re-run any time; certs
# are valid for 10 years (demo only — production uses a real internal PKI).
set -euo pipefail

OUT="$(cd "$(dirname "$0")" && pwd)/mtls-certs"
mkdir -p "$OUT"
cd "$OUT"

DAYS=3650

echo ">> Generating internal CA"
openssl genpkey -algorithm ed25519 -out ca.key
openssl req -x509 -new -nodes -key ca.key -sha256 -days "$DAYS" \
  -subj "/CN=tpt-gov-nz-internal-ca/O=tpt-gov-nz" -out ca.pem

gen_leaf() {
  local name="$1"; local cn="$2"
  echo ">> Generating $name certificate"
  openssl genpkey -algorithm ed25519 -out "$name.key"
  openssl req -new -key "$name.key" -subj "/CN=$cn/O=tpt-gov-nz" -out "$name.csr"
  openssl x509 -req -in "$name.csr" -CA ca.pem -CAkey ca.key -CAcreateserial \
    -days "$DAYS" -sha256 -out "$name.pem"
  rm -f "$name.csr"
}

gen_leaf server  "gov-gateway"
gen_leaf client  "gov-internal-client"
gen_leaf idserver "gov-identity-server"
gen_leaf ird     "gov-dept-ird"
gen_leaf winz    "gov-dept-winz"
gen_leaf moh     "gov-dept-moh"
gen_leaf dia     "gov-dept-dia"
gen_leaf nzta    "gov-dept-nzta"
gen_leaf acc     "gov-dept-acc"
gen_leaf fed     "gov-federation-node"

echo ">> Done. Certs in $OUT"
echo "   Mount ca.pem + the relevant <svc>.pem/<svc>.key into each container and set:"
echo "   TPT__GOV__MTLS_CA=/certs/ca.pem TPT__GOV__MTLS_CERT=/certs/<svc>.pem TPT__GOV__MTLS_KEY=/certs/<svc>.key"
