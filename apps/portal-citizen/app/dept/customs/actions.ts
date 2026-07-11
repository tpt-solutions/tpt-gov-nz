"use server";

import type { CUSTOMSDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceCustomsAiContext } from "./customs-ai";

const CUSTOMS_SERVICE_URL = process.env.CUSTOMS_SERVICE_URL ?? "http://localhost:8105";

export async function fetchCustomsData(scopes: string[]): Promise<CUSTOMSDataBundle | null> {
  return (await fetchDeptData("customs", scopes)) as CUSTOMSDataBundle | null;
}

export async function submitCustomsAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${CUSTOMS_SERVICE_URL}/citizen/action`, {
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

// ── AI: "what declarations have I submitted?" (Stage 6) ─────────────

export async function askCustoms(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchCustomsData(["customs:travel", "customs:declarations"]);
  if (!data) {
    return { answer: "Unable to load your customs information.", enabled: false };
  }
  const context = produceCustomsAiContext(data);
  return askWithContext(question, context);
}
