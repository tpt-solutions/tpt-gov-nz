import { NextRequest, NextResponse } from "next/server";
import {
  buildAuthnRequest,
  createRealmeState,
  REALME_STATE_COOKIE,
  getRealmeConfig,
} from "../../lib/realme";

/**
 * Initiates RealMe login: builds a signed SAML AuthnRequest, stores a signed
 * correlation token (the request ID) in an httpOnly cookie, and redirects to
 * the IdP. The callback validates the response's InResponseTo against it.
 */
export async function GET(req: NextRequest) {
  const cfg = getRealmeConfig();
  const relay = req.nextUrl.searchParams.get("relayState") ?? "";
  const { redirectUrl, requestId } = buildAuthnRequest(relay);

  const state = createRealmeState(requestId, cfg.requestTtlSeconds);
  const res = NextResponse.redirect(redirectUrl, { status: 302 });
  res.cookies.set(REALME_STATE_COOKIE, state, {
    httpOnly: true,
    sameSite: "lax",
    secure: process.env.NODE_ENV === "production",
    path: "/login/realme",
    maxAge: cfg.requestTtlSeconds,
  });
  return res;
}
