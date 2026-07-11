import type { WINZDataBundle, AiContextChunk } from "@tpt/gov-schema";

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
    const childSupplement = dependantChildren * 30;
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

/** Build the AI context chunks from a WINZ data bundle. */
export function produceWinzAiContext(data: WINZDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  const total = Number(data.totalWeeklyPayment);
  chunks.push({
    deptId: "winz",
    content: `Client ${data.clientId} receives a total of $${total.toFixed(2)} per week across ${data.activeBenefits.length} active benefit(s).`,
    metadata: { area: "summary" },
  });

  for (const b of data.activeBenefits) {
    chunks.push({
      deptId: "winz",
      content: `Benefit ${b.type}: $${Number(b.weeklyAmount).toFixed(2)}/week, status ${b.status}${b.reviewDate ? `, review due ${b.reviewDate}` : ""}.`,
      metadata: { area: "benefit" },
    });
  }

  if (data.payments && data.payments.length > 0) {
    const last = data.payments[0];
    chunks.push({
      deptId: "winz",
      content: `Most recent payment: ${last.benefitType} $${Number(last.amount).toFixed(2)} on ${last.paymentDate} via ${last.method}.`,
      metadata: { area: "payment" },
    });
  }

  return chunks;
}
