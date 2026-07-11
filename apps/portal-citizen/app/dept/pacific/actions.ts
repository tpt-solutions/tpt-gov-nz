"use server";

import type { PacificDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { producePacificAiContext } from "./pacific-ai";

const PACIFIC_SERVICE_URL = process.env.PACIFIC_SERVICE_URL ?? "http://localhost:8127";

export async function fetchPacificData(scopes: string[]): Promise<PacificDataBundle | null> {
  return (await fetchDeptData("pacific", scopes)) as PacificDataBundle | null;
}

export async function submitPacificAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(PACIFIC_SERVICE_URL + "/citizen/action", {
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

export async function askPacific(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchPacificData(["pacific:programmes", "pacific:language-services"]);
  if (!data) {
    return { answer: "Unable to load your Ministry for Pacific Peoples information.", enabled: false };
  }
  const context = producePacificAiContext(data);
  return askWithContext(question, context);
}
