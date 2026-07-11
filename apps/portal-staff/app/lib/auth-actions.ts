"use server";

import { redirect } from "next/navigation";
import {
  createStaffSessionToken,
  setStaffSessionCookie,
  clearStaffSessionCookie,
} from "./session";
import { randomToken } from "./jwt";
import { STAFF_CONFIG } from "./config";

/**
 * Demo sign-in: establish a staff session for the fictional case worker without
 * requiring a real RealMe credential. Used only when `NEXT_PUBLIC_DEMO_MODE` is on.
 */
export async function loginStaffDemo() {
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
 * Real sign-in: a shared passphrase gate standing in for a RealMe SAML2 lookup
 * (tracked under "Production Hardening"). When `STAFF_PASSWORD` is unset, real
 * sign-in is disabled and the user is directed to demo mode.
 *
 * `formData` is a server-action FormData with `staffId` and `password`.
 */
export async function loginStaff(formData: FormData) {
  const staffId = String(formData.get("staffId") ?? "").trim();
  const password = String(formData.get("password") ?? "");

  const expected = process.env.STAFF_PASSWORD;
  if (!expected || !staffId || password !== expected) {
    redirect("/login?error=invalid");
  }

  const token = createStaffSessionToken({
    staffId,
    displayName: staffId,
    sessionId: randomToken(16),
    demo: false,
  });
  await setStaffSessionCookie(token);
  redirect("/citizens");
}

/** Sign out of the staff session. */
export async function logoutStaff() {
  await clearStaffSessionCookie();
  redirect("/login");
}
