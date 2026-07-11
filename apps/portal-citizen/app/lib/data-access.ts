import { PORTAL_CONFIG, type DeptId } from "./config";
import { getCitizenDid } from "./session";
import { getDemoScenario } from "./demo";
import { getDemoData } from "./mock-data";

export type DeptBundle =
  | import("@tpt/gov-schema").IRDDataBundle
  | import("@tpt/gov-schema").WINZDataBundle
  | import("@tpt/gov-schema").MOHDataBundle
  | import("@tpt/gov-schema").DIADataBundle;

/**
 * Single data-access path used by every department module. In demo mode it
 * returns the seeded Alex Tane fixture for the active scenario; otherwise it
 * calls the department's native service directly with the signed-in DID.
 */
export async function fetchDeptData(dept: DeptId, scopes: string[]): Promise<DeptBundle | null> {
  if (PORTAL_CONFIG.demoMode) {
    const scenario = await getDemoScenario();
    const data = getDemoData(scenario);
    return data[dept] as DeptBundle;
  }

  const did = await getCitizenDid();
  if (!did) return null;

  const url = PORTAL_CONFIG.services[dept];
  try {
    const res = await fetch(`${url}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes }),
      next: { revalidate: 60 },
    });
    if (!res.ok) return null;
    return (await res.json()) as DeptBundle;
  } catch {
    return null;
  }
}
