import { NextRequest, NextResponse } from "next/server";
import {
  decodeSamlResponse,
  parseSamlSubject,
  verifySamlSignature,
} from "../../../lib/realme";
import { createStaffSessionToken, setStaffSessionCookie } from "../../../lib/session";
import { randomToken } from "../../../lib/jwt";

/**
 * RealMe ACS (Assertion Consumer Service). Receives the POST-binding SAMLResponse,
 * verifies it, and establishes a staff session from the asserted identity.
 */
export async function POST(req: NextRequest) {
  const form = await req.formData();
  const raw = String(form.get("SAMLResponse") ?? "");
  if (!raw) {
    return NextResponse.redirect(new URL("/login?error=norealme", req.url), 302);
  }

  const xml = decodeSamlResponse(raw);
  const verify = verifySamlSignature(xml);
  if (!verify.ok) {
    const detail = encodeURIComponent(verify.reason ?? "invalid signature");
    return NextResponse.redirect(
      new URL(`/login?error=saml&detail=${detail}`, req.url),
      302,
    );
  }

  const subject = parseSamlSubject(xml);
  if (!subject.nameId) {
    return NextResponse.redirect(new URL("/login?error=noid", req.url), 302);
  }

  const displayName =
    subject.attributes["urn:oid:2.5.4.42"] ??
    subject.attributes["cn"] ??
    subject.nameId;

  const token = createStaffSessionToken({
    staffId: subject.nameId,
    displayName,
    sessionId: randomToken(16),
    demo: false,
  });
  await setStaffSessionCookie(token);

  return NextResponse.redirect(new URL("/citizens", req.url), 302);
}
