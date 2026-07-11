"use server";

import type { SfoDataBundle } from "@tpt/gov-schema";

const SFO_SERVICE_URL = process.env.SFO_SERVICE_URL ?? "http://localhost:8124";

/**
 * Read-only fetch of a citizen's Serious Fraud Office data, performed by a case worker.
 */
export async function fetchSfoDataForCitizen(
  did: string,
  scopes: string[],
): Promise<SfoDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(SFO_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<SfoDataBundle>;
  } catch {
    return null;
  }
}
