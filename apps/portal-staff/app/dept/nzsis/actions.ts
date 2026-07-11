"use server";

import type { NzsisDataBundle } from "@tpt/gov-schema";

const NZSIS_SERVICE_URL = process.env.NZSIS_SERVICE_URL ?? "http://localhost:8147";

/**
 * Read-only fetch of a citizen's New Zealand Security Intelligence Service data, performed by a case worker.
 */
export async function fetchNzsisDataForCitizen(
  did: string,
  scopes: string[],
): Promise<NzsisDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(NZSIS_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<NzsisDataBundle>;
  } catch {
    return null;
  }
}
