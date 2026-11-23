/**
 * Server-side client for the identity server's DID registration endpoints.
 *
 * Only route handlers import this module. The identity API key is read from the
 * server environment and is never sent to the browser.
 */
import { PORTAL_CONFIG } from "./config";

/** The identity server could not be reached, or is misconfigured. Sign-in fails closed. */
export class IdentityUnavailableError extends Error {}

/** The identity server refused the request (bad challenge, conflicting key, and so on). */
export class IdentityRejectedError extends Error {
  constructor(
    message: string,
    readonly status: number,
  ) {
    super(message);
  }
}

const TIMEOUT_MS = 5000;

function apiKey(): string {
  const key = process.env.TPT__GOV__IDENTITY_API_KEY ?? "";
  if (!key) throw new IdentityUnavailableError("TPT__GOV__IDENTITY_API_KEY is not set");
  return key;
}

export interface IdentityChallenge {
  /** base64url challenge bytes. The wallet signs the decoded bytes. */
  challenge: string;
  challengeId: string;
}

/** Ask the identity server for a one-time challenge bound to this DID and key. */
export async function requestDidChallenge(did: string, publicKeyB64: string): Promise<IdentityChallenge> {
  const res = await postJson("/v1/did/challenge", { did, public_key_b64: publicKeyB64 });
  if (!res.ok) throw new IdentityRejectedError("challenge refused", res.status);
  const data = (await res.json()) as { challenge: string; challenge_id: string };
  return { challenge: data.challenge, challengeId: data.challenge_id };
}

/**
 * Register the DID with the signature over its challenge. Registration is
 * first-write-wins: the same key is accepted again, a different key is refused.
 */
export async function registerDidWithProof(challengeId: string, signatureStdB64: string): Promise<void> {
  const res = await postJson("/v1/did/register", {
    challenge_id: challengeId,
    signature_b64: signatureStdB64,
  });
  if (!res.ok) throw new IdentityRejectedError("registration refused", res.status);
}

async function postJson(path: string, body: unknown): Promise<Response> {
  const key = apiKey();
  try {
    return await fetch(`${PORTAL_CONFIG.identityServerUrl}${path}`, {
      method: "POST",
      headers: { "Content-Type": "application/json", "x-identity-api-key": key },
      body: JSON.stringify(body),
      signal: AbortSignal.timeout(TIMEOUT_MS),
    });
  } catch (e) {
    throw new IdentityUnavailableError(e instanceof Error ? e.message : "identity server unreachable");
  }
}
