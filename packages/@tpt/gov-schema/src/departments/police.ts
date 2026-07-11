import { z } from "zod";

export const PoliceInfringementSchema = z.object({
  ticketNumber: z.string(),
  offenseType: z.enum(["speeding", "parking", "other"]),
  status: z.enum(["unpaid", "paid", "disputed"]),
  amount: z.number(),
  issueDate: z.string(),
  location: z.string().optional(),
  demeritPoints: z.number().optional(),
  description: z.string(),
});

export const PoliceReportSchema = z.object({
  reportNumber: z.string(),
  reportType: z.enum(["theft", "incident", "lost-property"]),
  status: z.enum(["filed", "under-investigation", "closed"]),
  filedDate: z.string(),
  description: z.string(),
});

export const PoliceDataBundleSchema = z.object({
  clientNumber: z.string(),
  infringements: z.array(PoliceInfringementSchema).optional(),
  reports: z.array(PoliceReportSchema).optional(),
});

export type PoliceDataBundle = z.infer<typeof PoliceDataBundleSchema>;

export const PoliceActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("pay-infringement"),
    ticketNumber: z.string(),
  }),
  z.object({
    type: z.literal("dispute-infringement"),
    ticketNumber: z.string(),
    reason: z.string().min(1),
  }),
  z.object({
    type: z.literal("file-report"),
    reportType: z.enum(["theft", "incident", "lost-property"]),
    description: z.string().min(1),
  }),
]);

export type PoliceAction = z.infer<typeof PoliceActionSchema>;
