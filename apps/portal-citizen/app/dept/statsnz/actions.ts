"use server";

import type { StatsNZDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceStatsnzAiContext } from "./statsnz-ai";

const STATSNZ_SERVICE_URL = process.env.STATSNZ_SERVICE_URL ?? "http://localhost:8103";

export async function fetchStatsnzData(scopes: string[]): Promise<StatsNZDataBundle | null> {
  return (await fetchDeptData("statsnz", scopes)) as StatsNZDataBundle | null;
}

export async function submitStatsnzAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${STATSNZ_SERVICE_URL}/citizen/action`, {
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

// ── AI: "what census data is held about me?" (Stage 6) ─────────────

export async function askStatsnz(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchStatsnzData(["statsnz:census", "statsnz:profile"]);
  if (!data) {
    return { answer: "Unable to load your statistics information.", enabled: false };
  }
  const context = produceStatsnzAiContext(data);
  return askWithContext(question, context);
}
