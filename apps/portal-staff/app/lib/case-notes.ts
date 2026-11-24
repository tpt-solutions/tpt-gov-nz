import { randomUUID } from "node:crypto";
import { staffDbConfigured, staffPool } from "./staff-db";
import { STAFF_CONFIG } from "./config";

export interface CaseNote {
  id: string;
  did: string;
  author: string;
  note: string;
  createdAt: string;
}

/**
 * Case notes are append-only. A trigger rejects UPDATE and DELETE, so a note
 * cannot be changed or removed after it is written. Every read is recorded in
 * `case_note_access` in the same transaction as the read, so a note is never
 * returned without an audit row.
 */
const SCHEMA_SQL = `
CREATE TABLE IF NOT EXISTS case_notes (
  id uuid PRIMARY KEY,
  citizen_did text NOT NULL CHECK (char_length(citizen_did) BETWEEN 1 AND 200),
  author text NOT NULL CHECK (char_length(author) BETWEEN 1 AND 200),
  note text NOT NULL CHECK (char_length(note) BETWEEN 1 AND 4000),
  created_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX IF NOT EXISTS case_notes_citizen_idx ON case_notes (citizen_did, created_at DESC);

CREATE TABLE IF NOT EXISTS case_note_access (
  id bigserial PRIMARY KEY,
  citizen_did text NOT NULL,
  viewer text NOT NULL,
  accessed_at timestamptz NOT NULL DEFAULT now()
);

CREATE OR REPLACE FUNCTION reject_append_only_change() RETURNS trigger AS $$
BEGIN
  RAISE EXCEPTION '% is append-only', TG_TABLE_NAME;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS case_notes_append_only ON case_notes;
CREATE TRIGGER case_notes_append_only
  BEFORE UPDATE OR DELETE ON case_notes
  FOR EACH ROW EXECUTE FUNCTION reject_append_only_change();

DROP TRIGGER IF EXISTS case_note_access_append_only ON case_note_access;
CREATE TRIGGER case_note_access_append_only
  BEFORE UPDATE OR DELETE ON case_note_access
  FOR EACH ROW EXECUTE FUNCTION reject_append_only_change();
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

/**
 * Demo mode without a database keeps notes in memory for the life of the
 * process. Nothing is written to disk, and the store is not used for real data.
 */
const demoStore: CaseNote[] = [];

function demoStoreActive(): boolean {
  if (staffDbConfigured()) return false;
  if (STAFF_CONFIG.demoMode) return true;
  throw new Error("Staff case notes require TPT__GOV__STAFF_DATABASE_URL");
}

interface NoteRow {
  id: string;
  citizen_did: string;
  author: string;
  note: string;
  created_at: Date;
}

function toNote(row: NoteRow): CaseNote {
  return {
    id: row.id,
    did: row.citizen_did,
    author: row.author,
    note: row.note,
    createdAt: row.created_at.toISOString(),
  };
}

/** List a citizen's notes, newest first, recording the read against `viewer`. */
export async function listCaseNotes(did: string, viewer: string): Promise<CaseNote[]> {
  if (demoStoreActive()) {
    return demoStore
      .filter((n) => n.did === did)
      .sort((a, b) => b.createdAt.localeCompare(a.createdAt));
  }

  await ensureSchema();
  const client = await staffPool().connect();
  try {
    await client.query("BEGIN");
    await client.query(
      "INSERT INTO case_note_access (citizen_did, viewer) VALUES ($1, $2)",
      [did, viewer || "Case worker"],
    );
    const result = await client.query<NoteRow>(
      "SELECT id, citizen_did, author, note, created_at FROM case_notes WHERE citizen_did = $1 ORDER BY created_at DESC",
      [did],
    );
    await client.query("COMMIT");
    return result.rows.map(toNote);
  } catch (err) {
    await client.query("ROLLBACK").catch(() => undefined);
    throw err;
  } finally {
    client.release();
  }
}

/** Append a note to a citizen's case file. */
export async function addCaseNote(did: string, author: string, note: string): Promise<CaseNote> {
  const entry: CaseNote = {
    id: randomUUID(),
    did,
    author: author || "Case worker",
    note,
    createdAt: new Date().toISOString(),
  };

  if (demoStoreActive()) {
    demoStore.push(entry);
    return entry;
  }

  await ensureSchema();
  await staffPool().query(
    "INSERT INTO case_notes (id, citizen_did, author, note, created_at) VALUES ($1, $2, $3, $4, $5)",
    [entry.id, entry.did, entry.author, entry.note, entry.createdAt],
  );
  return entry;
}
