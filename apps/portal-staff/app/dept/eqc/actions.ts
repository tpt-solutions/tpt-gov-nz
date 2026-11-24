"use server";

import { parseDid, parseScopes } from "@tpt/gov-schema";
import type { EqcDataBundle } from "@tpt/gov-schema";

const EQC_SERVICE_URL = process.env.EQC_SERVICE_URL ?? "http://localhost:8134";

/**
 * Read-only fetch of a citizen's Earthquake Commission (Toka Tū Ake) data, performed by a case worker.
 */
export async function fetchEqcDataForCitizen(
  did: string,
  scopes: string[],
): Promise<EqcDataBundle | null> {
  const validDid = parseDid(did);
  if (!validDid) return null;
  const validScopes = parseScopes(scopes);
  if (!validScopes) return null;
  did = validDid;
  scopes = validScopes;
  if (!did) return null;

  try {
    const res = await fetch(EQC_SERVICE_URL + "/citizen/data", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<EqcDataBundle>;
  } catch {
    return null;
  }
}
