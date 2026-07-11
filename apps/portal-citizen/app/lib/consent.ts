import { cookies } from "next/headers";
import { PORTAL_CONFIG, DEPARTMENTS, type DeptId } from "./config";
import { getSession } from "./session";

export interface GrantView {
  id: string;
  requestingDeptId: string;
  providingDeptId: string;
  scopes: string[];
  issuedAt: string;
  expiresAt: string;
  demo?: boolean;
}

const GRANTS_COOKIE = "tpt_grants";
const GRANT_TTL = 60 * 60 * 24 * 365;

function timeout(ms = 2000): AbortSignal {
  return AbortSignal.timeout(ms);
}

function toView(g: Record<string, unknown>): GrantView {
  return {
    id: String(g.id),
    requestingDeptId: String(g.requesting_dept_id ?? g.requestingDeptId),
    providingDeptId: String(g.providing_dept_id ?? g.providingDeptId),
    scopes: Array.isArray(g.scopes) ? (g.scopes as string[]) : [],
    issuedAt: String(g.issued_at ?? g.issuedAt ?? ""),
    expiresAt: String(g.expires_at ?? g.expiresAt ?? ""),
    demo: g.demo === true,
  };
}

async function readCookieGrants(): Promise<GrantView[]> {
  const store = await cookies();
  const raw = store.get(GRANTS_COOKIE)?.value;
  if (!raw) return [];
  try {
    return JSON.parse(raw) as GrantView[];
  } catch {
    return [];
  }
}

/** List the citizen's active data-sharing consents (identity server, else cookie). */
export async function listGrants(): Promise<GrantView[]> {
  const session = await getSession();
  if (!session) return [];
  if (!PORTAL_CONFIG.demoMode) {
    try {
      const res = await fetch(
        `${PORTAL_CONFIG.identityServerUrl}/v1/grants?citizen_did=${encodeURIComponent(session.did)}`,
        { signal: timeout() },
      );
      if (res.ok) {
        const grants = (await res.json()) as Record<string, unknown>[];
        return grants.map(toView);
      }
    } catch {
      // fall through to cookie store
    }
  }
  return readCookieGrants();
}

export async function grantConsent(
  requestingDept: DeptId,
  providingDept: DeptId,
): Promise<{ ok: boolean; error?: string }> {
  const session = await getSession();
  if (!session) return { ok: false, error: "Not authenticated" };
  const providing = DEPARTMENTS.find((d) => d.id === providingDept);
  if (!providing) return { ok: false, error: "Unknown department" };

  const payload = {
    citizen_did: session.did,
    requesting_dept_id: requestingDept,
    providing_dept_id: providingDept,
    scopes: providing.scopes,
    expires_in_seconds: GRANT_TTL,
  };

  if (!PORTAL_CONFIG.demoMode) {
    try {
      const res = await fetch(`${PORTAL_CONFIG.identityServerUrl}/v1/grants`, {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(payload),
        signal: timeout(),
      });
      if (res.ok) return { ok: true };
      return { ok: false, error: `Identity server rejected the grant (${res.status})` };
    } catch {
      // fall through to cookie store for offline / demo operation
    }
  }

  const store = await cookies();
  const grants = await readCookieGrants();
  const grant: GrantView = {
    id: crypto.randomUUID(),
    requestingDeptId: requestingDept,
    providingDeptId: providingDept,
    scopes: [...providing.scopes],
    issuedAt: new Date().toISOString(),
    expiresAt: new Date(Date.now() + GRANT_TTL * 1000).toISOString(),
    demo: true,
  };
  store.set(GRANTS_COOKIE, JSON.stringify([...grants, grant]), {
    httpOnly: false,
    sameSite: "lax",
    path: "/",
    maxAge: GRANT_TTL,
  });
  return { ok: true };
}

export async function revokeConsent(grantId: string): Promise<{ ok: boolean }> {
  if (!PORTAL_CONFIG.demoMode) {
    try {
      await fetch(`${PORTAL_CONFIG.identityServerUrl}/v1/grants/${grantId}`, {
        method: "DELETE",
        signal: timeout(),
      });
    } catch {
      // ignore — also cleared from cookie below
    }
  }
  const store = await cookies();
  const grants = await readCookieGrants();
  store.set(GRANTS_COOKIE, JSON.stringify(grants.filter((g) => g.id !== grantId)), {
    httpOnly: false,
    sameSite: "lax",
    path: "/",
    maxAge: GRANT_TTL,
  });
  return { ok: true };
}
