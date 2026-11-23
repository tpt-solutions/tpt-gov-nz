import { NextRequest, NextResponse } from "next/server";
import { basicAuthPassword, passwordMatches } from "./app/lib/access";
import { buildCsp, generateNonce } from "./app/lib/csp";

/**
 * Every page and server action requires the policy-lab password (HTTP Basic).
 * In production, requests are refused when no password is configured. In development,
 * an unset password leaves the lab open so it runs with no setup.
 *
 * Every response carries a per-request nonce CSP.
 */
export async function middleware(req: NextRequest) {
  const nonce = generateNonce();
  const csp = buildCsp(nonce, process.env.NODE_ENV !== "production");
  const requestHeaders = new Headers(req.headers);
  requestHeaders.set("Content-Security-Policy", csp);
  requestHeaders.set("x-nonce", nonce);

  const expected = process.env.TPT__GOV__POLICY_PASSWORD;
  let response: NextResponse;
  if (!expected) {
    response =
      process.env.NODE_ENV === "production"
        ? new NextResponse("The policy lab is not configured.", { status: 503 })
        : NextResponse.next({ request: { headers: requestHeaders } });
  } else {
    const given = basicAuthPassword(req.headers.get("authorization"));
    if (given !== null && (await passwordMatches(given, expected))) {
      response = NextResponse.next({ request: { headers: requestHeaders } });
    } else {
      response = new NextResponse("Authentication required.", {
        status: 401,
        headers: { "WWW-Authenticate": 'Basic realm="Policy lab", charset="UTF-8"' },
      });
    }
  }

  response.headers.set("Content-Security-Policy", csp);
  return response;
}

export const config = {
  matcher: ["/((?!_next/static|_next/image|favicon.ico|manifest.webmanifest).*)"],
};
