import { NextResponse } from "next/server";
import { cookies } from "next/headers";
import { verifyEd25519 } from "@/app/lib/verify";
import { createSessionToken, setSessionCookie } from "@/app/lib/session";
import { randomToken } from "@/app/lib/jwt";
import { PORTAL_CONFIG } from "@/app/lib/config";

export const dynamic = "force-dynamic";

interface VerifyBody {
  did: string;
  publicKey: string;
  signature: string;
  challenge: string;
}

const DID_RE = /^did:gov:nz:[A-Za-z0-9_-]+$/;

export async function POST(req: Request) {
  const store = await cookies();
  const expectedChallenge = store.get("tpt_auth_challenge")?.value;
  if (!expectedChallenge) {
    return NextResponse.json({ ok: false, error: "No challenge in progress. Start again." }, { status: 400 });
  }

  const body = (await req.json()) as VerifyBody;
  if (!body.did || !body.publicKey || !body.signature || !body.challenge) {
    return NextResponse.json({ ok: false, error: "Missing fields." }, { status: 400 });
  }
  if (body.challenge !== expectedChallenge) {
    return NextResponse.json({ ok: false, error: "Challenge mismatch." }, { status: 400 });
  }
  if (!DID_RE.test(body.did)) {
    return NextResponse.json({ ok: false, error: "Invalid DID." }, { status: 400 });
  }
  if (!verifyEd25519(body.publicKey, body.challenge, body.signature)) {
    return NextResponse.json({ ok: false, error: "Signature verification failed." }, { status: 401 });
  }

  // Best-effort registration with the identity server. Registration is
  // idempotent (upsert on DID), so repeating it is safe.
  try {
    await fetch(`${PORTAL_CONFIG.identityServerUrl}/v1/did/register`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did: body.did, public_key_b64: body.publicKey }),
      signal: AbortSignal.timeout(2000),
    });
  } catch {
    // Identity server may be offline (e.g. pure demo). Continue with a local session.
  }

  store.delete("tpt_auth_challenge");

  const token = createSessionToken({
    did: body.did,
    sessionId: randomToken(16),
    grantedScopes: [],
  });
  await setSessionCookie(token);

  return NextResponse.json({ ok: true, did: body.did });
}
