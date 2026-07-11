"use server";

import type { HUDDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { produceHudAiContext } from "./hud-ai";

const HUD_SERVICE_URL = process.env.HUD_SERVICE_URL ?? "http://localhost:8098";

export async function fetchHudData(scopes: string[]): Promise<HUDDataBundle | null> {
  return (await fetchDeptData("hud", scopes)) as HUDDataBundle | null;
}

export async function submitHudAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${HUD_SERVICE_URL}/citizen/action`, {
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

// ── AI: "what's the status of my housing application?" (Stage 6) ─────────────

export async function askHud(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchHudData(["hud:applications", "hud:tenancy", "hud:maintenance"]);
  if (!data) {
    return { answer: "Unable to load your housing information.", enabled: false };
  }
  const context = produceHudAiContext(data);
  return askWithContext(question, context);
}
