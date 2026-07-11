import { NextRequest, NextResponse } from "next/server";
import { buildAuthnRequest } from "../../lib/realme";

/** Initiates RealMe login by redirecting to the IdP with a SAML AuthnRequest. */
export async function GET(req: NextRequest) {
  const relay = req.nextUrl.searchParams.get("relayState") ?? "";
  const { redirectUrl } = buildAuthnRequest(relay);
  return NextResponse.redirect(redirectUrl, { status: 302 });
}
