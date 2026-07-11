"use server";

import type { CaaDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceCaaAiContext } from "./caa-ai";

const CAA_SERVICE_URL = process.env.CAA_SERVICE_URL ?? "http://localhost:8136";

export async function fetchCaaData(scopes: string[]): Promise<CaaDataBundle | null> {
  return (await fetchDeptData("caa", scopes)) as CaaDataBundle | null;
}

export async function submitCaaAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(CAA_SERVICE_URL + "/citizen/action", {
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

export async function askCaa(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchCaaData(["caa:licences", "caa:aircraft"]);
  if (!data) {
    return { answer: "Unable to load your Civil Aviation Authority information.", enabled: false };
  }
  const context = produceCaaAiContext(data);
  return askWithContext(question, context);
}
