"use server";

import { parseDid, parseScopes } from "@tpt/gov-schema";
import type { NZTADataBundle } from "@tpt/gov-schema";

const NZTA_SERVICE_URL = process.env.NZTA_SERVICE_URL ?? "http://localhost:8094";

/**
 * Read-only fetch of a citizen's NZTA data, performed by a case worker.
 */
export async function fetchNztaDataForCitizen(
  did: string,
  scopes: string[],
): Promise<NZTADataBundle | null> {
  const validDid = parseDid(did);
  if (!validDid) return null;
  const validScopes = parseScopes(scopes);
  if (!validScopes) return null;
  did = validDid;
  scopes = validScopes;
  if (!did) return null;

  try {
    const res = await fetch(`${NZTA_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<NZTADataBundle>;
  } catch {
    return null;
  }
}
