"use server";

import type { GcsbDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceGcsbAiContext } from "./gcsb-ai";

const GCSB_SERVICE_URL = process.env.GCSB_SERVICE_URL ?? "http://localhost:8146";

export async function fetchGcsbData(scopes: string[]): Promise<GcsbDataBundle | null> {
  return (await fetchDeptData("gcsb", scopes)) as GcsbDataBundle | null;
}

export async function submitGcsbAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(GCSB_SERVICE_URL + "/citizen/action", {
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

export async function askGcsb(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchGcsbData(["gcsb:mandates", "gcsb:engagements"]);
  if (!data) {
    return { answer: "Unable to load your Government Communications Security Bureau information.", enabled: false };
  }
  const context = produceGcsbAiContext(data);
  return askWithContext(question, context);
}
