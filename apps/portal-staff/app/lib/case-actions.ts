"use server";

import { revalidatePath } from "next/cache";
import { addCaseNote } from "./case-notes";
import { addReferral } from "./referrals";
import type { StaffDeptId } from "./config";
import { getStaffSession } from "./session";

export async function submitCaseNote(did: string, formData: FormData) {
  const session = await getStaffSession();
  const note = String(formData.get("note") ?? "").trim();
  const dept = String(formData.get("dept") ?? "");
  if (!did || !note) return;
  await addCaseNote(did, session?.displayName ?? "Case worker", note);
  revalidatePath(`/citizens/${encodeURIComponent(did)}`);
  if (dept) revalidatePath(`/dept/${dept}`);
}

export async function submitReferral(did: string, formData: FormData) {
  const session = await getStaffSession();
  const fromDept = String(formData.get("fromDept") ?? "") as StaffDeptId;
  const toDept = String(formData.get("toDept") ?? "") as StaffDeptId;
  const reason = String(formData.get("reason") ?? "").trim();
  if (!did || !fromDept || !toDept || !reason) return;
  await addReferral(did, fromDept, toDept, reason, session?.displayName ?? "Case worker");
  revalidatePath(`/citizens/${encodeURIComponent(did)}`);
  revalidatePath(`/referrals`);
}
