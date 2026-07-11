import { z } from "zod";

export const MfatOverseasMissionsSchema = z.object({
  country: z.string(),
  status: z.string(),
});

export const MfatTravelAdvisoriesSchema = z.object({
  country: z.string(),
  level: z.string(),
  updated: z.string(),
});

export const MfatDataBundleSchema = z.object({
  mfatId: z.string(),
  overseas_missions: z.array(MfatOverseasMissionsSchema).optional(),
  travel_advisories: z.array(MfatTravelAdvisoriesSchema).optional(),
});

export type MfatDataBundle = z.infer<typeof MfatDataBundleSchema>;

export const MfatActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-advisory-update"),
    country: z.string().min(1),
  }),
]);

export type MfatAction = z.infer<typeof MfatActionSchema>;
