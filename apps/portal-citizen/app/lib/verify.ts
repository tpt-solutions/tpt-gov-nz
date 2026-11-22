import { createPublicKey, verify as cryptoVerify } from "node:crypto";

function fromBase64Url(s: string): Buffer {
  const padded = s.replace(/-/g, "+").replace(/_/g, "/");
  const pad = padded.length % 4 === 0 ? "" : "=".repeat(4 - (padded.length % 4));
  return Buffer.from(padded + pad, "base64");
}

/**
 * Server-side Ed25519 signature verification that mirrors the browser wallet.
 * Accepts a base64url-encoded raw 32-byte public key (as produced by the
 * WebCrypto wallet) and a base64url-encoded signature over `challenge`.
 */
/**
 * Bind a DID to the public key presenting it. Mirrors
 * `gov-identity-core::GovDid::from_verifying_key`: the DID is `did:gov:nz:` followed
 * by the unpadded base64url encoding of the 32-byte Ed25519 public key. A key that
 * does not produce the claimed DID must not be able to start a session as that DID.
 */
export function didMatchesPublicKey(did: string, publicKeyB64: string): boolean {
  const PREFIX = "did:gov:nz:";
  if (!did.startsWith(PREFIX)) return false;
  try {
    const bytes = fromBase64Url(publicKeyB64);
    if (bytes.length !== 32) return false;
    return did.slice(PREFIX.length) === bytes.toString("base64url");
  } catch {
    return false;
  }
}

export function verifyEd25519(
  publicKeyB64: string,
  challenge: string,
  signatureB64: string,
): boolean {
  try {
    const pubBytes = fromBase64Url(publicKeyB64);
    const pubKey = createPublicKey({
      key: { kty: "OKP", crv: "Ed25519", x: pubBytes.toString("base64url") },
      format: "jwk",
    });
    const sig = fromBase64Url(signatureB64);
    return cryptoVerify(null, Buffer.from(challenge, "utf8"), pubKey, sig);
  } catch {
    return false;
  }
}
