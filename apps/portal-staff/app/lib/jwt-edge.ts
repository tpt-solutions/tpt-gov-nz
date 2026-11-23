/**
 * HS256 JWT verification using Web Crypto, for the Edge runtime (middleware).
 *
 * `jwt.ts` uses `node:crypto` and cannot run in middleware. The two must accept
 * the same tokens; `jwt-edge.test.ts` checks that against `signJwt`.
 */

const encoder = new TextEncoder();

function base64urlToBytes(input: string): Uint8Array<ArrayBuffer> {
  const padded = input.replace(/-/g, "+").replace(/_/g, "/");
  const pad = padded.length % 4 === 0 ? "" : "=".repeat(4 - (padded.length % 4));
  const binary = atob(padded + pad);
  const out = new Uint8Array(new ArrayBuffer(binary.length));
  for (let i = 0; i < binary.length; i++) out[i] = binary.charCodeAt(i);
  return out;
}

/** Returns true only for a correctly signed, unexpired HS256 token. */
export async function verifyJwtEdge(token: string, secret: string): Promise<boolean> {
  const parts = token.split(".");
  if (parts.length !== 3) return false;
  const [headerB64, payloadB64, sigB64] = parts;
  try {
    const key = await crypto.subtle.importKey(
      "raw",
      encoder.encode(secret),
      { name: "HMAC", hash: "SHA-256" },
      false,
      ["verify"],
    );
    const valid = await crypto.subtle.verify(
      "HMAC",
      key,
      base64urlToBytes(sigB64),
      encoder.encode(`${headerB64}.${payloadB64}`),
    );
    if (!valid) return false;
    const payload = JSON.parse(new TextDecoder().decode(base64urlToBytes(payloadB64))) as {
      exp?: unknown;
    };
    const now = Math.floor(Date.now() / 1000);
    return typeof payload.exp === "number" && payload.exp >= now;
  } catch {
    return false;
  }
}
