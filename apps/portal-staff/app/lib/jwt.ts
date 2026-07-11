import { createHmac, randomBytes, timingSafeEqual } from "node:crypto";

const B64URL = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

function base64urlEncode(input: Buffer | string): string {
  const buf = Buffer.isBuffer(input) ? input : Buffer.from(input, "utf8");
  return buf.toString("base64").replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
}

function base64urlDecode(input: string): Buffer {
  const padded = input.replace(/-/g, "+").replace(/_/g, "/");
  const pad = padded.length % 4 === 0 ? "" : "=".repeat(4 - (padded.length % 4));
  return Buffer.from(padded + pad, "base64");
}

function hmac(secret: string, data: string): Buffer {
  return createHmac("sha256", secret).update(data).digest();
}

export interface JwtHeader {
  alg: "HS256";
  typ: "JWT";
}

/**
 * Sign an HS256 JWT. `payload` must be a JSON-serialisable object; `exp` is set
 * automatically `ttlSeconds` from now.
 */
export function signJwt(payload: Record<string, unknown>, secret: string, ttlSeconds: number): string {
  const header: JwtHeader = { alg: "HS256", typ: "JWT" };
  const now = Math.floor(Date.now() / 1000);
  const body = { ...payload, iat: now, exp: now + ttlSeconds };
  const signingInput = `${base64urlEncode(JSON.stringify(header))}.${base64urlEncode(JSON.stringify(body))}`;
  const sig = hmac(secret, signingInput).toString("base64").replace(/\+/g, "-").replace(/\//g, "_").replace(/=+$/, "");
  return `${signingInput}.${sig}`;
}

export interface VerifyResult {
  valid: boolean;
  payload?: Record<string, unknown>;
}

export function verifyJwt(token: string, secret: string): VerifyResult {
  const parts = token.split(".");
  if (parts.length !== 3) return { valid: false };
  const [headerB64, payloadB64, sigB64] = parts;
  const expected = hmac(secret, `${headerB64}.${payloadB64}`)
    .toString("base64")
    .replace(/\+/g, "-")
    .replace(/\//g, "_")
    .replace(/=+$/, "");
  const a = Buffer.from(sigB64);
  const b = Buffer.from(expected);
  if (a.length !== b.length || !timingSafeEqual(a, b)) return { valid: false };
  try {
    const payload = JSON.parse(base64urlDecode(payloadB64).toString("utf8")) as Record<string, unknown>;
    const now = Math.floor(Date.now() / 1000);
    if (typeof payload.exp === "number" && payload.exp < now) return { valid: false };
    return { valid: true, payload };
  } catch {
    return { valid: false };
  }
}

export function randomToken(bytes = 32): string {
  return randomBytes(bytes).toString("hex");
}

export { base64urlEncode, base64urlDecode, B64URL };
