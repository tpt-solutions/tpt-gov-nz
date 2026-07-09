import type { AiConfig, AiLevel } from "@tpt/gov-schema";

export interface AiClientOptions {
  config: AiConfig;
}

export interface ChatMessage {
  role: "system" | "user" | "assistant";
  content: string;
}

export interface ChatResponse {
  content: string;
  model: string;
  provider: string;
  tokensUsed?: number;
}

export interface AiProvider {
  chat(messages: ChatMessage[], model: string): Promise<ChatResponse>;
  isAvailable(): Promise<boolean>;
}

export { AiLevel };
