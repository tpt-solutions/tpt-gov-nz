import type { AiConfig, AiContextChunk } from "@tpt/gov-schema";
import type { ChatMessage, ChatResponse, AiProvider } from "./types.js";
import { OpenRouterProvider } from "./providers/openrouter.js";
import { OllamaProvider } from "./providers/ollama.js";
import { PiiRedactor } from "./pii-redactor.js";

export class GovAiClient {
  private provider: AiProvider | null = null;
  private readonly redactor = new PiiRedactor();

  constructor(private readonly config: AiConfig) {
    if (config.level === "none") return;

    if (config.provider === "openrouter" && config.apiKey) {
      this.provider = new OpenRouterProvider(config.apiKey, config.model, config.baseUrl);
    } else if (config.provider === "ollama") {
      this.provider = new OllamaProvider(config.baseUrl, config.model);
    }
  }

  get isEnabled(): boolean {
    return this.config.level !== "none" && this.provider !== null;
  }

  get level() {
    return this.config.level;
  }

  async chat(
    systemPrompt: string,
    userMessage: string,
    contextChunks: AiContextChunk[] = []
  ): Promise<ChatResponse> {
    if (!this.provider) {
      throw new Error("AI is disabled (level=none) or not configured");
    }

    const contextSection =
      contextChunks.length > 0
        ? `\n\nRelevant citizen information (consented data):\n${contextChunks
            .map((c) => `[${c.deptId.toUpperCase()}]\n${c.content}`)
            .join("\n\n")}`
        : "";

    const messages: ChatMessage[] = [
      {
        role: "system",
        content: `${systemPrompt}${contextSection}\n\nIMPORTANT: Never reproduce raw identifiers (NHI, IRD numbers, passport numbers) in your responses.`,
      },
      { role: "user", content: this.redactor.redact(userMessage) },
    ];

    return this.provider.chat(messages, this.config.model);
  }

  async isAvailable(): Promise<boolean> {
    if (!this.provider) return false;
    return this.provider.isAvailable();
  }
}
