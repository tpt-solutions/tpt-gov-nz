import { z } from "zod";

export const SfoInvestigationsSchema = z.object({
  reference: z.string(),
  matter: z.string(),
  status: z.string(),
  openedDate: z.string(),
});

export const SfoOutcomesSchema = z.object({
  reference: z.string(),
  result: z.string(),
  resultDate: z.string(),
});

export const SfoDataBundleSchema = z.object({
  sfoId: z.string(),
  investigations: z.array(SfoInvestigationsSchema).optional(),
  outcomes: z.array(SfoOutcomesSchema).optional(),
});

export type SfoDataBundle = z.infer<typeof SfoDataBundleSchema>;

export const SfoActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-investigation-update"),
    reference: z.string().min(1),
  }),
]);

export type SfoAction = z.infer<typeof SfoActionSchema>;
