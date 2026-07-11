"use server";

import type { PoliceDataBundle } from "@tpt/gov-schema";

const POLICE_SERVICE_URL = process.env.POLICE_SERVICE_URL ?? "http://localhost:8097";

/**
 * Read-only fetch of a citizen's Police data, performed by a case worker.
 */
export async function fetchPoliceDataForCitizen(
  did: string,
  scopes: string[],
): Promise<PoliceDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${POLICE_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<PoliceDataBundle>;
  } catch {
    return null;
  }
}
