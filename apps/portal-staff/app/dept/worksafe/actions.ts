"use server";

import type { WorksafeDataBundle } from "@tpt/gov-schema";

const WORKSAFE_SERVICE_URL = process.env.WORKSAFE_SERVICE_URL ?? "http://localhost:8131";

/**
 * Read-only fetch of a citizen's WorkSafe New Zealand data, performed by a case worker.
 */
export async function fetchWorksafeDataForCitizen(
  did: string,
  scopes: string[],
): Promise<WorksafeDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(WORKSAFE_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<WorksafeDataBundle>;
  } catch {
    return null;
  }
}
