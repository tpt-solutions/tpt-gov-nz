"use server";

import type { WINZDataBundle } from "@tpt/gov-schema";
import { askWithContext } from "../../ai/client";

const WINZ_SERVICE_URL = process.env.WINZ_SERVICE_URL ?? "http://localhost:8091";

async function getCitizenDid(): Promise<string | null> {
  // In Phase 1 this reads from the session cookie / JWT.
  // Stubbed for now — full implementation when identity server is wired up.
  return process.env.DEMO_CITIZEN_DID ?? "did:gov:nz:test-citizen-001";
}

export async function fetchWinzData(scopes: string[]): Promise<WINZDataBundle | null> {
  const did = await getCitizenDid();
  if (!did) return null;

  try {
    const res = await fetch(`${WINZ_SERVICE_URL}/citizen/data`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ did, scopes }),
      next: { revalidate: 60 },
    });

    if (!res.ok) return null;
    return res.json() as Promise<WINZDataBundle>;
  } catch {
    return null;
  }
}

export async function submitWinzAction(
  type: string,
  parameters: Record<string, unknown>
): Promise<{ success: boolean; message?: string }> {
  const did = await getCitizenDid();
  if (!did) return { success: false, message: "Not authenticated" };

  try {
    const res = await fetch(`${WINZ_SERVICE_URL}/citizen/action`, {
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

// ── AI benefit calculator (Stage 6) ────────────────────────────────────────────

export interface EntitlementInput {
  benefitType: string;
  weeklyAmount: number;
  partnerIncome: number;
  dependantChildren: number;
}

export interface EntitlementResult {
  benefitType: string;
  currentWeekly: number;
  recommendedWeekly: number;
  delta: number;
  note: string;
}

/**
 * Approximate entitlement adjustment for a sole-parent / jobseeker-style benefit.
 * The more dependant children and the lower the partner income, the higher the
 * recommended weekly amount. This is a transparent heuristic used to *suggest*
 * a figure to the citizen and the case worker — it is not an official calculation.
 */
export function calculateEstimatedEntitlement(input: EntitlementInput): EntitlementResult {
  const { benefitType, weeklyAmount, partnerIncome, dependantChildren } = input;

  let recommended = weeklyAmount;
  if (benefitType === "sole-parent" || benefitType === "jobseeker") {
    const childSupplement = dependantChildren * 30; // placeholder per-child amount
    const incomeAdjustment = partnerIncome < 20000 ? 60 : partnerIncome < 40000 ? 20 : 0;
    recommended = weeklyAmount + childSupplement + incomeAdjustment;
  }

  const delta = Math.round((recommended - weeklyAmount) * 100) / 100;

  const note =
    delta > 0
      ? `Based on ${dependantChildren} dependent children and a partner income of $${partnerIncome.toLocaleString()}, you may be entitled to about $${delta.toFixed(2)} more per week.`
      : `Your current entitlement of $${weeklyAmount.toFixed(2)} per week appears appropriate for your circumstances.`;

  return {
    benefitType,
    currentWeekly: weeklyAmount,
    recommendedWeekly: recommended,
    delta,
    note,
  };
}

export async function askWinzEntitlement(question: string): Promise<{ answer: string; enabled: boolean }> {
  const data = await fetchWinzData(["winz:benefits", "winz:payments"]);
  if (!data) {
    return { answer: "Unable to load your Work and Income information.", enabled: false };
  }
  const context = produceWinzAiContext(data);
  return askWithContext(question, context);
}

/** Build the AI context chunks from a WINZ data bundle. */
export function produceWinzAiContext(data: WINZDataBundle): { kind: string; text: string; pii: boolean }[] {
  const chunks: { kind: string; text: string; pii: boolean }[] = [];

  const total = Number(data.totalWeeklyPayment);
  chunks.push({
    kind: "winz_summary",
    text: `Client ${data.clientId} receives a total of $${total.toFixed(2)} per week across ${data.activeBenefits.length} active benefit(s).`,
    pii: false,
  });

  for (const b of data.activeBenefits) {
    chunks.push({
      kind: "winz_benefit",
      text: `Benefit ${b.type}: $${Number(b.weeklyAmount).toFixed(2)}/week, status ${b.status}${b.reviewDate ? `, review due ${b.reviewDate}` : ""}.`,
      pii: false,
    });
  }

  if (data.payments && data.payments.length > 0) {
    const last = data.payments[0];
    chunks.push({
      kind: "winz_payment",
      text: `Most recent payment: ${last.benefitType} $${Number(last.amount).toFixed(2)} on ${last.paymentDate} via ${last.method}.`,
      pii: false,
    });
  }

  return chunks;
}
