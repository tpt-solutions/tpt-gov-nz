"use server";

import type { CORRECTIONSDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { getCitizenDid } from "../../lib/session";
import { produceCorrectionsAiContext } from "./corrections-ai";

const CORRECTIONS_SERVICE_URL = process.env.CORRECTIONS_SERVICE_URL ?? "http://localhost:8104";

export async function fetchCorrectionsData(scopes: string[]): Promise<CORRECTIONSDataBundle | null> {
  const did = await getCitizenDid();
  if (!did) return null;

  try {
    const res = await fetch(`${CORRECTIONS_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes }),
      next: { revalidate: 60 },
    });
    if (!res.ok) return null;
    return res.json() as Promise<CORRECTIONSDataBundle>;
  } catch {
    return null;
  }
}

export async function submitCorrectionsAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${CORRECTIONS_SERVICE_URL}/citizen/action`, {
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

// ── AI: "what is the status of my probation?" (Stage 6) ───────────────────

export async function askCorrections(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchCorrectionsData(["corrections:probation", "corrections:case"]);
  if (!data) {
    return { answer: "Unable to load your Corrections information.", enabled: false };
  }
  const context = produceCorrectionsAiContext(data);
  return askWithContext(question, context);
}
