"use server";

import type { ACCDataBundle } from "@tpt/gov-schema";

const ACC_SERVICE_URL = process.env.ACC_SERVICE_URL ?? "http://localhost:8095";

/**
 * Read-only fetch of a citizen's ACC data, performed by a case worker.
 */
export async function fetchAccDataForCitizen(
  did: string,
  scopes: string[],
): Promise<ACCDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${ACC_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<ACCDataBundle>;
  } catch {
    return null;
  }
}
