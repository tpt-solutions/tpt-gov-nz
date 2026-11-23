import { NextRequest, NextResponse } from "next/server";
import { STAFF_CONFIG } from "./app/lib/config";
import { verifyJwtEdge } from "./app/lib/jwt-edge";
import { buildCsp, generateNonce } from "./app/lib/csp";

const PUBLIC_PATHS = ["/login", "/realme"];

/**
 * Every non-public request, including server-action POSTs to those pages, must
 * carry a correctly signed, unexpired staff session token. A cookie that merely
 * exists is not enough. Pages and actions also re-check via `getStaffSession`.
 *
 * Every response, including redirects, carries a per-request nonce CSP.
 */
export async function middleware(req: NextRequest) {
  const nonce = generateNonce();
  const csp = buildCsp(nonce, process.env.NODE_ENV !== "production");
  const { pathname } = req.nextUrl;

  const requestHeaders = new Headers(req.headers);
  requestHeaders.set("Content-Security-Policy", csp);
  requestHeaders.set("x-nonce", nonce);

  let response: NextResponse;
  const isPublic = PUBLIC_PATHS.some((p) => pathname === p || pathname.startsWith(`${p}/`));
  if (isPublic) {
    response = NextResponse.next({ request: { headers: requestHeaders } });
  } else {
    const token = req.cookies.get(STAFF_CONFIG.sessionCookieName)?.value;
    const valid = token ? await verifyJwtEdge(token, STAFF_CONFIG.sessionSecret) : false;
    if (valid) {
      response = NextResponse.next({ request: { headers: requestHeaders } });
    } else {
      const url = req.nextUrl.clone();
      url.pathname = "/login";
      url.searchParams.set("redirect", pathname);
      response = NextResponse.redirect(url);
      if (token) response.cookies.delete(STAFF_CONFIG.sessionCookieName);
    }
  }

  response.headers.set("Content-Security-Policy", csp);
  return response;
}

export const config = {
  matcher: ["/((?!_next/static|_next/image|favicon.ico|manifest.webmanifest).*)"],
};
