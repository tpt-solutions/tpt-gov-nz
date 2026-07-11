"use server";

import type { MotDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceMotAiContext } from "./mot-ai";

const MOT_SERVICE_URL = process.env.MOT_SERVICE_URL ?? "http://localhost:8135";

export async function fetchMotData(scopes: string[]): Promise<MotDataBundle | null> {
  return (await fetchDeptData("mot", scopes)) as MotDataBundle | null;
}

export async function submitMotAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(MOT_SERVICE_URL + "/citizen/action", {
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

export async function askMot(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchMotData(["mot:strategies", "mot:programmes"]);
  if (!data) {
    return { answer: "Unable to load your Ministry of Transport information.", enabled: false };
  }
  const context = produceMotAiContext(data);
  return askWithContext(question, context);
}
