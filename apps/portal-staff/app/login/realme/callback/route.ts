import { NextRequest, NextResponse } from "next/server";
import {
  processSamlResponse,
  verifyRealmeState,
  REALME_STATE_COOKIE,
  getRealmeConfig,
} from "../../../lib/realme";
import { createStaffSessionToken, setStaffSessionCookie } from "../../../lib/session";
import { randomToken } from "../../../lib/jwt";

/**
 * RealMe ACS (Assertion Consumer Service). Receives the POST-binding SAMLResponse,
 * verifies the signature, validates conditions/audience/InResponseTo correlation
 * and replay protection, then establishes a staff session from the asserted identity.
 */
export async function POST(req: NextRequest) {
  const form = await req.formData();
  const raw = String(form.get("SAMLResponse") ?? "");
  if (!raw) {
    return NextResponse.redirect(new URL("/login?error=norealme", req.url), 302);
  }

  const state = verifyRealmeState(req.cookies.get(REALME_STATE_COOKIE)?.value);
  if (!state.ok) {
    return NextResponse.redirect(new URL("/login?error=realme_state", req.url), 302);
  }

  const cfg = getRealmeConfig();
  let result;
  try {
    result = processSamlResponse(raw, {
      requestId: state.requestId,
      acsUrl: cfg.acsUrl,
      audience: cfg.audience,
    });
  } catch (err) {
    const detail = encodeURIComponent(
      err instanceof Error ? err.message : "SAML processing failed",
    );
    return NextResponse.redirect(
      new URL(`/login?error=saml&detail=${detail}`, req.url),
      302,
    );
  }

  const subject = result.subject;
  if (!subject.nameId) {
    return NextResponse.redirect(new URL("/login?error=noid", req.url), 302);
  }

  const attrs = subject.attributes;
  const displayName =
    attrs["urn:oid:2.5.4.42"]?.[0] ??
    attrs["cn"]?.[0] ??
    attrs["urn:oid:2.5.4.4"]?.[0] ??
    subject.nameId;

  const token = createStaffSessionToken({
    staffId: subject.nameId,
    displayName,
    sessionId: randomToken(16),
    demo: false,
  });
  await setStaffSessionCookie(token);

  const res = NextResponse.redirect(new URL("/citizens", req.url), 302);
  res.cookies.delete(REALME_STATE_COOKIE);
  return res;
}
