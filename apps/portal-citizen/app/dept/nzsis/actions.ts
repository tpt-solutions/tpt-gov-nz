"use server";

import type { NzsisDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceNzsisAiContext } from "./nzsis-ai";

const NZSIS_SERVICE_URL = process.env.NZSIS_SERVICE_URL ?? "http://localhost:8147";

export async function fetchNzsisData(scopes: string[]): Promise<NzsisDataBundle | null> {
  return (await fetchDeptData("nzsis", scopes)) as NzsisDataBundle | null;
}

export async function submitNzsisAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(NZSIS_SERVICE_URL + "/citizen/action", {
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

export async function askNzsis(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchNzsisData(["nzsis:mandates", "nzsis:threats"]);
  if (!data) {
    return { answer: "Unable to load your New Zealand Security Intelligence Service information.", enabled: false };
  }
  const context = produceNzsisAiContext(data);
  return askWithContext(question, context);
}
