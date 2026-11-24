"use server";

import { parseDid, parseScopes } from "@tpt/gov-schema";
import type { EroDataBundle } from "@tpt/gov-schema";

const ERO_SERVICE_URL = process.env.ERO_SERVICE_URL ?? "http://localhost:8140";

/**
 * Read-only fetch of a citizen's Education Review Office data, performed by a case worker.
 */
export async function fetchEroDataForCitizen(
  did: string,
  scopes: string[],
): Promise<EroDataBundle | null> {
  const validDid = parseDid(did);
  if (!validDid) return null;
  const validScopes = parseScopes(scopes);
  if (!validScopes) return null;
  did = validDid;
  scopes = validScopes;
  if (!did) return null;

  try {
    const res = await fetch(ERO_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<EroDataBundle>;
  } catch {
    return null;
  }
}
