"use server";

import type { IRDDataBundle } from "@tpt/gov-schema";
import { IRDAdapter } from "@tpt/adapter-ird";
import { askWithContext } from "../../ai/client";

const IRD_SERVICE_URL = process.env.IRD_SERVICE_URL ?? "http://localhost:8090";

async function getCitizenDid(): Promise<string | null> {
  // In Phase 1 this reads from the session cookie / JWT
  // Stubbed for now — full implementation when identity server is wired up
  return process.env.DEMO_CITIZEN_DID ?? "did:gov:nz:test-citizen-001";
}

export async function fetchIrdData(scopes: string[]): Promise<IRDDataBundle | null> {
  const did = await getCitizenDid();
  if (!did) return null;

  try {
    const res = await fetch(`${IRD_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<IRDDataBundle>;
  } catch {
    return null;
  }
}

export async function submitIrdAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
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

export interface WffEligibilityInput {
  dependantChildren: number;
  familyIncome: number;
}

export interface WffEligibilityResult {
  eligible: boolean;
  dependantChildren: number;
  familyIncome: number;
  incomeThreshold: number | null;
  headroom: number | null;
  note: string;
}

/**
 * Pure WFF eligibility assessment. Eligibility requires at least one dependent
 * child and family income at or below the IRD income threshold for the family's
 * circumstances. Returns `headroom` (how much income sits below the threshold).
 */
export function assessWffEligibility(
  input: WffEligibilityInput,
  incomeThreshold: number | null,
): WffEligibilityResult {
  if (incomeThreshold == null) {
    return {
      eligible: false,
      dependantChildren: input.dependantChildren,
      familyIncome: input.familyIncome,
      incomeThreshold: null,
      headroom: null,
      note: "Your income threshold is not yet known. Complete your IRD assessment to check eligibility.",
    };
  }

  const headroom = incomeThreshold - input.familyIncome;
  const eligible = input.dependantChildren >= 1 && input.familyIncome <= incomeThreshold;
  const note = eligible
    ? `Based on ${input.dependantChildren} dependent children and a family income of $${input.familyIncome.toLocaleString()}, you appear eligible for Working for Families.`
    : input.dependantChildren < 1
      ? "Working for Families requires at least one dependent child."
      : `Your family income is $${Math.abs(headroom).toLocaleString()} above the income threshold of $${incomeThreshold.toLocaleString()}.`;

  return {
    eligible,
    dependantChildren: input.dependantChildren,
    familyIncome: input.familyIncome,
    incomeThreshold,
    headroom,
    note,
  };
}

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
  const adapter = new IRDAdapter(IRD_SERVICE_URL, "citizen");
  const context = adapter.produceAiContext(data);
  return askWithContext(question, context);
}

// ── AI action suggestion (level ≥ assisted): KiwiSaver rate ────────────────────

export interface KiwiSaverRecommendation {
  suggestedRate: number;
  reason: string;
  currentRate: number | null;
}

const ALLOWED_RATES = [3, 4, 6, 8, 10];

/**
 * Heuristic KiwiSaver contribution-rate suggestion. A larger estimated balance
 * and active membership justify a lower rate; a small balance justifies a higher
 * rate to build retirement savings faster. Only returns an allowed rate.
 */
export function recommendKiwiSaverRate(
  currentRate: number | null,
  totalBalance: number | null,
): KiwiSaverRecommendation {
  const balance = totalBalance ?? 0;
  let suggested: number;
  if (balance < 10_000) suggested = 6;
  else if (balance < 50_000) suggested = 4;
  else suggested = 3;

  if (!ALLOWED_RATES.includes(suggested)) suggested = 4;

  const reason =
    balance < 10_000
      ? "Your balance is modest, so a higher contribution rate (6%) helps build retirement savings faster while you are young."
      : balance < 50_000
        ? "A balanced 4% rate grows your savings steadily without reducing take-home pay too much."
        : "Your balance is healthy, so the minimum 3% rate is sufficient unless you want to save more aggressively.";

  return { suggestedRate: suggested, reason, currentRate };
}

export async function recommendKiwiSaverRateAction(): Promise<KiwiSaverRecommendation> {
  const data = await fetchIrdData(["ird:kiwisaver"]);
  const ks = data?.kiwiSaver;
  const currentRate = ks?.contributionRate != null ? Number(ks.contributionRate) : null;
  const totalBalance = ks?.totalBalance != null ? Number(ks.totalBalance) : null;
  return recommendKiwiSaverRate(currentRate, totalBalance);
}
