import { promises as fs } from "node:fs";
import path from "node:path";
import { randomUUID } from "node:crypto";

export interface CaseNote {
  id: string;
  did: string;
  author: string;
  note: string;
  createdAt: string;
}

const DATA_DIR = path.join(process.cwd(), ".data");
const NOTES_FILE = path.join(DATA_DIR, "staff-case-notes.json");

async function readAll(): Promise<CaseNote[]> {
  try {
    const raw = await fs.readFile(NOTES_FILE, "utf8");
    const parsed = JSON.parse(raw) as CaseNote[];
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

async function writeAll(notes: CaseNote[]): Promise<void> {
  await fs.mkdir(DATA_DIR, { recursive: true });
  await fs.writeFile(NOTES_FILE, JSON.stringify(notes, null, 2), "utf8");
}

export async function listCaseNotes(did: string): Promise<CaseNote[]> {
  const all = await readAll();
  return all
    .filter((n) => n.did === did)
    .sort((a, b) => b.createdAt.localeCompare(a.createdAt));
}

export async function addCaseNote(did: string, author: string, note: string): Promise<CaseNote> {
  const entry: CaseNote = {
    id: randomUUID(),
    did,
    author: author || "Case worker",
    note,
    createdAt: new Date().toISOString(),
  };
  const all = await readAll();
  all.push(entry);
  await writeAll(all);
  return entry;
}
