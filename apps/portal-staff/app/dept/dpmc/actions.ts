"use server";

import type { DpmcDataBundle } from "@tpt/gov-schema";

const DPMC_SERVICE_URL = process.env.DPMC_SERVICE_URL ?? "http://localhost:8121";

/**
 * Read-only fetch of a citizen's Department of the Prime Minister and Cabinet data, performed by a case worker.
 */
export async function fetchDpmcDataForCitizen(
  did: string,
  scopes: string[],
): Promise<DpmcDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(DPMC_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<DpmcDataBundle>;
  } catch {
    return null;
  }
}
