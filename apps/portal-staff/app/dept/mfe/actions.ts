"use server";

import type { MfeDataBundle } from "@tpt/gov-schema";

const MFE_SERVICE_URL = process.env.MFE_SERVICE_URL ?? "http://localhost:8133";

/**
 * Read-only fetch of a citizen's Ministry for the Environment data, performed by a case worker.
 */
export async function fetchMfeDataForCitizen(
  did: string,
  scopes: string[],
): Promise<MfeDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(MFE_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<MfeDataBundle>;
  } catch {
    return null;
  }
}
