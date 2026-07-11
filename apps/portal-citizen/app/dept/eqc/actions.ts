"use server";

import type { EqcDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceEqcAiContext } from "./eqc-ai";

const EQC_SERVICE_URL = process.env.EQC_SERVICE_URL ?? "http://localhost:8134";

export async function fetchEqcData(scopes: string[]): Promise<EqcDataBundle | null> {
  return (await fetchDeptData("eqc", scopes)) as EqcDataBundle | null;
}

export async function submitEqcAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(EQC_SERVICE_URL + "/citizen/action", {
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

export async function askEqc(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchEqcData(["eqc:claims", "eqc:cover"]);
  if (!data) {
    return { answer: "Unable to load your Earthquake Commission (Toka Tū Ake) information.", enabled: false };
  }
  const context = produceEqcAiContext(data);
  return askWithContext(question, context);
}
