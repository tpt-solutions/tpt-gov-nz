"use server";

import type { MfatDataBundle } from "@tpt/gov-schema";

const MFAT_SERVICE_URL = process.env.MFAT_SERVICE_URL ?? "http://localhost:8143";

/**
 * Read-only fetch of a citizen's Ministry of Foreign Affairs and Trade data, performed by a case worker.
 */
export async function fetchMfatDataForCitizen(
  did: string,
  scopes: string[],
): Promise<MfatDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(MFAT_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<MfatDataBundle>;
  } catch {
    return null;
  }
}
