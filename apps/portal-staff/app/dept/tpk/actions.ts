"use server";

import type { TPKDataBundle } from "@tpt/gov-schema";

const TPK_SERVICE_URL = process.env.TPK_SERVICE_URL ?? "http://localhost:8108";

/**
 * Read-only fetch of a citizen's Te Puni Kōkiri data, performed by a case worker.
 */
export async function fetchTpkDataForCitizen(
  did: string,
  scopes: string[],
): Promise<TPKDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${TPK_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<TPKDataBundle>;
  } catch {
    return null;
  }
}
