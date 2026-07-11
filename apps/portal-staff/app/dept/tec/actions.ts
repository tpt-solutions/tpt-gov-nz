"use server";

import type { TecDataBundle } from "@tpt/gov-schema";

const TEC_SERVICE_URL = process.env.TEC_SERVICE_URL ?? "http://localhost:8141";

/**
 * Read-only fetch of a citizen's Tertiary Education Commission data, performed by a case worker.
 */
export async function fetchTecDataForCitizen(
  did: string,
  scopes: string[],
): Promise<TecDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(TEC_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<TecDataBundle>;
  } catch {
    return null;
  }
}
