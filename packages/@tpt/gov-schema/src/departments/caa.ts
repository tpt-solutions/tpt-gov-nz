import { z } from "zod";

export const CaaLicencesSchema = z.object({
  licenceNo: z.string(),
  category: z.string(),
  status: z.string(),
  expires: z.string(),
});

export const CaaAircraftSchema = z.object({
  registration: z.string(),
  aircraftType: z.string(),
  status: z.string(),
});

export const CaaDataBundleSchema = z.object({
  caaId: z.string(),
  licences: z.array(CaaLicencesSchema).optional(),
  aircraft: z.array(CaaAircraftSchema).optional(),
});

export type CaaDataBundle = z.infer<typeof CaaDataBundleSchema>;

export const CaaActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-licence-replacement"),
    licenceNo: z.string().min(1),
  }),
]);

export type CaaAction = z.infer<typeof CaaActionSchema>;
