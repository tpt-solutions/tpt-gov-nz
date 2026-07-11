"use server";

import type { ACCDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceAccAiContext } from "./acc-ai";

const ACC_SERVICE_URL = process.env.ACC_SERVICE_URL ?? "http://localhost:8095";

export async function fetchAccData(scopes: string[]): Promise<ACCDataBundle | null> {
  return (await fetchDeptData("acc", scopes)) as ACCDataBundle | null;
}

export async function submitAccAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${ACC_SERVICE_URL}/citizen/action`, {
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

// ── AI: "what is the status of my ACC claim?" (Stage 6) ───────────────────────

export async function askAcc(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchAccData(["acc:claims", "acc:entitlements", "acc:rehabilitation"]);
  if (!data) {
    return { answer: "Unable to load your ACC information.", enabled: false };
  }
  const context = produceAccAiContext(data);
  return askWithContext(question, context);
}
