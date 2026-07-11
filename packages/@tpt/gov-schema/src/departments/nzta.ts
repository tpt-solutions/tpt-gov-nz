import { z } from "zod";

export const NztaDriverLicenceSchema = z.object({
  licenceNumber: z.string(),
  fullName: z.string(),
  licenceClass: z.string(),
  expiryDate: z.string(),
  conditions: z.string().optional(),
});

export const NztaVehicleSchema = z.object({
  registration: z.string(),
  make: z.string(),
  model: z.string(),
  year: z.number(),
  fuelType: z.string(),
  registrationExpiry: z.string(),
});

export const NztaRucSchema = z.object({
  vehicleRego: z.string(),
  licenceType: z.string(),
  expiryDate: z.string(),
  unitsRemaining: z.number(),
});

export const NZTADataBundleSchema = z.object({
  driverLicenceNumber: z.string(),
  driverLicence: NztaDriverLicenceSchema.optional(),
  vehicles: z.array(NztaVehicleSchema).optional(),
  ruc: z.array(NztaRucSchema).optional(),
});

export type NZTADataBundle = z.infer<typeof NZTADataBundleSchema>;

export const NztaActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("renew-vehicle-registration"),
    registration: z.string(),
    months: z.number().int().min(1).max(24),
  }),
  z.object({
    type: z.literal("request-licence-replacement"),
    reason: z.string().min(1),
  }),
]);

export type NztaAction = z.infer<typeof NztaActionSchema>;
