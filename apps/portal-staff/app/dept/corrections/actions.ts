"use server";

import type { CORRECTIONSDataBundle } from "@tpt/gov-schema";

const CORRECTIONS_SERVICE_URL = process.env.CORRECTIONS_SERVICE_URL ?? "http://localhost:8104";

/**
 * Read-only fetch of a citizen's Corrections data, performed by a case worker.
 */
export async function fetchCorrectionsDataForCitizen(
  did: string,
  scopes: string[],
): Promise<CORRECTIONSDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${CORRECTIONS_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<CORRECTIONSDataBundle>;
  } catch {
    return null;
  }
}
