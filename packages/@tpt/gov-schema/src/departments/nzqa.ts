import { z } from "zod";

export const NzqaQualificationSchema = z.object({
  title: z.string(),
  level: z.number(),
  awardedDate: z.string(),
  provider: z.string(),
});

export const NzqaTranscriptSchema = z.object({
  recordSummary: z.string().optional(),
  totalCredits: z.number().optional(),
  creditSummary: z.string().optional(),
});

export const NZQADataBundleSchema = z.object({
  nsn: z.string(),
  qualifications: z.array(NzqaQualificationSchema).optional(),
  transcript: NzqaTranscriptSchema.optional(),
});

export type NZQADataBundle = z.infer<typeof NZQADataBundleSchema>;

export const NzqaActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-transcript"),
    purpose: z.string().min(1),
  }),
  z.object({
    type: z.literal("order-convocation"),
    qualificationId: z.string().min(1),
  }),
]);

export type NzqaAction = z.infer<typeof NzqaActionSchema>;
