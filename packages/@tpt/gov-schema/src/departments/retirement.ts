import { z } from "zod";

export const RetirementRetirementPlanSchema = z.object({
  hasPlan: z.boolean(),
  retirementAge: z.number(),
  lastReview: z.string(),
});

export const RetirementGuidanceSchema = z.object({
  topic: z.string(),
  summary: z.string(),
  published: z.string(),
});

export const RetirementDataBundleSchema = z.object({
  retirementId: z.string(),
  retirement_plan: RetirementRetirementPlanSchema.optional(),
  guidance: z.array(RetirementGuidanceSchema).optional(),
});

export type RetirementDataBundle = z.infer<typeof RetirementDataBundleSchema>;

export const RetirementActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-guidance"),
    topic: z.string().min(1),
  }),
]);

export type RetirementAction = z.infer<typeof RetirementActionSchema>;
