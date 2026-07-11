"use server";

import type { WorksafeDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceWorksafeAiContext } from "./worksafe-ai";

const WORKSAFE_SERVICE_URL = process.env.WORKSAFE_SERVICE_URL ?? "http://localhost:8131";

export async function fetchWorksafeData(scopes: string[]): Promise<WorksafeDataBundle | null> {
  return (await fetchDeptData("worksafe", scopes)) as WorksafeDataBundle | null;
}

export async function submitWorksafeAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(WORKSAFE_SERVICE_URL + "/citizen/action", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, type, parameters, performed_by: "citizen" }),
    });

    if (!res.ok) {
      const err = (await res.json()) as { error?: string };
      return { success: false, message: err.error ?? "Action failed" };
    }

    return res.json() as Promise<{ success: boolean; message?: string }>;
  } catch {
    return { success: false, message: "Service unavailable" };
  }
}

// ── AI ─────────────────────────────────────────────────────────────

export async function askWorksafe(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchWorksafeData(["worksafe:inspections", "worksafe:investigations"]);
  if (!data) {
    return { answer: "Unable to load your WorkSafe New Zealand information.", enabled: false };
  }
  const context = produceWorksafeAiContext(data);
  return askWithContext(question, context);
}
