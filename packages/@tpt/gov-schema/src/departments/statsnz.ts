import { z } from "zod";

export const StatsnzCensusSchema = z.object({
  censusYear: z.number(),
  dwellingType: z.string(),
  householdSize: z.number(),
  region: z.string(),
});

export const StatsnzProfileSchema = z.object({
  dataSummary: z.string(),
  recordCount: z.number(),
  lastUpdated: z.string(),
});

export const STATSNZDataBundleSchema = z.object({
  statsId: z.string(),
  census: z.array(StatsnzCensusSchema).optional(),
  profile: StatsnzProfileSchema.optional(),
});

export type StatsNZDataBundle = z.infer<typeof STATSNZDataBundleSchema>;

export const StatsnzActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-data-export"),
    purpose: z.string().min(1),
  }),
]);

export type StatsnzAction = z.infer<typeof StatsnzActionSchema>;
