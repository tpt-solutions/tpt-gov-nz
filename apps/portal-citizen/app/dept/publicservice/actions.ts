"use server";

import type { PublicserviceDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { producePublicserviceAiContext } from "./publicservice-ai";

const PUBLICSERVICE_SERVICE_URL = process.env.PUBLICSERVICE_SERVICE_URL ?? "http://localhost:8122";

export async function fetchPublicserviceData(scopes: string[]): Promise<PublicserviceDataBundle | null> {
  return (await fetchDeptData("publicservice", scopes)) as PublicserviceDataBundle | null;
}

export async function submitPublicserviceAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(PUBLICSERVICE_SERVICE_URL + "/citizen/action", {
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

export async function askPublicservice(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchPublicserviceData(["publicservice:workforce", "publicservice:agency-ratings"]);
  if (!data) {
    return { answer: "Unable to load your Te Kawa Mataaho Public Service Commission information.", enabled: false };
  }
  const context = producePublicserviceAiContext(data);
  return askWithContext(question, context);
}
