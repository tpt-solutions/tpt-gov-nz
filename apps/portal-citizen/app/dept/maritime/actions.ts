"use server";

import type { MaritimeDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceMaritimeAiContext } from "./maritime-ai";

const MARITIME_SERVICE_URL = process.env.MARITIME_SERVICE_URL ?? "http://localhost:8137";

export async function fetchMaritimeData(scopes: string[]): Promise<MaritimeDataBundle | null> {
  return (await fetchDeptData("maritime", scopes)) as MaritimeDataBundle | null;
}

export async function submitMaritimeAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(MARITIME_SERVICE_URL + "/citizen/action", {
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

export async function askMaritime(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchMaritimeData(["maritime:vessels", "maritime:incidents"]);
  if (!data) {
    return { answer: "Unable to load your Maritime New Zealand information.", enabled: false };
  }
  const context = produceMaritimeAiContext(data);
  return askWithContext(question, context);
}
