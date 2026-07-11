"use server";

import type { MfeDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceMfeAiContext } from "./mfe-ai";

const MFE_SERVICE_URL = process.env.MFE_SERVICE_URL ?? "http://localhost:8133";

export async function fetchMfeData(scopes: string[]): Promise<MfeDataBundle | null> {
  return (await fetchDeptData("mfe", scopes)) as MfeDataBundle | null;
}

export async function submitMfeAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(MFE_SERVICE_URL + "/citizen/action", {
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

export async function askMfe(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchMfeData(["mfe:emissions", "mfe:reports"]);
  if (!data) {
    return { answer: "Unable to load your Ministry for the Environment information.", enabled: false };
  }
  const context = produceMfeAiContext(data);
  return askWithContext(question, context);
}
