"use server";

import type { FenzDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceFenzAiContext } from "./fenz-ai";

const FENZ_SERVICE_URL = process.env.FENZ_SERVICE_URL ?? "http://localhost:8138";

export async function fetchFenzData(scopes: string[]): Promise<FenzDataBundle | null> {
  return (await fetchDeptData("fenz", scopes)) as FenzDataBundle | null;
}

export async function submitFenzAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(FENZ_SERVICE_URL + "/citizen/action", {
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

export async function askFenz(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchFenzData(["fenz:fire-safety", "fenz:incidents"]);
  if (!data) {
    return { answer: "Unable to load your Fire and Emergency New Zealand information.", enabled: false };
  }
  const context = produceFenzAiContext(data);
  return askWithContext(question, context);
}
