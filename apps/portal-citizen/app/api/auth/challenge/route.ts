import { secureCookies } from "@/app/lib/config";
import { NextResponse } from "next/server";
import { cookies } from "next/headers";
import {
  IdentityRejectedError,
  IdentityUnavailableError,
  requestDidChallenge,
} from "@/app/lib/identity-server";
import { didMatchesPublicKey } from "@/app/lib/verify";

export const dynamic = "force-dynamic";

const DID_RE = /^did:gov:nz:[A-Za-z0-9_-]+$/;

/**
 * Start sign-in. The challenge comes from the identity server, bound to this DID
 * and key, so the verify step can register the DID with proof of possession.
 */
export async function POST(req: Request) {
  let body: { did?: unknown; publicKey?: unknown };
  try {
    body = (await req.json()) as { did?: unknown; publicKey?: unknown };
  } catch {
    return NextResponse.json({ error: "Invalid JSON body" }, { status: 400 });
  }
  const did = typeof body.did === "string" ? body.did : "";
  const publicKey = typeof body.publicKey === "string" ? body.publicKey : "";
  if (!DID_RE.test(did) || !publicKey || !didMatchesPublicKey(did, publicKey)) {
    return NextResponse.json({ error: "DID does not match public key." }, { status: 400 });
  }

  let challenge;
  try {
    challenge = await requestDidChallenge(did, publicKey);
  } catch (e) {
    if (e instanceof IdentityRejectedError) {
      return NextResponse.json({ error: "Sign-in could not start. Try again." }, { status: 400 });
    }
    // Fail closed: without the identity server, sign-in cannot proceed.
    if (!(e instanceof IdentityUnavailableError)) console.error("challenge failed", e);
    return NextResponse.json({ error: "Sign-in is unavailable right now." }, { status: 503 });
  }

  const store = await cookies();
  const cookieOptions = {
    httpOnly: true,
    sameSite: "lax" as const,
    secure: secureCookies(),
    path: "/",
    maxAge: 300,
  };
  store.set("tpt_auth_challenge", challenge.challenge, cookieOptions);
  store.set("tpt_auth_challenge_id", challenge.challengeId, cookieOptions);

  return NextResponse.json({ challenge: challenge.challenge });
}
