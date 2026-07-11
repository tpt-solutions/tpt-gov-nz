import { z } from "zod";

export const LinzTitleSchema = z.object({
  titleNumber: z.string(),
  propertyAddress: z.string(),
  landAreaSqm: z.number(),
  estateType: z.string(),
});

export const LinzOwnershipSchema = z.object({
  titleNumber: z.string(),
  ownershipShare: z.string(),
  registeredOwners: z.array(z.string()),
});

export const LINZDataBundleSchema = z.object({
  customerId: z.string(),
  titles: z.array(LinzTitleSchema).optional(),
  ownership: z.array(LinzOwnershipSchema).optional(),
});

export type LINZDataBundle = z.infer<typeof LINZDataBundleSchema>;

export const LinzActionSchema = z.discriminatedUnion("type", [
  z.object({
    type: z.literal("request-title-copy"),
    titleNumber: z.string().min(1),
  }),
  z.object({
    type: z.literal("update-mailing-address"),
    address: z.string().min(1),
  }),
]);

export type LinzAction = z.infer<typeof LinzActionSchema>;
