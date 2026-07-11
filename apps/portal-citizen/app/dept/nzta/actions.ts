"use server";

import type { NZTADataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceNztaAiContext } from "./nzta-ai";

const NZTA_SERVICE_URL = process.env.NZTA_SERVICE_URL ?? "http://localhost:8094";

export async function fetchNztaData(scopes: string[]): Promise<NZTADataBundle | null> {
  return (await fetchDeptData("nzta", scopes)) as NZTADataBundle | null;
}

export async function submitNztaAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${NZTA_SERVICE_URL}/citizen/action`, {
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

// ── AI: "is my licence about to expire, and what can I do?" (Stage 6) ──────────

export async function askNzta(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchNztaData(["nzta:driver-licence", "nzta:vehicles", "nzta:ruc"]);
  if (!data) {
    return { answer: "Unable to load your NZTA information.", enabled: false };
  }
  const context = produceNztaAiContext(data);
  return askWithContext(question, context);
}
