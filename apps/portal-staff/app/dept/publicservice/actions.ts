"use server";

import type { PublicserviceDataBundle } from "@tpt/gov-schema";

const PUBLICSERVICE_SERVICE_URL = process.env.PUBLICSERVICE_SERVICE_URL ?? "http://localhost:8122";

/**
 * Read-only fetch of a citizen's Te Kawa Mataaho Public Service Commission data, performed by a case worker.
 */
export async function fetchPublicserviceDataForCitizen(
  did: string,
  scopes: string[],
): Promise<PublicserviceDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(PUBLICSERVICE_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<PublicserviceDataBundle>;
  } catch {
    return null;
  }
}
