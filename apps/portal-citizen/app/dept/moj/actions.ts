"use server";

import type { MOJDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceMojAiContext } from "./moj-ai";

const MOJ_SERVICE_URL = process.env.MOJ_SERVICE_URL ?? "http://localhost:8096";

export async function fetchMojData(scopes: string[]): Promise<MOJDataBundle | null> {
  return (await fetchDeptData("moj", scopes)) as MOJDataBundle | null;
}

export async function submitMojAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${MOJ_SERVICE_URL}/citizen/action`, {
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

// ── AI: "what fines or court matters do I have?" (Stage 6) ────────────────────

export async function askMoj(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchMojData(["moj:fines", "moj:disputes", "moj:court-records"]);
  if (!data) {
    return { answer: "Unable to load your Ministry of Justice information.", enabled: false };
  }
  const context = produceMojAiContext(data);
  return askWithContext(question, context);
}
