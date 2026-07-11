import { z } from "zod";

export const CrownlawLegalOpinionsSchema = z.object({
  reference: z.string(),
  topic: z.string(),
  issuedDate: z.string(),
  status: z.string(),
});

export const CrownlawLitigationSchema = z.object({
  caseName: z.string(),
  crownRole: z.string(),
  status: z.string(),
});

export const CrownlawDataBundleSchema = z.object({
  crownlawId: z.string(),
  legal_opinions: z.array(CrownlawLegalOpinionsSchema).optional(),
  litigation: z.array(CrownlawLitigationSchema).optional(),
});

export type CrownlawDataBundle = z.infer<typeof CrownlawDataBundleSchema>;

export const CrownlawActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-opinion-copy"),
    reference: z.string().min(1),
  }),
]);

export type CrownlawAction = z.infer<typeof CrownlawActionSchema>;
