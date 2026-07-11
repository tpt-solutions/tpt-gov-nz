"use server";

import type { GcsbDataBundle } from "@tpt/gov-schema";

const GCSB_SERVICE_URL = process.env.GCSB_SERVICE_URL ?? "http://localhost:8146";

/**
 * Read-only fetch of a citizen's Government Communications Security Bureau data, performed by a case worker.
 */
export async function fetchGcsbDataForCitizen(
  did: string,
  scopes: string[],
): Promise<GcsbDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(GCSB_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<GcsbDataBundle>;
  } catch {
    return null;
  }
}
