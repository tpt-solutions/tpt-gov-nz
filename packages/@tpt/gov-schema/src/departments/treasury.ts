import { z } from "zod";

export const TreasuryBudgetSchema = z.object({
  fiscalYear: z.number(),
  portfolio: z.string(),
  appropriation: z.string(),
  amount: z.number(),
});

export const TreasuryEconomicOutlookSchema = z.object({
  forecastYear: z.number(),
  gdpGrowthPct: z.number(),
  inflationPct: z.number(),
  netDebtPct: z.number(),
});

export const TreasuryDataBundleSchema = z.object({
  treasuryId: z.string(),
  budget: z.array(TreasuryBudgetSchema).optional(),
  economic_outlook: TreasuryEconomicOutlookSchema.optional(),
});

export type TreasuryDataBundle = z.infer<typeof TreasuryDataBundleSchema>;

export const TreasuryActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-economic-brief"),
    topic: z.string().min(1),
  }),
]);

export type TreasuryAction = z.infer<typeof TreasuryActionSchema>;
