import { z } from "zod";

export const MOHDataBundleSchema = z.object({
  nhiNumber: z.string(),
  enrolledGP: z.object({
    practiceName: z.string(),
    address: z.string(),
    phone: z.string(),
  }).optional(),
  activePrescriptions: z.array(
    z.object({
      medication: z.string(),
      dose: z.string(),
      repeatsRemaining: z.number(),
      issuedAt: z.string().optional(),
    })
  ).optional(),
  upcomingAppointments: z.array(
    z.object({
      provider: z.string(),
      date: z.string(),
      type: z.string(),
      status: z.string().optional(),
    })
  ).optional(),
  vaccinations: z.array(
    z.object({
      vaccine: z.string(),
      date: z.string(),
      dueForBooster: z.boolean().optional(),
    })
  ).optional(),
});

export type MOHDataBundle = z.infer<typeof MOHDataBundleSchema>;
