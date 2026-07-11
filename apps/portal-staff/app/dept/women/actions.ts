"use server";

import type { WomenDataBundle } from "@tpt/gov-schema";

const WOMEN_SERVICE_URL = process.env.WOMEN_SERVICE_URL ?? "http://localhost:8126";

/**
 * Read-only fetch of a citizen's Ministry for Women data, performed by a case worker.
 */
export async function fetchWomenDataForCitizen(
  did: string,
  scopes: string[],
): Promise<WomenDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(WOMEN_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<WomenDataBundle>;
  } catch {
    return null;
  }
}
