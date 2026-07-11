"use server";

import { cookies } from "next/headers";
import { redirect } from "next/navigation";
import { createSessionToken, setSessionCookie, clearSessionCookie } from "./session";
import { PORTAL_CONFIG, DEPARTMENTS, DEMO_DID, type DeptId } from "./config";
import { setDemoScenario } from "./demo";
import type { ScenarioId } from "./mock-data";
import { randomToken } from "./jwt";

const ALL_SCOPES = DEPARTMENTS.flatMap((d) => d.scopes);

/** Demo sign-in: set a demo session for Alex Tane and pick a scenario. */
export async function loginDemo(scenario: ScenarioId) {
  await setDemoScenario(scenario);
  const token = createSessionToken({
    did: DEMO_DID,
    sessionId: randomToken(16),
    grantedScopes: [...ALL_SCOPES],
    demo: true,
  });
  await setSessionCookie(token);
  redirect("/dashboard");
}

/** Switch the active demo scenario (persists in a cookie). */
export async function setScenario(scenario: ScenarioId) {
  await setDemoScenario(scenario);
  redirect("/dashboard");
}

/** Sign out of any session (real or demo). */
export async function logout() {
  await clearSessionCookie();
  redirect("/");
}

export async function isAuthenticated(): Promise<boolean> {
  const store = await cookies();
  return store.has(PORTAL_CONFIG.sessionCookieName);
}
