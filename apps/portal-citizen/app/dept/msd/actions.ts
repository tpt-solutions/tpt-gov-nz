"use server";

import type { MSDDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceMsdAiContext } from "./msd-ai";

const MSD_SERVICE_URL = process.env.MSD_SERVICE_URL ?? "http://localhost:8100";

export async function fetchMsdData(scopes: string[]): Promise<MSDDataBundle | null> {
  return (await fetchDeptData("msd", scopes)) as MSDDataBundle | null;
}

export async function submitMsdAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${MSD_SERVICE_URL}/citizen/action`, {
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

// ── AI: "what student support am I entitled to?" (Stage 6) ─────────────

export async function askMsd(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchMsdData(["msd:studylink", "msd:case-history"]);
  if (!data) {
    return { answer: "Unable to load your study and case information.", enabled: false };
  }
  const context = produceMsdAiContext(data);
  return askWithContext(question, context);
}
