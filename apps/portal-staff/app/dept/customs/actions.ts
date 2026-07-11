"use server";

import type { CUSTOMSDataBundle } from "@tpt/gov-schema";

const CUSTOMS_SERVICE_URL = process.env.CUSTOMS_SERVICE_URL ?? "http://localhost:8105";

/**
 * Read-only fetch of a citizen's customs data, performed by a case worker.
 */
export async function fetchCustomsDataForCitizen(
  did: string,
  scopes: string[],
): Promise<CUSTOMSDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${CUSTOMS_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<CUSTOMSDataBundle>;
  } catch {
    return null;
  }
}
