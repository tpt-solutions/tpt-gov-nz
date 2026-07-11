"use server";

import type { PacificDataBundle } from "@tpt/gov-schema";

const PACIFIC_SERVICE_URL = process.env.PACIFIC_SERVICE_URL ?? "http://localhost:8127";

/**
 * Read-only fetch of a citizen's Ministry for Pacific Peoples data, performed by a case worker.
 */
export async function fetchPacificDataForCitizen(
  did: string,
  scopes: string[],
): Promise<PacificDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(PACIFIC_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<PacificDataBundle>;
  } catch {
    return null;
  }
}
