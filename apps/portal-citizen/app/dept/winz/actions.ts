"use server";

import type { WINZDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceWinzAiContext } from "./entitlement";

const WINZ_SERVICE_URL = process.env.WINZ_SERVICE_URL ?? "http://localhost:8091";

export async function fetchWinzData(scopes: string[]): Promise<WINZDataBundle | null> {
  return (await fetchDeptData("winz", scopes)) as WINZDataBundle | null;
}

export async function submitWinzAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${WINZ_SERVICE_URL}/citizen/action`, {
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

// ── AI benefit calculator (Stage 6) ────────────────────────────────────────────

export async function askWinzEntitlement(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchWinzData(["winz:benefits", "winz:payments"]);
  if (!data) {
    return { answer: "Unable to load your Work and Income information.", enabled: false };
  }
  const context = produceWinzAiContext(data);
  return askWithContext(question, context);
}
