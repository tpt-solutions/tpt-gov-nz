"use server";

import type { WomenDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceWomenAiContext } from "./women-ai";

const WOMEN_SERVICE_URL = process.env.WOMEN_SERVICE_URL ?? "http://localhost:8126";

export async function fetchWomenData(scopes: string[]): Promise<WomenDataBundle | null> {
  return (await fetchDeptData("women", scopes)) as WomenDataBundle | null;
}

export async function submitWomenAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(WOMEN_SERVICE_URL + "/citizen/action", {
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

export async function askWomen(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchWomenData(["women:programmes", "women:insights"]);
  if (!data) {
    return { answer: "Unable to load your Ministry for Women information.", enabled: false };
  }
  const context = produceWomenAiContext(data);
  return askWithContext(question, context);
}
