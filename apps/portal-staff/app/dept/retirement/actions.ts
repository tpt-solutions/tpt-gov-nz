"use server";

import type { RetirementDataBundle } from "@tpt/gov-schema";

const RETIREMENT_SERVICE_URL = process.env.RETIREMENT_SERVICE_URL ?? "http://localhost:8132";

/**
 * Read-only fetch of a citizen's Retirement Commission (Te Ara Ahunga Ora) data, performed by a case worker.
 */
export async function fetchRetirementDataForCitizen(
  did: string,
  scopes: string[],
): Promise<RetirementDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(RETIREMENT_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<RetirementDataBundle>;
  } catch {
    return null;
  }
}
