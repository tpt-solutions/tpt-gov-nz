import { GovAiClient } from "@tpt/gov-ai-client";
import type { AiConfig, AiContextChunk } from "@tpt/gov-schema";
import { getAiLevel } from "./level";

const SYSTEM_PROMPT = `You are an assistant for the New Zealand government portal, helping citizens
understand their Inland Revenue entitlements. Use only the consented data provided. Be concise,
plain-language, and never invent figures. If information is missing, say so. Do not reveal raw
identifiers such as IRD numbers.`;

export function buildAiClient(): GovAiClient {
  const level = getAiLevel();
  const config: AiConfig = {
    level,
    provider: (process.env.TPT__GOV__AI_PROVIDER as AiConfig["provider"]) ?? undefined,
    model: process.env.TPT__GOV__AI_MODEL,
    baseUrl: process.env.TPT__GOV__AI_BASE_URL,
    apiKey: process.env.TPT__GOV__AI_API_KEY,
  };
  return new GovAiClient(config);
}

export interface AiAnswer {
  answer: string;
  enabled: boolean;
}

export async function askWithContext(question: string, context: AiContextChunk[]): Promise<AiAnswer> {
  const client = buildAiClient();
  if (!client.isEnabled) {
    return {
      answer: "AI assistance is currently disabled for this portal. Set an AI level to enable it.",
      enabled: false,
    };
  }
  const res = await client.chat(SYSTEM_PROMPT, question, context);
  return { answer: res.content, enabled: true };
}
