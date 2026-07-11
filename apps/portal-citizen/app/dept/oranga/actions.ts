"use server";

import type { OrangaDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceOrangaAiContext } from "./oranga-ai";

const ORANGA_SERVICE_URL = process.env.ORANGA_SERVICE_URL ?? "http://localhost:8125";

export async function fetchOrangaData(scopes: string[]): Promise<OrangaDataBundle | null> {
  return (await fetchDeptData("oranga", scopes)) as OrangaDataBundle | null;
}

export async function submitOrangaAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(ORANGA_SERVICE_URL + "/citizen/action", {
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

export async function askOranga(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchOrangaData(["oranga:care-placements", "oranga:support-services"]);
  if (!data) {
    return { answer: "Unable to load your Oranga Tamariki information.", enabled: false };
  }
  const context = produceOrangaAiContext(data);
  return askWithContext(question, context);
}
