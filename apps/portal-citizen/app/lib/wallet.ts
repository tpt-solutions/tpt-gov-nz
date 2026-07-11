"use client";

// Browser-based verifiable-credential wallet.
//
// Uses the WebCrypto Ed25519 implementation (SubtleCrypto "EdDSA"). The citizen's
// private key never leaves the browser and is persisted (non-extractable export)
// in localStorage. The DID is derived from the public key exactly as the Rust
// `gov-identity-core::GovDid::from_verifying_key` does: `did:gov:nz:` + the
// URL-safe, unpadded base64 of the 32-byte public key.

const WALLET_KEY = "tpt_wallet_v1";
const DID_PREFIX = "did:gov:nz:";

function toBase64Url(bytes: Uint8Array): string {
  let binary = "";
  for (const b of bytes) binary += String.fromCharCode(b);
  return btoa(binary).replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

function fromBase64Url(s: string): Uint8Array {
  const padded = s.replace(/-/g, "+").replace(/_/g, "/");
  const pad = padded.length % 4 === 0 ? "" : "=".repeat(4 - (padded.length % 4));
  const binary = atob(padded + pad);
  const out = new Uint8Array(binary.length);
  for (let i = 0; i < binary.length; i++) out[i] = binary.charCodeAt(i);
  return out;
}

export interface Wallet {
  did: string;
  publicKeyB64: string;
  privateKeyB64: string;
}

export function isBrowser(): boolean {
  return typeof window !== "undefined" && typeof window.crypto !== "undefined";
}

function didFromPublicKey(publicKeyBytes: Uint8Array): string {
  return DID_PREFIX + toBase64Url(publicKeyBytes);
}

/** Create a new Ed25519 wallet and persist it in localStorage. */
export async function generateWallet(): Promise<Wallet> {
  if (!isBrowser()) throw new Error("Wallet can only be created in the browser");
  const keyPair = await window.crypto.subtle.generateKey(
    { name: "EdDSA", namedCurve: "Ed25519" },
    false,
    ["sign", "verify"],
  );
  const publicKeyBytes = new Uint8Array(
    await window.crypto.subtle.exportKey("raw", keyPair.publicKey),
  );
  const privateKeyBytes = new Uint8Array(
    await window.crypto.subtle.exportKey("pkcs8", keyPair.privateKey),
  );
  const wallet: Wallet = {
    did: didFromPublicKey(publicKeyBytes),
    publicKeyB64: toBase64Url(publicKeyBytes),
    // pkcs8-wrapped private key, base64url encoded
    privateKeyB64: toBase64Url(privateKeyBytes),
  };
  localStorage.setItem(WALLET_KEY, JSON.stringify(wallet));
  return wallet;
}

/** Read the persisted wallet, or null if none exists. */
export function getWallet(): Wallet | null {
  if (!isBrowser()) return null;
  const raw = localStorage.getItem(WALLET_KEY);
  if (!raw) return null;
  try {
    const w = JSON.parse(raw) as Wallet;
    if (w.did?.startsWith(DID_PREFIX) && w.privateKeyB64) return w;
    return null;
  } catch {
    return null;
  }
}

export function getWalletDid(): string | null {
  return getWallet()?.did ?? null;
}

/** Sign an arbitrary challenge string with the persisted private key. */
export async function signChallenge(challenge: string): Promise<string> {
  const wallet = getWallet();
  if (!wallet) throw new Error("No wallet found");
  const privateKey = await window.crypto.subtle.importKey(
    "pkcs8",
    fromBase64Url(wallet.privateKeyB64) as BufferSource,
    { name: "EdDSA", namedCurve: "Ed25519" },
    false,
    ["sign"],
  );
  const sig = await window.crypto.subtle.sign(
    { name: "EdDSA" },
    privateKey,
    new TextEncoder().encode(challenge) as BufferSource,
  );
  return toBase64Url(new Uint8Array(sig));
}

/** Verify a signature against a public key (used in tests / demonstration). */
export async function verifySignature(
  publicKeyB64: string,
  challenge: string,
  signatureB64: string,
): Promise<boolean> {
  const publicKey = await window.crypto.subtle.importKey(
    "raw",
    fromBase64Url(publicKeyB64) as BufferSource,
    { name: "EdDSA", namedCurve: "Ed25519" },
    false,
    ["verify"],
  );
  return window.crypto.subtle.verify(
    { name: "EdDSA" },
    publicKey,
    fromBase64Url(signatureB64) as BufferSource,
    new TextEncoder().encode(challenge) as BufferSource,
  );
}

export function clearWallet(): void {
  if (isBrowser()) localStorage.removeItem(WALLET_KEY);
}
