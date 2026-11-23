import { describe, expect, it } from "vitest";
import { generateKeyPairSync, sign } from "node:crypto";
import { challengeBytes, didMatchesPublicKey, verifyEd25519, verifyEd25519Raw } from "./verify";

// Ed25519 key pairs from Node's crypto. The JWK `x` value is the unpadded
// base64url encoding of the raw 32-byte public key, the same encoding the
// browser wallet produces.
function newIdentity() {
  const { publicKey, privateKey } = generateKeyPairSync("ed25519");
  const { x } = publicKey.export({ format: "jwk" }) as { x: string };
  return { publicKeyB64: x, did: `did:gov:nz:${x}`, privateKey };
}

describe("didMatchesPublicKey", () => {
  it("accepts the DID derived from the presented key", () => {
    const id = newIdentity();
    expect(didMatchesPublicKey(id.did, id.publicKeyB64)).toBe(true);
  });

  it("rejects a DID that belongs to a different key", () => {
    const alice = newIdentity();
    const mallory = newIdentity();
    expect(didMatchesPublicKey(alice.did, mallory.publicKeyB64)).toBe(false);
  });

  it("rejects DIDs without the gov prefix", () => {
    const id = newIdentity();
    expect(didMatchesPublicKey(`did:example:${id.publicKeyB64}`, id.publicKeyB64)).toBe(false);
  });

  it("rejects keys that are not 32 bytes", () => {
    const short = Buffer.alloc(31, 7).toString("base64url");
    expect(didMatchesPublicKey(`did:gov:nz:${short}`, short)).toBe(false);
  });
});

describe("verifyEd25519", () => {
  it("accepts a signature over the challenge by the matching key", () => {
    const id = newIdentity();
    const challenge = "login-challenge-123";
    const sig = sign(null, Buffer.from(challenge, "utf8"), id.privateKey).toString("base64url");
    expect(verifyEd25519(id.publicKeyB64, challenge, sig)).toBe(true);
  });

  it("rejects a signature over a different challenge", () => {
    const id = newIdentity();
    const sig = sign(null, Buffer.from("challenge-a", "utf8"), id.privateKey).toString("base64url");
    expect(verifyEd25519(id.publicKeyB64, "challenge-b", sig)).toBe(false);
  });

  it("rejects a signature made by a different key", () => {
    const alice = newIdentity();
    const mallory = newIdentity();
    const sig = sign(null, Buffer.from("c", "utf8"), mallory.privateKey).toString("base64url");
    expect(verifyEd25519(alice.publicKeyB64, "c", sig)).toBe(false);
  });

  it("verifies a signature over the decoded challenge bytes, as the identity server does", () => {
    const id = newIdentity();
    // Identity challenges are base64url of random bytes; the wallet signs the decoded bytes.
    const challenge = Buffer.from("random-challenge-bytes-0123456789").toString("base64url");
    const sig = sign(null, challengeBytes(challenge), id.privateKey).toString("base64url");
    expect(verifyEd25519Raw(id.publicKeyB64, challengeBytes(challenge), sig)).toBe(true);
    // Signing the base64url text instead of its decoded bytes must not verify.
    const wrong = sign(null, Buffer.from(challenge, "utf8"), id.privateKey).toString("base64url");
    expect(verifyEd25519Raw(id.publicKeyB64, challengeBytes(challenge), wrong)).toBe(false);
  });

  it("returns false for malformed input rather than throwing", () => {
    expect(verifyEd25519("not-a-key", "c", "not-a-signature")).toBe(false);
  });
});
