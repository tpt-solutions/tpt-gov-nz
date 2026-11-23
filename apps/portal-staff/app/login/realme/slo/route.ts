import { NextRequest, NextResponse } from "next/server";
import { clearStaffSessionCookie, getStaffSession } from "../../../lib/session";
import {
  buildLogoutResponse,
  processLogoutRequest,
  processLogoutResponse,
  REALME_SLO_COOKIE,
  verifyRealmeState,
} from "../../../lib/realme";

/**
 * RealMe Single Logout endpoint (HTTP-Redirect binding).
 *
 * Only a signed message from the pinned IdP changes the session:
 *  - IdP LogoutRequest: end the matching staff session, answer with a signed LogoutResponse.
 *  - IdP LogoutResponse: completes a logout we started; must answer our LogoutRequest.
 * Anything else, including a plain GET from a link or a cross-site page, is ignored, so it
 * cannot log a case worker out (the previous version cleared the session on every request).
 */
export async function GET(req: NextRequest) {
  const rawQuery = req.nextUrl.search;
  const loginUrl = (query: string) => new URL(`/login${query}`, req.url);

  if (rawQuery.includes("SAMLRequest=")) {
    let nameId: string | undefined;
    let requestId: string;
    try {
      const msg = processLogoutRequest(rawQuery);
      nameId = msg.nameId;
      requestId = msg.id;
    } catch {
      return NextResponse.redirect(loginUrl("?error=slo"), 302);
    }
    // Only end the session that belongs to the NameID the IdP is logging out.
    const session = await getStaffSession();
    if (session?.realmeNameId && (!nameId || nameId === session.realmeNameId)) {
      await clearStaffSessionCookie();
    }
    return NextResponse.redirect(buildLogoutResponse(requestId), 302);
  }

  if (rawQuery.includes("SAMLResponse=")) {
    const state = verifyRealmeState(req.cookies.get(REALME_SLO_COOKIE)?.value);
    const res = (query: string) => {
      const r = NextResponse.redirect(loginUrl(query), 302);
      r.cookies.set(REALME_SLO_COOKIE, "", { path: "/login/realme", maxAge: 0 });
      return r;
    };
    try {
      processLogoutResponse(rawQuery, state.requestId ?? "");
    } catch {
      return res("?error=slo");
    }
    await clearStaffSessionCookie();
    return res("?loggedOut=1");
  }

  return NextResponse.redirect(loginUrl(""), 302);
}
