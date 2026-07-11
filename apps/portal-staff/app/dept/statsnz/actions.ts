"use server";

import type { StatsNZDataBundle } from "@tpt/gov-schema";

const STATSNZ_SERVICE_URL = process.env.STATSNZ_SERVICE_URL ?? "http://localhost:8103";

/**
 * Read-only fetch of a citizen's Statistics New Zealand data, performed by a case worker.
 */
export async function fetchStatsnzDataForCitizen(
  did: string,
  scopes: string[],
): Promise<StatsNZDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${STATSNZ_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<StatsNZDataBundle>;
  } catch {
    return null;
  }
}
