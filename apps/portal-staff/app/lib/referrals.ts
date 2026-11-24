import { randomUUID } from "node:crypto";
import { staffDbConfigured, staffPool } from "./staff-db";
import { STAFF_CONFIG, type StaffDeptId } from "./config";

export type ReferralStatus = "open" | "accepted" | "closed";

export interface Referral {
  id: string;
  did: string;
  fromDept: StaffDeptId;
  toDept: StaffDeptId;
  reason: string;
  author: string;
  createdAt: string;
  status: ReferralStatus;
}

/**
 * Referrals are stored in the staff database. Rows are never deleted: a trigger
 * rejects DELETE. Each read is recorded in `referral_access` in the same
 * transaction as the read. A listing across all citizens is recorded with a null
 * `citizen_did`.
 */
const SCHEMA_SQL = `
CREATE TABLE IF NOT EXISTS referrals (
  id uuid PRIMARY KEY,
  citizen_did text NOT NULL CHECK (char_length(citizen_did) BETWEEN 1 AND 200),
  from_dept text NOT NULL CHECK (char_length(from_dept) BETWEEN 1 AND 40),
  to_dept text NOT NULL CHECK (char_length(to_dept) BETWEEN 1 AND 40),
  reason text NOT NULL CHECK (char_length(reason) BETWEEN 1 AND 2000),
  author text NOT NULL CHECK (char_length(author) BETWEEN 1 AND 200),
  status text NOT NULL DEFAULT 'open' CHECK (status IN ('open', 'accepted', 'closed')),
  created_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS referrals_citizen_idx ON referrals (citizen_did, created_at DESC);

CREATE TABLE IF NOT EXISTS referral_access (
  id bigserial PRIMARY KEY,
  citizen_did text,
  viewer text NOT NULL,
  accessed_at timestamptz NOT NULL DEFAULT now()
);

CREATE OR REPLACE FUNCTION reject_referral_delete() RETURNS trigger AS $$
BEGIN
  RAISE EXCEPTION 'referrals are not deleted';
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS referrals_no_delete ON referrals;
CREATE TRIGGER referrals_no_delete
  BEFORE DELETE ON referrals
  FOR EACH ROW EXECUTE FUNCTION reject_referral_delete();

CREATE OR REPLACE FUNCTION reject_referral_access_change() RETURNS trigger AS $$
BEGIN
  RAISE EXCEPTION 'referral_access is append-only';
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS referral_access_append_only ON referral_access;
CREATE TRIGGER referral_access_append_only
  BEFORE UPDATE OR DELETE ON referral_access
  FOR EACH ROW EXECUTE FUNCTION reject_referral_access_change();
`;

let schemaReady: Promise<void> | undefined;

function ensureSchema(): Promise<void> {
  schemaReady ??= staffPool()
    .query(SCHEMA_SQL)
    .then(() => undefined)
    .catch((err: unknown) => {
      schemaReady = undefined;
      throw err;
    });
  return schemaReady;
}

/** Demo mode without a database keeps referrals in memory only. */
const demoStore: Referral[] = [];

function demoStoreActive(): boolean {
  if (staffDbConfigured()) return false;
  if (STAFF_CONFIG.demoMode) return true;
  throw new Error("Staff referrals require TPT__GOV__STAFF_DATABASE_URL");
}

interface ReferralRow {
  id: string;
  citizen_did: string;
  from_dept: StaffDeptId;
  to_dept: StaffDeptId;
  reason: string;
  author: string;
  status: ReferralStatus;
  created_at: Date;
}

function toReferral(row: ReferralRow): Referral {
  return {
    id: row.id,
    did: row.citizen_did,
    fromDept: row.from_dept,
    toDept: row.to_dept,
    reason: row.reason,
    author: row.author,
    status: row.status,
    createdAt: row.created_at.toISOString(),
  };
}

const SELECT_COLUMNS =
  "id, citizen_did, from_dept, to_dept, reason, author, status, created_at";

async function readWithAudit(
  citizenDid: string | null,
  viewer: string,
  where: string,
  params: unknown[],
): Promise<Referral[]> {
  await ensureSchema();
  const client = await staffPool().connect();
  try {
    await client.query("BEGIN");
    await client.query("INSERT INTO referral_access (citizen_did, viewer) VALUES ($1, $2)", [
      citizenDid,
      viewer || "Case worker",
    ]);
    const result = await client.query<ReferralRow>(
      `SELECT ${SELECT_COLUMNS} FROM referrals ${where} ORDER BY created_at DESC`,
      params,
    );
    await client.query("COMMIT");
    return result.rows.map(toReferral);
  } catch (err) {
    await client.query("ROLLBACK").catch(() => undefined);
    throw err;
  } finally {
    client.release();
  }
}

/** All referrals, newest first. Recorded in the access log as a cross-citizen read. */
export async function listReferrals(viewer: string): Promise<Referral[]> {
  if (demoStoreActive()) {
    return [...demoStore].sort((a, b) => b.createdAt.localeCompare(a.createdAt));
  }
  return readWithAudit(null, viewer, "", []);
}

/** One citizen's referrals, newest first, recorded against the citizen. */
export async function listReferralsForCitizen(did: string, viewer: string): Promise<Referral[]> {
  if (demoStoreActive()) {
    return demoStore
      .filter((r) => r.did === did)
      .sort((a, b) => b.createdAt.localeCompare(a.createdAt));
  }
  return readWithAudit(did, viewer, "WHERE citizen_did = $1", [did]);
}

/** Create a referral. Status starts as `open`. */
export async function addReferral(
  did: string,
  fromDept: StaffDeptId,
  toDept: StaffDeptId,
  reason: string,
  author: string,
): Promise<Referral> {
  const entry: Referral = {
    id: randomUUID(),
    did,
    fromDept,
    toDept,
    reason,
    author: author || "Case worker",
    createdAt: new Date().toISOString(),
    status: "open",
  };

  if (demoStoreActive()) {
    demoStore.push(entry);
    return entry;
  }

  await ensureSchema();
  await staffPool().query(
    "INSERT INTO referrals (id, citizen_did, from_dept, to_dept, reason, author, status, created_at) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    [
      entry.id,
      entry.did,
      entry.fromDept,
      entry.toDept,
      entry.reason,
      entry.author,
      entry.status,
      entry.createdAt,
    ],
  );
  return entry;
}
