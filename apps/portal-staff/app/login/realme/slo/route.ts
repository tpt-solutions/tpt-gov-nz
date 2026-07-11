import { NextRequest, NextResponse } from "next/server";
import { clearStaffSessionCookie } from "../../../lib/session";

/**
 * Single Logout (SLO) endpoint for the RealMe logout flow.
 *
 * Receives a LogoutRequest (HTTP-Redirect, signed) or LogoutResponse and ends
 * the local staff session. Producing a signed LogoutResponse back to the IdP is
 * a no-op stub here: staff sign-out is local-only and the portal does not yet
 * initiate IdP-side global logout. Wire up response signing when the IdP
 * requires front-channel SLO completion.
 */
export async function GET(req: NextRequest) {
  await clearStaffSessionCookie();
  const base = new URL(req.url);
  base.pathname = "/login";
  return NextResponse.redirect(base, 302);
}

export async function POST(req: NextRequest) {
  await clearStaffSessionCookie();
  const base = new URL(req.url);
  base.pathname = "/login";
  return NextResponse.redirect(base, 302);
}
