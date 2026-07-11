#!/usr/bin/env bash
#
# Generate a self-signed SP signing key + certificate for RealMe SAML2 dev/demo.
#
# RealMe production requires a certificate issued by a New Zealand recognised CA
# (or the RealMe-issued certificate). This script is for local development and
# demo only — do NOT use these keys in production.
#
# Output:
#   realme-sp-signing-key.pem   RSA private key (keep secret)
#   realme-sp-signing-cert.pem  X.509 certificate (upload to RealMe / SP metadata)
#
# Usage:
#   ./scripts/gen-realme-sp-certs.sh [entityId] [days]
#
set -euo pipefail

OUT_DIR="${REALME_OUT_DIR:-./certs}"
ENTITY_ID="${1:-https://policy.example.govt.nz/realme}"
DAYS="${2:-825}"

mkdir -p "$OUT_DIR"
KEY="$OUT_DIR/realme-sp-signing-key.pem"
CERT="$OUT_DIR/realme-sp-signing-cert.pem"

openssl req -x509 -newkey rsa:3072 -nodes \
  -keyout "$KEY" \
  -out "$CERT" \
  -days "$DAYS" \
  -subj "/CN=$ENTITY_ID" \
  -addext "subjectAltName=URI:$ENTITY_ID"

echo "Wrote:"
echo "  $KEY"
echo "  $CERT"
echo
echo "Set in your environment / .env:"
echo "  REALME_SP_SIGNING_KEY=$KEY"
echo "  REALME_SP_SIGNING_CERT=$CERT"
echo "  REALME_SP_ENTITY_ID=$ENTITY_ID"
