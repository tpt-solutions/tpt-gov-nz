"use server";

import type { DIADataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceDiaAiContext } from "./dia-ai";

const DIA_SERVICE_URL = process.env.DIA_SERVICE_URL ?? "http://localhost:8093";

export async function fetchDiaData(scopes: string[]): Promise<DIADataBundle | null> {
  return (await fetchDeptData("dia", scopes)) as DIADataBundle | null;
}

export async function submitDiaAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${DIA_SERVICE_URL}/citizen/action`, {
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

// ── AI: "my passport is expiring, what do I do?" (Stage 6) ──────────────────────

export async function askDia(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchDiaData(["dia:passport", "dia:citizenship"]);
  if (!data) {
    return { answer: "Unable to load your DIA information.", enabled: false };
  }
  const context = produceDiaAiContext(data);
  return askWithContext(question, context);
}
