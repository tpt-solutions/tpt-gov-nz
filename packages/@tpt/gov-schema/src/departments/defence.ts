import { z } from "zod";

export const DefenceProcurementsSchema = z.object({
  programme: z.string(),
  value: z.number(),
  status: z.string(),
});

export const DefenceBasesSchema = z.object({
  name: z.string(),
  location: z.string(),
  status: z.string(),
});

export const DefenceDataBundleSchema = z.object({
  defenceId: z.string(),
  procurements: z.array(DefenceProcurementsSchema).optional(),
  bases: z.array(DefenceBasesSchema).optional(),
});

export type DefenceDataBundle = z.infer<typeof DefenceDataBundleSchema>;

export const DefenceActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-procurement-info"),
    programme: z.string().min(1),
  }),
]);

export type DefenceAction = z.infer<typeof DefenceActionSchema>;
