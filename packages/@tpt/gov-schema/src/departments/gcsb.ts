import { z } from "zod";

export const GcsbMandatesSchema = z.object({
  reference: z.string(),
  agency: z.string(),
  status: z.string(),
  issuedDate: z.string(),
});

export const GcsbEngagementsSchema = z.object({
  partner: z.string(),
  engagementType: z.string(),
  engagementDate: z.string(),
});

export const GcsbDataBundleSchema = z.object({
  gcsbId: z.string(),
  mandates: z.array(GcsbMandatesSchema).optional(),
  engagements: z.array(GcsbEngagementsSchema).optional(),
});

export type GcsbDataBundle = z.infer<typeof GcsbDataBundleSchema>;

export const GcsbActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-mandate-info"),
    reference: z.string().min(1),
  }),
]);

export type GcsbAction = z.infer<typeof GcsbActionSchema>;
