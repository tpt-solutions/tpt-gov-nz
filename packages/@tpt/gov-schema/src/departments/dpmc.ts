import { z } from "zod";

export const DpmcHonoursSchema = z.object({
  awardYear: z.number(),
  award: z.string(),
  status: z.string(),
});

export const DpmcEngagementsSchema = z.object({
  eventName: z.string(),
  eventDate: z.string(),
  location: z.string(),
});

export const DpmcDataBundleSchema = z.object({
  dpmcId: z.string(),
  honours: z.array(DpmcHonoursSchema).optional(),
  engagements: z.array(DpmcEngagementsSchema).optional(),
});

export type DpmcDataBundle = z.infer<typeof DpmcDataBundleSchema>;

export const DpmcActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-honours-update"),
    topic: z.string().min(1),
  }),
]);

export type DpmcAction = z.infer<typeof DpmcActionSchema>;
