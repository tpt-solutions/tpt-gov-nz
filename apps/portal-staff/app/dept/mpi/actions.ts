"use server";

import type { MPIDataBundle } from "@tpt/gov-schema";

const MPI_SERVICE_URL = process.env.MPI_SERVICE_URL ?? "http://localhost:8106";

/**
 * Read-only fetch of a citizen's MPI data, performed by a case worker.
 */
export async function fetchMpiDataForCitizen(
  did: string,
  scopes: string[],
): Promise<MPIDataBundle | null> {
  if (!did) return null;

  try {
    const res = await fetch(`${MPI_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes, requesting_dept_id: "staff", performed_by: "staff" }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<MPIDataBundle>;
  } catch {
    return null;
  }
}
