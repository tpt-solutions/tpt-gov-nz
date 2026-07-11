"use server";

import type { TPKDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceTpkAiContext } from "./tpk-ai";

const TPK_SERVICE_URL = process.env.TPK_SERVICE_URL ?? "http://localhost:8108";

export async function fetchTpkData(scopes: string[]): Promise<TPKDataBundle | null> {
  return (await fetchDeptData("tpk", scopes)) as TPKDataBundle | null;
}

export async function submitTpkAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${TPK_SERVICE_URL}/citizen/action`, {
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

// ── AI: "what programmes am I enrolled in?" (Stage 6) ─────────────

export async function askTpk(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchTpkData(["tpk:programmes", "tpk:funding"]);
  if (!data) {
    return { answer: "Unable to load your Te Puni Kōkiri information.", enabled: false };
  }
  const context = produceTpkAiContext(data);
  return askWithContext(question, context);
}
