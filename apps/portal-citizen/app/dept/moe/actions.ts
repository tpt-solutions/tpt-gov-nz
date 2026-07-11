"use server";

import type { MoeDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceMoeAiContext } from "./moe-ai";

const MOE_SERVICE_URL = process.env.MOE_SERVICE_URL ?? "http://localhost:8139";

export async function fetchMoeData(scopes: string[]): Promise<MoeDataBundle | null> {
  return (await fetchDeptData("moe", scopes)) as MoeDataBundle | null;
}

export async function submitMoeAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(MOE_SERVICE_URL + "/citizen/action", {
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

export async function askMoe(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchMoeData(["moe:enrolment", "moe:student-support"]);
  if (!data) {
    return { answer: "Unable to load your Ministry of Education information.", enabled: false };
  }
  const context = produceMoeAiContext(data);
  return askWithContext(question, context);
}
