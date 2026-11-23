import { NextRequest, NextResponse } from "next/server";
import { buildCsp, generateNonce } from "./app/lib/csp";

/**
 * Applies a per-request CSP. Next.js reads the nonce from the request's
 * Content-Security-Policy header and applies it to its own inline scripts.
 */
export function middleware(req: NextRequest) {
  const nonce = generateNonce();
  const csp = buildCsp(nonce, process.env.NODE_ENV !== "production");

  const requestHeaders = new Headers(req.headers);
  requestHeaders.set("Content-Security-Policy", csp);
  requestHeaders.set("x-nonce", nonce);

  const response = NextResponse.next({ request: { headers: requestHeaders } });
  response.headers.set("Content-Security-Policy", csp);
  return response;
}

export const config = {
  matcher: ["/((?!_next/static|_next/image|favicon.ico|manifest.webmanifest).*)"],
};
