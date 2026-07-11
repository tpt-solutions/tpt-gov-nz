"use server";

import type { DefenceDataBundle } from "@tpt/gov-schema";

const DEFENCE_SERVICE_URL = process.env.DEFENCE_SERVICE_URL ?? "http://localhost:8144";

/**
 * Read-only fetch of a citizen's Ministry of Defence data, performed by a case worker.
 */
export async function fetchDefenceDataForCitizen(
  did: string,
  scopes: string[],
): Promise<DefenceDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(DEFENCE_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<DefenceDataBundle>;
  } catch {
    return null;
  }
}
