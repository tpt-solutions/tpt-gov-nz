"use server";

import type { IRDDataBundle } from "@tpt/gov-schema";

const IRD_SERVICE_URL = process.env.IRD_SERVICE_URL ?? "http://localhost:8090";

/**
 * Read-only fetch of a citizen's IRD data, performed by a case worker on behalf
 * of the citizen. No action buttons are rendered in the staff views, so this is
 * the only data-access path exposed to staff.
 */
export async function fetchIrdDataForCitizen(
  did: string,
  scopes: string[],
): Promise<IRDDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${IRD_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<IRDDataBundle>;
  } catch {
    return null;
  }
}
