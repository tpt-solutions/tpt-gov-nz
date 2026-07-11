import { promises as fs } from "node:fs";
import path from "node:path";
import { randomUUID } from "node:crypto";
import type { StaffDeptId } from "./config";

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

const DATA_DIR = path.join(process.cwd(), ".data");
const REFERRALS_FILE = path.join(DATA_DIR, "staff-referrals.json");

async function readAll(): Promise<Referral[]> {
  try {
    const raw = await fs.readFile(REFERRALS_FILE, "utf8");
    const parsed = JSON.parse(raw) as Referral[];
    return Array.isArray(parsed) ? parsed : [];
  } catch {
    return [];
  }
}

async function writeAll(referrals: Referral[]): Promise<void> {
  await fs.mkdir(DATA_DIR, { recursive: true });
  await fs.writeFile(REFERRALS_FILE, JSON.stringify(referrals, null, 2), "utf8");
}

export async function listReferrals(): Promise<Referral[]> {
  const all = await readAll();
  return all.sort((a, b) => b.createdAt.localeCompare(a.createdAt));
}

export async function listReferralsForCitizen(did: string): Promise<Referral[]> {
  return (await listReferrals()).filter((r) => r.did === did);
}

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
  const all = await readAll();
  all.push(entry);
  await writeAll(all);
  return entry;
}
