"use server";

import type { OrangaDataBundle } from "@tpt/gov-schema";

const ORANGA_SERVICE_URL = process.env.ORANGA_SERVICE_URL ?? "http://localhost:8125";

/**
 * Read-only fetch of a citizen's Oranga Tamariki data, performed by a case worker.
 */
export async function fetchOrangaDataForCitizen(
  did: string,
  scopes: string[],
): Promise<OrangaDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(ORANGA_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<OrangaDataBundle>;
  } catch {
    return null;
  }
}
