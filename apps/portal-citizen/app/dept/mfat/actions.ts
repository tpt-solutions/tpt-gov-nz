"use server";

import type { MfatDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceMfatAiContext } from "./mfat-ai";

const MFAT_SERVICE_URL = process.env.MFAT_SERVICE_URL ?? "http://localhost:8143";

export async function fetchMfatData(scopes: string[]): Promise<MfatDataBundle | null> {
  return (await fetchDeptData("mfat", scopes)) as MfatDataBundle | null;
}

export async function submitMfatAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(MFAT_SERVICE_URL + "/citizen/action", {
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

export async function askMfat(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchMfatData(["mfat:overseas-missions", "mfat:travel-advisories"]);
  if (!data) {
    return { answer: "Unable to load your Ministry of Foreign Affairs and Trade information.", enabled: false };
  }
  const context = produceMfatAiContext(data);
  return askWithContext(question, context);
}
