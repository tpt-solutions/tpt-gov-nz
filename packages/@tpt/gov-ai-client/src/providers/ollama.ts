import type { AiProvider, ChatMessage, ChatResponse } from "../types.js";

export class OllamaProvider implements AiProvider {
  constructor(
    private readonly baseUrl: string = "http://localhost:11434",
    private readonly defaultModel: string = "llama3.2"
  ) {}

  async chat(messages: ChatMessage[], model?: string): Promise<ChatResponse> {
    const res = await fetch(`${this.baseUrl}/api/chat`, {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        model: model ?? this.defaultModel,
        messages,
        stream: false,
      }),
    });

    if (!res.ok) {
      const err = await res.text();
      throw new Error(`Ollama error ${res.status}: ${err}`);
    }

    const data = (await res.json()) as {
      message: { content: string };
      model: string;
      eval_count?: number;
    };

    return {
      content: data.message.content,
      model: data.model,
      provider: "ollama",
      tokensUsed: data.eval_count,
    };
  }

  async isAvailable(): Promise<boolean> {
    try {
      const res = await fetch(`${this.baseUrl}/api/tags`);
      return res.ok;
    } catch {
      return false;
    }
  }
}
