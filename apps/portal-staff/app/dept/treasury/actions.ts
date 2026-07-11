"use server";

import type { TreasuryDataBundle } from "@tpt/gov-schema";

const TREASURY_SERVICE_URL = process.env.TREASURY_SERVICE_URL ?? "http://localhost:8120";

/**
 * Read-only fetch of a citizen's The Treasury data, performed by a case worker.
 */
export async function fetchTreasuryDataForCitizen(
  did: string,
  scopes: string[],
): Promise<TreasuryDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(TREASURY_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<TreasuryDataBundle>;
  } catch {
    return null;
  }
}
