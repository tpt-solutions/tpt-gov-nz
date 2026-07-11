"use server";

import type { RetirementDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceRetirementAiContext } from "./retirement-ai";

const RETIREMENT_SERVICE_URL = process.env.RETIREMENT_SERVICE_URL ?? "http://localhost:8132";

export async function fetchRetirementData(scopes: string[]): Promise<RetirementDataBundle | null> {
  return (await fetchDeptData("retirement", scopes)) as RetirementDataBundle | null;
}

export async function submitRetirementAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(RETIREMENT_SERVICE_URL + "/citizen/action", {
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

export async function askRetirement(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchRetirementData(["retirement:retirement-plan", "retirement:guidance"]);
  if (!data) {
    return { answer: "Unable to load your Retirement Commission (Te Ara Ahunga Ora) information.", enabled: false };
  }
  const context = produceRetirementAiContext(data);
  return askWithContext(question, context);
}
