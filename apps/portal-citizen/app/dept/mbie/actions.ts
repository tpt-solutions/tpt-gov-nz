"use server";

import type { MBIEDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceMbieAiContext } from "./mbie-ai";

const MBIE_SERVICE_URL = process.env.MBIE_SERVICE_URL ?? "http://localhost:8101";

export async function fetchMbieData(scopes: string[]): Promise<MBIEDataBundle | null> {
  return (await fetchDeptData("mbie", scopes)) as MBIEDataBundle | null;
}

export async function submitMbieAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${MBIE_SERVICE_URL}/citizen/action`, {
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

// ── AI: "what businesses am I registered for?" (Stage 6) ─────────────────────

export async function askMbie(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchMbieData(["mbie:business", "mbie:directorships"]);
  if (!data) {
    return { answer: "Unable to load your business information.", enabled: false };
  }
  const context = produceMbieAiContext(data);
  return askWithContext(question, context);
}
