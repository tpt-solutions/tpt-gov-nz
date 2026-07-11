"use server";

import type { MchDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceMchAiContext } from "./mch-ai";

const MCH_SERVICE_URL = process.env.MCH_SERVICE_URL ?? "http://localhost:8142";

export async function fetchMchData(scopes: string[]): Promise<MchDataBundle | null> {
  return (await fetchDeptData("mch", scopes)) as MchDataBundle | null;
}

export async function submitMchAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(MCH_SERVICE_URL + "/citizen/action", {
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

export async function askMch(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchMchData(["mch:heritage-sites", "mch:grants"]);
  if (!data) {
    return { answer: "Unable to load your Ministry for Culture and Heritage information.", enabled: false };
  }
  const context = produceMchAiContext(data);
  return askWithContext(question, context);
}
