"use server";

import type { HUDDataBundle } from "@tpt/gov-schema";

const HUD_SERVICE_URL = process.env.HUD_SERVICE_URL ?? "http://localhost:8098";

/**
 * Read-only fetch of a citizen's housing data, performed by a case worker.
 */
export async function fetchHudDataForCitizen(
  did: string,
  scopes: string[],
): Promise<HUDDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${HUD_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<HUDDataBundle>;
  } catch {
    return null;
  }
}
