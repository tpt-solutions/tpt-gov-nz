"use server";

import type { WINZDataBundle } from "@tpt/gov-schema";

const WINZ_SERVICE_URL = process.env.WINZ_SERVICE_URL ?? "http://localhost:8091";

/**
 * Read-only fetch of a citizen's Work and Income data, performed by a case worker
 * on behalf of the citizen. No action buttons are rendered in the staff views.
 */
export async function fetchWinzDataForCitizen(
  did: string,
  scopes: string[],
): Promise<WINZDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${WINZ_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<WINZDataBundle>;
  } catch {
    return null;
  }
}
