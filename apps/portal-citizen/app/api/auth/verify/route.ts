import { NextResponse } from "next/server";
import { cookies } from "next/headers";
import { didMatchesPublicKey, verifyEd25519 } from "@/app/lib/verify";
import { createSessionToken, setSessionCookie } from "@/app/lib/session";
import { randomToken } from "@/app/lib/jwt";

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
  // The DID must be the fingerprint of the presented key, otherwise anyone with a
  // key pair could sign in as someone else's DID.
  if (!didMatchesPublicKey(body.did, body.publicKey)) {
    return NextResponse.json({ ok: false, error: "DID does not match public key." }, { status: 401 });
  }
  if (!verifyEd25519(body.publicKey, body.challenge, body.signature)) {
    return NextResponse.json({ ok: false, error: "Signature verification failed." }, { status: 401 });
  }

  // Note: DID registration with the identity server is not performed here. The
  // challenge-based registration endpoint requires the server-side API key.
  store.delete("tpt_auth_challenge");

  const token = createSessionToken({
    did: body.did,
    sessionId: randomToken(16),
    grantedScopes: [],
  });
  await setSessionCookie(token);

  return NextResponse.json({ ok: true, did: body.did });
}
