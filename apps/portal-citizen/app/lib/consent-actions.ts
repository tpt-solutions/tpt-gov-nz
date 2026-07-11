"use server";

import { revalidatePath } from "next/cache";
import { grantConsent, revokeConsent } from "./consent";
import { resetDemo } from "./demo";
import type { DeptId } from "./config";

export async function grantConsentAction(requestingDept: DeptId, providingDept: DeptId) {
  const result = await grantConsent(requestingDept, providingDept);
  revalidatePath("/consent");
  revalidatePath("/dashboard");
  return result;
}

export async function revokeConsentAction(grantId: string) {
  const result = await revokeConsent(grantId);
  revalidatePath("/consent");
  revalidatePath("/dashboard");
  return result;
}

export async function resetDemoAction() {
  await resetDemo();
  revalidatePath("/");
  revalidatePath("/dashboard");
}
