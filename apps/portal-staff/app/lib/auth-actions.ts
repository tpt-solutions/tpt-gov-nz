"use server";

import { redirect } from "next/navigation";
import { createHash, timingSafeEqual } from "node:crypto";
import { cookies } from "next/headers";
import {
  createStaffSessionToken,
  setStaffSessionCookie,
  clearStaffSessionCookie,
  getStaffSession,
} from "./session";
import { randomToken } from "./jwt";
import { clearFailures, isLocked, recordFailure } from "./login-limiter";
import { STAFF_CONFIG, secureCookies } from "./config";
import {
  buildLogoutRequest,
  createRealmeState,
  getRealmeConfig,
  REALME_SLO_COOKIE,
} from "./realme";

/** Compare secrets in constant time (hashing first so lengths never leak). */
function passwordMatches(given: string, expected: string): boolean {
  const a = createHash("sha256").update(given, "utf8").digest();
  const b = createHash("sha256").update(expected, "utf8").digest();
  return timingSafeEqual(a, b);
}

/**
 * Demo sign-in: establish a staff session for the fictional case worker without
 * requiring a real RealMe credential. Only available when `NEXT_PUBLIC_DEMO_MODE`
 * is on; enforced here on the server, not just by hiding the button.
 */
export async function loginStaffDemo() {
  if (!STAFF_CONFIG.demoMode) redirect("/login");
  const token = createStaffSessionToken({
    staffId: "staff-demo",
    displayName: "Demo Case Worker",
    sessionId: randomToken(16),
    demo: true,
  });
  await setStaffSessionCookie(token);
  redirect("/citizens");
}

/**
 * Real sign-in: a shared passphrase gate used when RealMe SAML2 SSO is not
 * enabled (`NEXT_PUBLIC_REALME_ENABLED=false`). When `STAFF_PASSWORD` is unset,
 * real sign-in is disabled and the user is directed to demo mode. Real sign-in
 * via RealMe is handled by the `/login/realme` + callback routes (see
 * `realme.ts`).
 *
 * `formData` is a server-action FormData with `staffId` and `password`.
 */
export async function loginStaff(formData: FormData) {
  const staffId = String(formData.get("staffId") ?? "").trim();
  const password = String(formData.get("password") ?? "");
  const limiterKey = staffId.toLowerCase() || "anonymous";

  if (isLocked(limiterKey)) {
    redirect("/login?error=locked");
  }

  const expected = process.env.STAFF_PASSWORD;
  if (!expected || !staffId || !passwordMatches(password, expected)) {
    recordFailure(limiterKey);
    redirect("/login?error=invalid");
  }
  clearFailures(limiterKey);

  const token = createStaffSessionToken({
    staffId,
    displayName: staffId,
    sessionId: randomToken(16),
    demo: false,
  });
  await setStaffSessionCookie(token);
  redirect("/citizens");
}

/**
 * Sign out of the staff session. The local session always ends first. For a RealMe
 * sign-in, when Single Logout is configured, the browser is then sent to the IdP with a
 * signed LogoutRequest. Server actions are origin-checked by Next.js, so this cannot be
 * triggered from another site the way a GET link could.
 */
export async function logoutStaff() {
  const session = await getStaffSession();
  await clearStaffSessionCookie();

  let idpRedirect: string | undefined;
  const cfg = getRealmeConfig();
  if (session?.realmeNameId && cfg.sloUrl && cfg.idpCert && cfg.idpEntityId) {
    try {
      const { redirectUrl, requestId } = buildLogoutRequest({
        nameId: session.realmeNameId,
        sessionIndex: session.realmeSessionIndex,
      });
      const store = await cookies();
      store.set(REALME_SLO_COOKIE, createRealmeState(requestId, 600), {
        httpOnly: true,
        sameSite: "lax",
        secure: secureCookies(),
        path: "/login/realme",
        maxAge: 600,
      });
      idpRedirect = redirectUrl;
    } catch {
      // Signing misconfigured: fall back to local sign-out rather than failing the action.
      idpRedirect = undefined;
    }
  }
  // redirect() throws to end the action, so it must stay outside the try block above.
  if (idpRedirect) {
    // External IdP URL, built and signed in realme.ts; typed-routes only knows internal paths.
    redirect(idpRedirect as Parameters<typeof redirect>[0]);
  }
  redirect("/login");
}
