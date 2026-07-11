import { STAFF_CONFIG, STAFF_DEPARTMENTS, type StaffDeptId } from "./config";
import type {
  IRDDataBundle,
  WINZDataBundle,
  MOHDataBundle,
  DIADataBundle,
  NZTADataBundle,
  ACCDataBundle,
  NZQADataBundle,
  MSDDataBundle,
  MBIEDataBundle,
  LINZDataBundle,
  StatsNZDataBundle,
  CORRECTIONSDataBundle,
  CUSTOMSDataBundle,
  MPIDataBundle,
  DOCDataBundle,
  TPKDataBundle,
} from "@tpt/gov-schema";

export type StaffBundle =
  | IRDDataBundle
  | WINZDataBundle
  | MOHDataBundle
  | DIADataBundle
  | NZTADataBundle
  | ACCDataBundle
  | NZQADataBundle
  | MSDDataBundle
  | MBIEDataBundle
  | LINZDataBundle
  | StatsNZDataBundle
  | CORRECTIONSDataBundle
  | CUSTOMSDataBundle
  | MPIDataBundle
  | DOCDataBundle
  | TPKDataBundle;

export interface DeptCaseResult {
  dept: StaffDeptId;
  consentGranted: boolean;
  scopes: string[];
  data: StaffBundle | null;
}

/** Read-only fetch of a citizen's department data on behalf of a case worker. */
async function fetchDeptBundle(
  dept: StaffDeptId,
  did: string,
  scopes: string[],
): Promise<StaffBundle | null> {
  if (!did) return null;
  const url = STAFF_CONFIG.services[dept];
  try {
    const res = await fetch(`${url}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        did,
        scopes,
        requesting_dept_id: "staff",
        performed_by: "staff",
      }),
      next: { revalidate: 60 },
    });
    if (!res.ok) return null;
    return (await res.json()) as StaffBundle;
  } catch {
    return null;
  }
}

/**
 * Build a cross-department case file for a citizen. Only departments for which
 * the case worker holds a valid consent grant are queried; the rest are returned
 * with `consentGranted: false` so the UI can show a consent-required notice.
 */
export async function fetchStaffCase(did: string): Promise<DeptCaseResult[]> {
  const { checkStaffConsent } = await import("./consent");
  const consents = await checkStaffConsent(did);

  return Promise.all(
    STAFF_DEPARTMENTS.map(async (meta) => {
      const consent = consents.find((c) => c.dept === meta.id);
      if (!consent || !consent.granted) {
        return { dept: meta.id, consentGranted: false, scopes: [], data: null };
      }
      const data = await fetchDeptBundle(meta.id, did, consent.scopes);
      return { dept: meta.id, consentGranted: true, scopes: consent.scopes, data };
    }),
  );
}
