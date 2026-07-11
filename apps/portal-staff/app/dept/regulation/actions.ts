"use server";

import type { RegulationDataBundle } from "@tpt/gov-schema";

const REGULATION_SERVICE_URL = process.env.REGULATION_SERVICE_URL ?? "http://localhost:8130";

/**
 * Read-only fetch of a citizen's Ministry for Regulation data, performed by a case worker.
 */
export async function fetchRegulationDataForCitizen(
  did: string,
  scopes: string[],
): Promise<RegulationDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(REGULATION_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<RegulationDataBundle>;
  } catch {
    return null;
  }
}
