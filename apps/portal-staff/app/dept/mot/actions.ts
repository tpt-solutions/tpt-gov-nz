"use server";

import type { MotDataBundle } from "@tpt/gov-schema";

const MOT_SERVICE_URL = process.env.MOT_SERVICE_URL ?? "http://localhost:8135";

/**
 * Read-only fetch of a citizen's Ministry of Transport data, performed by a case worker.
 */
export async function fetchMotDataForCitizen(
  did: string,
  scopes: string[],
): Promise<MotDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(MOT_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<MotDataBundle>;
  } catch {
    return null;
  }
}
