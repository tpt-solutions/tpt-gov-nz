import { z } from "zod";
import { type Scope } from "../identity.js";
import { type AiContextChunk } from "../ai.js";

export const CitizenRefSchema = z.object({
  did: z.string(),
  deptLocalId: z.string(),
  displayName: z.string().optional(),
});

export type CitizenRef = z.infer<typeof CitizenRefSchema>;

export const ServiceSchema = z.object({
  id: z.string(),
  name: z.string(),
  description: z.string(),
  requiredScopes: z.array(z.string()),
  url: z.string().optional(),
});

export type Service = z.infer<typeof ServiceSchema>;

export const DeptActionSchema = z.object({
  type: z.string(),
  parameters: z.record(z.string(), z.unknown()),
});

export type DeptAction = z.infer<typeof DeptActionSchema>;

export const ActionResultSchema = z.object({
  success: z.boolean(),
  message: z.string().optional(),
  data: z.unknown().optional(),
});

export type ActionResult = z.infer<typeof ActionResultSchema>;

export interface DeptAdapter {
  readonly deptId: string;
  readonly displayName: string;

  resolveCitizen(did: string): Promise<CitizenRef>;
  fetchConsentedData(did: string, scopes: Scope[]): Promise<unknown>;
  submitAction(did: string, action: DeptAction): Promise<ActionResult>;
  listServices(): Promise<Service[]>;
  produceAiContext(bundle: unknown): AiContextChunk[];
}
