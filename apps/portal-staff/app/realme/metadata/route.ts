import { NextResponse } from "next/server";
import { generateSpMetadata } from "../../lib/realme";

/** Serves the SP SAML 2.0 metadata for registration with the RealMe IdP. */
export async function GET() {
  const xml = generateSpMetadata();
  return new NextResponse(xml, {
    status: 200,
    headers: {
      "Content-Type": "application/samlmetadata+xml; charset=utf-8",
      "Cache-Control": "public, max-age=3600",
    },
  });
}
