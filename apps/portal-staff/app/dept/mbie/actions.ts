"use server";

import type { MBIEDataBundle } from "@tpt/gov-schema";

const MBIE_SERVICE_URL = process.env.MBIE_SERVICE_URL ?? "http://localhost:8101";

/**
 * Read-only fetch of a citizen's business data, performed by a case worker.
 */
export async function fetchMbieDataForCitizen(
  did: string,
  scopes: string[],
): Promise<MBIEDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${MBIE_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<MBIEDataBundle>;
  } catch {
    return null;
  }
}
