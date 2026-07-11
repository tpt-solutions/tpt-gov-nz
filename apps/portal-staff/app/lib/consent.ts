import { STAFF_CONFIG, STAFF_DEPARTMENTS, type StaffDeptId } from "./config";

export interface DeptConsent {
  dept: StaffDeptId;
  granted: boolean;
  scopes: string[];
  /** True when derived from a live identity-server grant rather than a demo default. */
  live: boolean;
}

interface GrantResponse {
  id: string;
  citizen_did: string;
  requesting_dept_id: string;
  providing_dept_id: string;
  scopes: string[];
  issued_at: string;
  expires_at: string;
  signature: string;
}

function isUnexpired(expiresAt: string): boolean {
  const t = Date.parse(expiresAt);
  return Number.isFinite(t) && t > Date.now();
}

/**
 * Determine, per department, whether the case worker (requesting_dept_id "staff")
 * currently holds a valid consent grant from the citizen to view their data.
 *
 * In demo mode the fictional citizen is assumed to have pre-consented to every
 * department, so all departments return `granted`. In real mode the decision is
 * read live from the identity server's grant list.
 */
export async function checkStaffConsent(did: string): Promise<DeptConsent[]> {
  if (STAFF_CONFIG.demoMode) {
    return STAFF_DEPARTMENTS.map((d) => ({
      dept: d.id,
      granted: true,
      scopes: d.scopes,
      live: false,
    }));
  }

  let grants: GrantResponse[] = [];
  try {
    const res = await fetch(
      `${STAFF_CONFIG.identityServerUrl}/v1/grants?citizen_did=${encodeURIComponent(did)}`,
      { next: { revalidate: 30 } },
    );
    if (res.ok) grants = (await res.json()) as GrantResponse[];
  } catch {
    grants = [];
  }

  return STAFF_DEPARTMENTS.map((d) => {
    const covering = grants.filter(
      (g) =>
        g.requesting_dept_id === "staff" &&
        g.providing_dept_id === d.id &&
        isUnexpired(g.expires_at),
    );
    const scopes = Array.from(new Set(covering.flatMap((g) => g.scopes))).filter((s) =>
      d.scopes.includes(s),
    );
    return {
      dept: d.id,
      granted: scopes.length > 0,
      scopes,
      live: true,
    };
  });
}

/** Convenience: the set of department ids the case worker may view for `did`. */
export async function consentedDepartments(did: string): Promise<Set<StaffDeptId>> {
  const consents = await checkStaffConsent(did);
  return new Set(consents.filter((c) => c.granted).map((c) => c.dept));
}

/** Single-department consent lookup used by the department detail pages. */
export async function staffConsentForDept(
  did: string,
  dept: StaffDeptId,
): Promise<DeptConsent> {
  const consents = await checkStaffConsent(did);
  return (
    consents.find((c) => c.dept === dept) ?? {
      dept,
      granted: false,
      scopes: [],
      live: false,
    }
  );
}
