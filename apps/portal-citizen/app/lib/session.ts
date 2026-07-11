import { cookies } from "next/headers";
import { PORTAL_CONFIG } from "./config";
import { verifyJwt, signJwt } from "./jwt";
import type { CitizenIdentityToken } from "@tpt/gov-schema";

export interface SessionClaims {
  did: string;
  sessionId: string;
  grantedScopes: string[];
  issuedAt: number;
  expiresAt: number;
  demo?: boolean;
}

/** Read and verify the session JWT from the httpOnly cookie. */
export async function getSession(): Promise<SessionClaims | null> {
  const store = await cookies();
  const token = store.get(PORTAL_CONFIG.sessionCookieName)?.value;
  if (!token) return null;
  const result = verifyJwt(token, PORTAL_CONFIG.sessionSecret);
  if (!result.valid || !result.payload) return null;
  const p = result.payload;
  if (typeof p.did !== "string" || typeof p.sessionId !== "string") return null;
  return {
    did: p.did,
    sessionId: p.sessionId,
    grantedScopes: Array.isArray(p.grantedScopes) ? (p.grantedScopes as string[]) : [],
    issuedAt: Number(p.iat ?? 0),
    expiresAt: Number(p.exp ?? 0),
    demo: p.demo === true,
  };
}

/** Convenience: the signed-in citizen DID, or null. */
export async function getCitizenDid(): Promise<string | null> {
  const session = await getSession();
  return session?.did ?? null;
}

/** Build a signed session JWT string (does not set the cookie). */
export function createSessionToken(claims: Omit<SessionClaims, "issuedAt" | "expiresAt">): string {
  return signJwt(
    {
      did: claims.did,
      sessionId: claims.sessionId,
      grantedScopes: claims.grantedScopes,
      ...(claims.demo ? { demo: true } : {}),
    },
    PORTAL_CONFIG.sessionSecret,
    PORTAL_CONFIG.sessionTtlSeconds,
  );
}

/** Set the session cookie on the outgoing response. */
export async function setSessionCookie(token: string): Promise<void> {
  const store = await cookies();
  store.set(PORTAL_CONFIG.sessionCookieName, token, {
    httpOnly: true,
    sameSite: "lax",
    secure: process.env.NODE_ENV === "production",
    path: "/",
    maxAge: PORTAL_CONFIG.sessionTtlSeconds,
  });
}

/** Clear the session cookie. */
export async function clearSessionCookie(): Promise<void> {
  const store = await cookies();
  store.delete(PORTAL_CONFIG.sessionCookieName);
}

export function toCitizenIdentityToken(session: SessionClaims): CitizenIdentityToken {
  return {
    did: session.did,
    sessionId: session.sessionId,
    grantedScopes: session.grantedScopes as CitizenIdentityToken["grantedScopes"],
    issuedAt: session.issuedAt,
    expiresAt: session.expiresAt,
  };
}
