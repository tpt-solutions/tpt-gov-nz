"use server";

import type { RegulationDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceRegulationAiContext } from "./regulation-ai";

const REGULATION_SERVICE_URL = process.env.REGULATION_SERVICE_URL ?? "http://localhost:8130";

export async function fetchRegulationData(scopes: string[]): Promise<RegulationDataBundle | null> {
  return (await fetchDeptData("regulation", scopes)) as RegulationDataBundle | null;
}

export async function submitRegulationAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(REGULATION_SERVICE_URL + "/citizen/action", {
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

export async function askRegulation(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchRegulationData(["regulation:regulatory-reviews", "regulation:proposals"]);
  if (!data) {
    return { answer: "Unable to load your Ministry for Regulation information.", enabled: false };
  }
  const context = produceRegulationAiContext(data);
  return askWithContext(question, context);
}
