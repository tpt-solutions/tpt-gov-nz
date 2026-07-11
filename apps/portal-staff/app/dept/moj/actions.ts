"use server";

import type { MOJDataBundle } from "@tpt/gov-schema";

const MOJ_SERVICE_URL = process.env.MOJ_SERVICE_URL ?? "http://localhost:8096";

/**
 * Read-only fetch of a citizen's MOJ data, performed by a case worker.
 */
export async function fetchMojDataForCitizen(
  did: string,
  scopes: string[],
): Promise<MOJDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${MOJ_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<MOJDataBundle>;
  } catch {
    return null;
  }
}
