"use server";

import type { CrownlawDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceCrownlawAiContext } from "./crownlaw-ai";

const CROWNLAW_SERVICE_URL = process.env.CROWNLAW_SERVICE_URL ?? "http://localhost:8123";

export async function fetchCrownlawData(scopes: string[]): Promise<CrownlawDataBundle | null> {
  return (await fetchDeptData("crownlaw", scopes)) as CrownlawDataBundle | null;
}

export async function submitCrownlawAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(CROWNLAW_SERVICE_URL + "/citizen/action", {
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

export async function askCrownlaw(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchCrownlawData(["crownlaw:legal-opinions", "crownlaw:litigation"]);
  if (!data) {
    return { answer: "Unable to load your Crown Law Office information.", enabled: false };
  }
  const context = produceCrownlawAiContext(data);
  return askWithContext(question, context);
}
