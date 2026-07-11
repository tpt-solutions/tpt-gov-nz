"use server";

import type { LINZDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceLinzAiContext } from "./linz-ai";

const LINZ_SERVICE_URL = process.env.LINZ_SERVICE_URL ?? "http://localhost:8102";

export async function fetchLinzData(scopes: string[]): Promise<LINZDataBundle | null> {
  return (await fetchDeptData("linz", scopes)) as LINZDataBundle | null;
}

export async function submitLinzAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${LINZ_SERVICE_URL}/citizen/action`, {
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

// ── AI: "what property titles are in my name?" (Stage 6) ─────────────

export async function askLinz(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchLinzData(["linz:titles", "linz:ownership"]);
  if (!data) {
    return { answer: "Unable to load your land information.", enabled: false };
  }
  const context = produceLinzAiContext(data);
  return askWithContext(question, context);
}
