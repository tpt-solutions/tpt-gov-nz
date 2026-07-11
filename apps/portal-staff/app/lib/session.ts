import { cookies } from "next/headers";
import { STAFF_CONFIG } from "./config";
import { verifyJwt, signJwt } from "./jwt";

export interface StaffSessionClaims {
  staffId: string;
  displayName: string;
  sessionId: string;
  issuedAt: number;
  expiresAt: number;
  demo: boolean;
}

/** Read and verify the staff session JWT from the httpOnly cookie. */
export async function getStaffSession(): Promise<StaffSessionClaims | null> {
  const store = await cookies();
  const token = store.get(STAFF_CONFIG.sessionCookieName)?.value;
  if (!token) return null;
  const result = verifyJwt(token, STAFF_CONFIG.sessionSecret);
  if (!result.valid || !result.payload) return null;
  const p = result.payload;
  if (typeof p.staffId !== "string" || typeof p.sessionId !== "string") return null;
  return {
    staffId: p.staffId,
    displayName: typeof p.displayName === "string" ? p.displayName : p.staffId,
    sessionId: p.sessionId,
    issuedAt: Number(p.iat ?? 0),
    expiresAt: Number(p.exp ?? 0),
    demo: p.demo === true,
  };
}

/** Build a signed staff session JWT string (does not set the cookie). */
export function createStaffSessionToken(claims: Omit<StaffSessionClaims, "issuedAt" | "expiresAt">): string {
  return signJwt(
    {
      staffId: claims.staffId,
      displayName: claims.displayName,
      sessionId: claims.sessionId,
      demo: claims.demo,
    },
    STAFF_CONFIG.sessionSecret,
    STAFF_CONFIG.sessionTtlSeconds,
  );
}

/** Set the staff session cookie on the outgoing response. */
export async function setStaffSessionCookie(token: string): Promise<void> {
  const store = await cookies();
  store.set(STAFF_CONFIG.sessionCookieName, token, {
    httpOnly: true,
    sameSite: "lax",
    secure: process.env.NODE_ENV === "production",
    path: "/",
    maxAge: STAFF_CONFIG.sessionTtlSeconds,
  });
}

/** Clear the staff session cookie. */
export async function clearStaffSessionCookie(): Promise<void> {
  const store = await cookies();
  store.delete(STAFF_CONFIG.sessionCookieName);
}
