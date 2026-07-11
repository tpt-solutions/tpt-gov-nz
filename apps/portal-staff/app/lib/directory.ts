import { STAFF_CONFIG, DEMO_CITIZEN_DID, DEMO_CITIZEN_NAME, type StaffDeptId } from "./config";

export interface DirectoryEntry {
  did: string;
  displayName: string;
  /** Department-local identifiers surfaced by each department's resolve endpoint. */
  deptIds: Partial<Record<StaffDeptId, string>>;
}

/** Demo directory: the single fictional citizen shared with the citizen portal. */
const DEMO_DIRECTORY: DirectoryEntry[] = [
  {
    did: DEMO_CITIZEN_DID,
    displayName: DEMO_CITIZEN_NAME,
    deptIds: {
      ird: "123-456-789",
      winz: "WINZ-000001",
      moh: "BHN1234",
      dia: "RE1234567",
      nzta: "NZ1234567",
      acc: "ACC-100001",
    },
  },
];

function matches(entry: DirectoryEntry, query: string): boolean {
  const q = query.trim().toLowerCase();
  if (!q) return true;
  return (
    entry.did.toLowerCase().includes(q) ||
    entry.displayName.toLowerCase().includes(q) ||
    Object.values(entry.deptIds).some((v) => v?.toLowerCase().includes(q))
  );
}

/**
 * Search the citizen directory. Demo mode returns the fictional citizen (name or
 * DID). Real mode resolves the query as a DID against the identity server and
 * probes each department's resolve endpoint for a department-local identifier.
 */
export async function searchCitizens(query: string): Promise<DirectoryEntry[]> {
  if (STAFF_CONFIG.demoMode) {
    return DEMO_DIRECTORY.filter((e) => matches(e, query));
  }

  const q = query.trim();
  if (!q || !q.startsWith("did:gov:nz:")) return [];

  const entry: DirectoryEntry = { did: q, displayName: q, deptIds: {} };
  const deptIds = await Promise.all(
    (Object.entries(STAFF_CONFIG.services) as [StaffDeptId, string][]).map(
      async ([dept, baseUrl]) => {
        try {
          const res = await fetch(`${baseUrl}/citizen/resolve`, {
            method: "POST",
            headers: { "Content-Type": "application/json" },
            body: JSON.stringify({ did: q }),
            next: { revalidate: 60 },
          });
          if (!res.ok) return [dept, undefined] as const;
          const data = (await res.json()) as { deptLocalId?: string; displayName?: string };
          if (data.displayName) entry.displayName = data.displayName;
          return [dept, data.deptLocalId] as const;
        } catch {
          return [dept, undefined] as const;
        }
      },
    ),
  );
  for (const [dept, localId] of deptIds) {
    if (localId) entry.deptIds[dept] = localId;
  }
  return [entry];
}
