"use server";

import type { IRDDataBundle } from "@tpt/gov-schema";
import { produceIrdAiContext } from "./ai-context";
import { askWithContext } from "../../ai/client";
import { fetchDeptData } from "../../lib/data-access";
import { getCitizenDid } from "../../lib/session";
import { assessWffEligibility, recommendKiwiSaverRate } from "./helpers";
import type { WffEligibilityInput, WffEligibilityResult, KiwiSaverRecommendation } from "./helpers";

const IRD_SERVICE_URL = process.env.IRD_SERVICE_URL ?? "http://localhost:8090";

export async function fetchIrdData(scopes: string[]): Promise<IRDDataBundle | null> {
  return (await fetchDeptData("ird", scopes)) as IRDDataBundle | null;
}

export async function submitIrdAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  if (process.env.NEXT_PUBLIC_DEMO_MODE === "true") {
    return { success: true, message: "Demo mode: action recorded locally (no live service)." };
  }

  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${IRD_SERVICE_URL}/citizen/action`, {
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

// ── Life-event: "I just had a baby" → WFF eligibility ──────────────────────────

export async function checkWffEligibility(
  input: WffEligibilityInput,
): Promise<WffEligibilityResult> {
  const did = await getCitizenDid();
  if (!did) {
    return {
      eligible: false,
      dependantChildren: input.dependantChildren,
      familyIncome: input.familyIncome,
      incomeThreshold: null,
      headroom: null,
      note: "Not authenticated.",
    };
  }

  const data = await fetchIrdData(["ird:wff"]);
  const threshold = data?.workingForFamilies?.incomeThreshold
    ? Number(data.workingForFamilies.incomeThreshold)
    : null;

  return assessWffEligibility(input, threshold);
}

// ── AI entitlement prompt: "Am I eligible for WFF?" ────────────────────────────

export async function askWffEligibility(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchIrdData(["ird:wff", "ird:income", "ird:kiwisaver"]);
  if (!data) {
    return { answer: "Unable to load your IRD information.", enabled: false };
  }
  const context = produceIrdAiContext(data);
  return askWithContext(question, context);
}

// ── AI action suggestion (level ≥ assisted): KiwiSaver rate ────────────────────

export async function recommendKiwiSaverRateAction(): Promise<KiwiSaverRecommendation> {
  const data = await fetchIrdData(["ird:kiwisaver"]);
  const ks = data?.kiwiSaver;
  const currentRate = ks?.contributionRate != null ? Number(ks.contributionRate) : null;
  const totalBalance = ks?.totalBalance != null ? Number(ks.totalBalance) : null;
  return recommendKiwiSaverRate(currentRate, totalBalance);
}
