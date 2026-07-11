"use server";

import type { DOCDataBundle } from "@tpt/gov-schema";

const DOC_SERVICE_URL = process.env.DOC_SERVICE_URL ?? "http://localhost:8107";

/**
 * Read-only fetch of a citizen's conservation data, performed by a case worker.
 */
export async function fetchDocDataForCitizen(
  did: string,
  scopes: string[],
): Promise<DOCDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${DOC_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<DOCDataBundle>;
  } catch {
    return null;
  }
}
