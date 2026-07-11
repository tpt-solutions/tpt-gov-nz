"use server";

import type { MchDataBundle } from "@tpt/gov-schema";

const MCH_SERVICE_URL = process.env.MCH_SERVICE_URL ?? "http://localhost:8142";

/**
 * Read-only fetch of a citizen's Ministry for Culture and Heritage data, performed by a case worker.
 */
export async function fetchMchDataForCitizen(
  did: string,
  scopes: string[],
): Promise<MchDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(MCH_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<MchDataBundle>;
  } catch {
    return null;
  }
}
