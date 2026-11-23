import { NextResponse } from "next/server";
import { cookies } from "next/headers";
import { challengeBytes, didMatchesPublicKey, verifyEd25519Raw } from "@/app/lib/verify";
import { createSessionToken, setSessionCookie } from "@/app/lib/session";
import { randomToken } from "@/app/lib/jwt";
import {
  IdentityRejectedError,
  IdentityUnavailableError,
  registerDidWithProof,
} from "@/app/lib/identity-server";

export const dynamic = "force-dynamic";

interface VerifyBody {
  did: string;
  publicKey: string;
  signature: string;
  challenge: string;
}

const DID_RE = /^did:gov:nz:[A-Za-z0-9_-]+$/;

/**
 * Complete sign-in. The DID must match the presented key, the signature must cover
 * the challenge issued for this browser, and the identity server must accept the
 * registration. Any failure denies the session.
 */
export async function POST(req: Request) {
  const store = await cookies();
  const expectedChallenge = store.get("tpt_auth_challenge")?.value;
  const challengeId = store.get("tpt_auth_challenge_id")?.value;
  if (!expectedChallenge || !challengeId) {
    return NextResponse.json({ ok: false, error: "No challenge in progress. Start again." }, { status: 400 });
  }

  let body: VerifyBody;
  try {
    body = (await req.json()) as VerifyBody;
  } catch {
    return NextResponse.json({ ok: false, error: "Invalid JSON body." }, { status: 400 });
  }
  if (!body.did || !body.publicKey || !body.signature || !body.challenge) {
    return NextResponse.json({ ok: false, error: "Missing fields." }, { status: 400 });
  }
  if (body.challenge !== expectedChallenge) {
    return NextResponse.json({ ok: false, error: "Challenge mismatch." }, { status: 400 });
  }
  if (!DID_RE.test(body.did) || !didMatchesPublicKey(body.did, body.publicKey)) {
    return NextResponse.json({ ok: false, error: "DID does not match public key." }, { status: 401 });
  }
  // The identity server verifies the signature over the decoded challenge bytes, so verify the same bytes.
  if (!verifyEd25519Raw(body.publicKey, challengeBytes(body.challenge), body.signature)) {
    return NextResponse.json({ ok: false, error: "Signature verification failed." }, { status: 401 });
  }

  // The identity server standard-base64-encodes the signature it receives.
  const signatureStdB64 = Buffer.from(body.signature, "base64url").toString("base64");
  try {
    await registerDidWithProof(challengeId, signatureStdB64);
  } catch (e) {
    if (e instanceof IdentityUnavailableError) {
      return NextResponse.json({ ok: false, error: "Sign-in is unavailable right now." }, { status: 503 });
    }
    const status = e instanceof IdentityRejectedError && e.status === 409 ? 409 : 401;
    const error =
      status === 409
        ? "This identity is registered with a different key."
        : "Sign-in could not be completed. Start again.";
    return NextResponse.json({ ok: false, error }, { status });
  }

  store.delete("tpt_auth_challenge");
  store.delete("tpt_auth_challenge_id");

  const token = createSessionToken({
    did: body.did,
    sessionId: randomToken(16),
    grantedScopes: [],
  });
  await setSessionCookie(token);

  return NextResponse.json({ ok: true, did: body.did });
}
