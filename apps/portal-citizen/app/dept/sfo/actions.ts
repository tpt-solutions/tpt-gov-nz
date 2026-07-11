"use server";

import type { SfoDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceSfoAiContext } from "./sfo-ai";

const SFO_SERVICE_URL = process.env.SFO_SERVICE_URL ?? "http://localhost:8124";

export async function fetchSfoData(scopes: string[]): Promise<SfoDataBundle | null> {
  return (await fetchDeptData("sfo", scopes)) as SfoDataBundle | null;
}

export async function submitSfoAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(SFO_SERVICE_URL + "/citizen/action", {
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

export async function askSfo(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchSfoData(["sfo:investigations", "sfo:outcomes"]);
  if (!data) {
    return { answer: "Unable to load your Serious Fraud Office information.", enabled: false };
  }
  const context = produceSfoAiContext(data);
  return askWithContext(question, context);
}
