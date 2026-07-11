import { z } from "zod";

export const CustomsTravelSchema = z.object({
  passportNumber: z.string(),
  lastArrival: z.string(),
  arrivalPort: z.string(),
  frequentTraveller: z.boolean(),
});

export const CustomsDeclarationSchema = z.object({
  declarationId: z.string(),
  date: z.string(),
  countryFrom: z.string(),
  goodsDeclared: z.string(),
  status: z.string(),
});

export const CUSTOMSDataBundleSchema = z.object({
  travellerId: z.string(),
  travel: CustomsTravelSchema.optional(),
  declarations: z.array(CustomsDeclarationSchema).optional(),
});

export type CUSTOMSDataBundle = z.infer<typeof CUSTOMSDataBundleSchema>;
