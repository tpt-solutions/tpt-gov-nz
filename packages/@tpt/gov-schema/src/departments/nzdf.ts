import { z } from "zod";

export const NzdfServiceRecordsSchema = z.object({
  serviceNo: z.string(),
  branch: z.string(),
  status: z.string(),
});

export const NzdfDeploymentsSchema = z.object({
  operation: z.string(),
  country: z.string(),
  year: z.number(),
});

export const NzdfDataBundleSchema = z.object({
  nzdfId: z.string(),
  service_records: z.array(NzdfServiceRecordsSchema).optional(),
  deployments: z.array(NzdfDeploymentsSchema).optional(),
});

export type NzdfDataBundle = z.infer<typeof NzdfDataBundleSchema>;

export const NzdfActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-service-record"),
    serviceNo: z.string().min(1),
  }),
]);

export type NzdfAction = z.infer<typeof NzdfActionSchema>;
