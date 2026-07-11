"use server";

import type { NZQADataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceNzqaAiContext } from "./nzqa-ai";

const NZQA_SERVICE_URL = process.env.NZQA_SERVICE_URL ?? "http://localhost:8099";

export async function fetchNzqaData(scopes: string[]): Promise<NZQADataBundle | null> {
  return (await fetchDeptData("nzqa", scopes)) as NZQADataBundle | null;
}

export async function submitNzqaAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${NZQA_SERVICE_URL}/citizen/action`, {
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

// ── AI: "what qualifications do I have recorded?" (Stage 6) ─────────────

export async function askNzqa(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchNzqaData(["nzqa:qualifications", "nzqa:transcripts"]);
  if (!data) {
    return { answer: "Unable to load your NZQA information.", enabled: false };
  }
  const context = produceNzqaAiContext(data);
  return askWithContext(question, context);
}
