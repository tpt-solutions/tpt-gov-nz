"use server";

import type { CaaDataBundle } from "@tpt/gov-schema";

const CAA_SERVICE_URL = process.env.CAA_SERVICE_URL ?? "http://localhost:8136";

/**
 * Read-only fetch of a citizen's Civil Aviation Authority data, performed by a case worker.
 */
export async function fetchCaaDataForCitizen(
  did: string,
  scopes: string[],
): Promise<CaaDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(CAA_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<CaaDataBundle>;
  } catch {
    return null;
  }
}
