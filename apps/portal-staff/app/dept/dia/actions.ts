"use server";

import type { DIADataBundle } from "@tpt/gov-schema";

const DIA_SERVICE_URL = process.env.DIA_SERVICE_URL ?? "http://localhost:8093";

/**
 * Read-only fetch of a citizen's DIA data, performed by a case worker.
 */
export async function fetchDiaDataForCitizen(
  did: string,
  scopes: string[],
): Promise<DIADataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${DIA_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<DIADataBundle>;
  } catch {
    return null;
  }
}
