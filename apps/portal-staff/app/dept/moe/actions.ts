"use server";

import type { MoeDataBundle } from "@tpt/gov-schema";

const MOE_SERVICE_URL = process.env.MOE_SERVICE_URL ?? "http://localhost:8139";

/**
 * Read-only fetch of a citizen's Ministry of Education data, performed by a case worker.
 */
export async function fetchMoeDataForCitizen(
  did: string,
  scopes: string[],
): Promise<MoeDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(MOE_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<MoeDataBundle>;
  } catch {
    return null;
  }
}
