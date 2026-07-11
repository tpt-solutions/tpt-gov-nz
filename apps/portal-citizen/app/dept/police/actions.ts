"use server";

import type { PoliceDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { producePoliceAiContext } from "./police-ai";

const POLICE_SERVICE_URL = process.env.POLICE_SERVICE_URL ?? "http://localhost:8097";

export async function fetchPoliceData(scopes: string[]): Promise<PoliceDataBundle | null> {
  return (await fetchDeptData("police", scopes)) as PoliceDataBundle | null;
}

export async function submitPoliceAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${POLICE_SERVICE_URL}/citizen/action`, {
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

// ── AI: "do I have any unpaid infringements?" (Stage 6) ────────────────────────

export async function askPolice(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchPoliceData(["police:infringements", "police:reports"]);
  if (!data) {
    return { answer: "Unable to load your Police information.", enabled: false };
  }
  const context = producePoliceAiContext(data);
  return askWithContext(question, context);
}
