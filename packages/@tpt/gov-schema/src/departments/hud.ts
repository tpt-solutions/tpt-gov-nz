import { z } from "zod";

export const HudApplicationSchema = z.object({
  applicationNumber: z.string(),
  applicationType: z.enum(["public-housing", "emergency-housing", "home-ownership"]),
  status: z.enum(["submitted", "assessed", "approved", "declined", "waitlisted"]),
  priorityBand: z.enum(["A", "B", "C", "D"]).optional(),
  bedroomsNeeded: z.number().optional(),
  submittedDate: z.string(),
});

export const HudTenancySchema = z.object({
  tenancyId: z.string(),
  propertyAddress: z.string(),
  weeklyRent: z.number(),
  incomeRelatedRent: z.boolean(),
  startDate: z.string(),
  status: z.enum(["active", "ended"]),
});

export const HudMaintenanceRequestSchema = z.object({
  requestNumber: z.string(),
  category: z.enum(["plumbing", "electrical", "heating", "structural", "other"]),
  status: z.enum(["submitted", "scheduled", "completed"]),
  description: z.string(),
  requestedDate: z.string(),
});

export const HUDDataBundleSchema = z.object({
  clientNumber: z.string(),
  applications: z.array(HudApplicationSchema).optional(),
  tenancies: z.array(HudTenancySchema).optional(),
  maintenanceRequests: z.array(HudMaintenanceRequestSchema).optional(),
});

export type HUDDataBundle = z.infer<typeof HUDDataBundleSchema>;

export const HudActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("submit-housing-application"),
    applicationType: z.enum(["public-housing", "emergency-housing", "home-ownership"]),
    bedroomsNeeded: z.number().optional(),
    reason: z.string().min(1),
  }),
  z.object({
    type: z.literal("request-maintenance"),
    category: z.enum(["plumbing", "electrical", "heating", "structural", "other"]),
    description: z.string().min(1),
  }),
]);

export type HudAction = z.infer<typeof HudActionSchema>;
