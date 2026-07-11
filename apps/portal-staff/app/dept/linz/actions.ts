"use server";

import type { LINZDataBundle } from "@tpt/gov-schema";

const LINZ_SERVICE_URL = process.env.LINZ_SERVICE_URL ?? "http://localhost:8102";

/**
 * Read-only fetch of a citizen's land information, performed by a case worker.
 */
export async function fetchLinzDataForCitizen(
  did: string,
  scopes: string[],
): Promise<LINZDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${LINZ_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<LINZDataBundle>;
  } catch {
    return null;
  }
}
