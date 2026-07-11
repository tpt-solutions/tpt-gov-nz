"use server";

import type { EthnicDataBundle } from "@tpt/gov-schema";

const ETHNIC_SERVICE_URL = process.env.ETHNIC_SERVICE_URL ?? "http://localhost:8128";

/**
 * Read-only fetch of a citizen's Ministry for Ethnic Communities data, performed by a case worker.
 */
export async function fetchEthnicDataForCitizen(
  did: string,
  scopes: string[],
): Promise<EthnicDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(ETHNIC_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<EthnicDataBundle>;
  } catch {
    return null;
  }
}
