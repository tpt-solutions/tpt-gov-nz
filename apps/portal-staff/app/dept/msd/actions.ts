"use server";

import type { MSDDataBundle } from "@tpt/gov-schema";

const MSD_SERVICE_URL = process.env.MSD_SERVICE_URL ?? "http://localhost:8100";

/**
 * Read-only fetch of a citizen's MSD data, performed by a case worker.
 */
export async function fetchMsdDataForCitizen(
  did: string,
  scopes: string[],
): Promise<MSDDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${MSD_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<MSDDataBundle>;
  } catch {
    return null;
  }
}
