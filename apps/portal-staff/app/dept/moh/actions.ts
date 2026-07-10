"use server";

import type { MOHDataBundle } from "@tpt/gov-schema";

const MOH_SERVICE_URL = process.env.MOH_SERVICE_URL ?? "http://localhost:8092";

/**
 * Read-only fetch of a citizen's health data, performed by a case worker.
 */
export async function fetchMohDataForCitizen(
  did: string,
  scopes: string[],
): Promise<MOHDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${MOH_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<MOHDataBundle>;
  } catch {
    return null;
  }
}
