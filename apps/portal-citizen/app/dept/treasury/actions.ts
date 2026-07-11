"use server";

import type { TreasuryDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceTreasuryAiContext } from "./treasury-ai";

const TREASURY_SERVICE_URL = process.env.TREASURY_SERVICE_URL ?? "http://localhost:8120";

export async function fetchTreasuryData(scopes: string[]): Promise<TreasuryDataBundle | null> {
  return (await fetchDeptData("treasury", scopes)) as TreasuryDataBundle | null;
}

export async function submitTreasuryAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(TREASURY_SERVICE_URL + "/citizen/action", {
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

export async function askTreasury(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchTreasuryData(["treasury:budget", "treasury:economic-outlook"]);
  if (!data) {
    return { answer: "Unable to load your The Treasury information.", enabled: false };
  }
  const context = produceTreasuryAiContext(data);
  return askWithContext(question, context);
}
