import { cookies } from "next/headers";
import { PORTAL_CONFIG } from "./config";
import type { ScenarioId } from "./mock-data";

export const DEFAULT_SCENARIO: ScenarioId = "standard";

const VALID: ScenarioId[] = ["standard", "beneficiary", "new-parent"];

export async function getDemoScenario(): Promise<ScenarioId> {
  const store = await cookies();
  const v = store.get(PORTAL_CONFIG.demoCookieName)?.value;
  if (v && (VALID as string[]).includes(v)) return v as ScenarioId;
  return DEFAULT_SCENARIO;
}

export async function setDemoScenario(scenario: ScenarioId): Promise<void> {
  const store = await cookies();
  store.set(PORTAL_CONFIG.demoCookieName, scenario, {
    httpOnly: false,
    sameSite: "lax",
    path: "/",
    maxAge: 60 * 60 * 24 * 30,
  });
}

export async function resetDemo(): Promise<void> {
  const store = await cookies();
  store.delete(PORTAL_CONFIG.demoCookieName);
}

export function isDemoMode(): boolean {
  return PORTAL_CONFIG.demoMode;
}
