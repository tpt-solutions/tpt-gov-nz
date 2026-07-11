"use server";

import type { EthnicDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceEthnicAiContext } from "./ethnic-ai";

const ETHNIC_SERVICE_URL = process.env.ETHNIC_SERVICE_URL ?? "http://localhost:8128";

export async function fetchEthnicData(scopes: string[]): Promise<EthnicDataBundle | null> {
  return (await fetchDeptData("ethnic", scopes)) as EthnicDataBundle | null;
}

export async function submitEthnicAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(ETHNIC_SERVICE_URL + "/citizen/action", {
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

export async function askEthnic(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchEthnicData(["ethnic:programmes", "ethnic:community-grants"]);
  if (!data) {
    return { answer: "Unable to load your Ministry for Ethnic Communities information.", enabled: false };
  }
  const context = produceEthnicAiContext(data);
  return askWithContext(question, context);
}
