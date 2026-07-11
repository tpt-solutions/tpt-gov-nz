import { z } from "zod";

export const WomenProgrammesSchema = z.object({
  programmeName: z.string(),
  status: z.string(),
  year: z.number(),
});

export const WomenInsightsSchema = z.object({
  topic: z.string(),
  summary: z.string(),
  published: z.string(),
});

export const WomenDataBundleSchema = z.object({
  womenId: z.string(),
  programmes: z.array(WomenProgrammesSchema).optional(),
  insights: z.array(WomenInsightsSchema).optional(),
});

export type WomenDataBundle = z.infer<typeof WomenDataBundleSchema>;

export const WomenActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-programme-info"),
    programmeName: z.string().min(1),
  }),
]);

export type WomenAction = z.infer<typeof WomenActionSchema>;
