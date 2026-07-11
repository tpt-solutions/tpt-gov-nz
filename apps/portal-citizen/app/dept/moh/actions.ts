"use server";

import type { MOHDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceMohAiContext } from "./moh-ai";

const MOH_SERVICE_URL = process.env.MOH_SERVICE_URL ?? "http://localhost:8092";

export async function fetchMohData(scopes: string[]): Promise<MOHDataBundle | null> {
  return (await fetchDeptData("moh", scopes)) as MOHDataBundle | null;
}

export async function submitMohAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${MOH_SERVICE_URL}/citizen/action`, {
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

// ── AI health navigation (Stage 6) ────────────────────────────────────────────

export async function askMohHealth(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchMohData(["moh:gp", "moh:prescriptions", "moh:vaccinations"]);
  if (!data) {
    return { answer: "Unable to load your health information.", enabled: false };
  }
  const context = produceMohAiContext(data);
  return askWithContext(question, context);
}
