import { z } from "zod";

export const AiLevelSchema = z.enum(["none", "advisory", "assisted", "automated"]);
export type AiLevel = z.infer<typeof AiLevelSchema>;

export const AiProviderSchema = z.enum(["openrouter", "ollama"]);
export type AiProvider = z.infer<typeof AiProviderSchema>;

export const AiConfigSchema = z.object({
  level: AiLevelSchema,
  provider: AiProviderSchema.optional(),
  model: z.string().optional(),
  baseUrl: z.string().url().optional(),
  apiKey: z.string().optional(),
});

export type AiConfig = z.infer<typeof AiConfigSchema>;

export const AiContextChunkSchema = z.object({
  deptId: z.string(),
  content: z.string(),
  metadata: z.record(z.string(), z.unknown()).optional(),
});

export type AiContextChunk = z.infer<typeof AiContextChunkSchema>;

export const AiActionSchema = z.object({
  id: z.string().uuid(),
  type: z.string(),
  deptId: z.string(),
  citizenDid: z.string(),
  description: z.string(),
  parameters: z.record(z.string(), z.unknown()),
  aiLevel: AiLevelSchema,
  status: z.enum(["pending_approval", "approved", "rejected", "executed", "reversed"]),
  humanApprovedBy: z.string().optional(),
  timestamp: z.number(),
});

export type AiAction = z.infer<typeof AiActionSchema>;
