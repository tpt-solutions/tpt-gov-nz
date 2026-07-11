"use server";

import type { FenzDataBundle } from "@tpt/gov-schema";

const FENZ_SERVICE_URL = process.env.FENZ_SERVICE_URL ?? "http://localhost:8138";

/**
 * Read-only fetch of a citizen's Fire and Emergency New Zealand data, performed by a case worker.
 */
export async function fetchFenzDataForCitizen(
  did: string,
  scopes: string[],
): Promise<FenzDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(FENZ_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<FenzDataBundle>;
  } catch {
    return null;
  }
}
