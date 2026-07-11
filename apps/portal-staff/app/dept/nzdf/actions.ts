"use server";

import type { NzdfDataBundle } from "@tpt/gov-schema";

const NZDF_SERVICE_URL = process.env.NZDF_SERVICE_URL ?? "http://localhost:8145";

/**
 * Read-only fetch of a citizen's New Zealand Defence Force data, performed by a case worker.
 */
export async function fetchNzdfDataForCitizen(
  did: string,
  scopes: string[],
): Promise<NzdfDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(NZDF_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<NzdfDataBundle>;
  } catch {
    return null;
  }
}
