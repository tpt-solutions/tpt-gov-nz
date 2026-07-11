"use server";

import type { TearawhitiDataBundle } from "@tpt/gov-schema";

const TEARAWHITI_SERVICE_URL = process.env.TEARAWHITI_SERVICE_URL ?? "http://localhost:8129";

/**
 * Read-only fetch of a citizen's Te Arawhiti data, performed by a case worker.
 */
export async function fetchTearawhitiDataForCitizen(
  did: string,
  scopes: string[],
): Promise<TearawhitiDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(TEARAWHITI_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<TearawhitiDataBundle>;
  } catch {
    return null;
  }
}
