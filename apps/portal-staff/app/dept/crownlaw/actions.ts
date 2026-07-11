"use server";

import type { CrownlawDataBundle } from "@tpt/gov-schema";

const CROWNLAW_SERVICE_URL = process.env.CROWNLAW_SERVICE_URL ?? "http://localhost:8123";

/**
 * Read-only fetch of a citizen's Crown Law Office data, performed by a case worker.
 */
export async function fetchCrownlawDataForCitizen(
  did: string,
  scopes: string[],
): Promise<CrownlawDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(CROWNLAW_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<CrownlawDataBundle>;
  } catch {
    return null;
  }
}
