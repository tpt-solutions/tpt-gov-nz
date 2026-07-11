import type { IRDDataBundle } from "@tpt/gov-schema";

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
