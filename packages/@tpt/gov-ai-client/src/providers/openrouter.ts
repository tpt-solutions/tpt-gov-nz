import type { AiProvider, ChatMessage, ChatResponse } from "../types.js";

export class OpenRouterProvider implements AiProvider {
  private readonly baseUrl: string;

  constructor(
    private readonly apiKey: string,
    private readonly defaultModel: string = "anthropic/claude-sonnet-4-6",
    baseUrl = "https://openrouter.ai/api/v1"
  ) {
    this.baseUrl = baseUrl;
  }

  async chat(messages: ChatMessage[], model?: string): Promise<ChatResponse> {
    const res = await fetch(`${this.baseUrl}/chat/completions`, {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        Authorization: `Bearer ${this.apiKey}`,
        "HTTP-Referer": "https://github.com/tpt-open-source/tpt-gov-nz",
        "X-Title": "tpt-gov-nz",
      },
      body: JSON.stringify({
        model: model ?? this.defaultModel,
        messages,
      }),
    });

    if (!res.ok) {
      const err = await res.text();
      throw new Error(`OpenRouter error ${res.status}: ${err}`);
    }

    const data = (await res.json()) as {
      choices: Array<{ message: { content: string } }>;
      model: string;
      usage?: { total_tokens: number };
    };

    return {
      content: data.choices[0]?.message.content ?? "",
      model: data.model,
      provider: "openrouter",
      tokensUsed: data.usage?.total_tokens,
    };
  }

  async isAvailable(): Promise<boolean> {
    try {
      const res = await fetch(`${this.baseUrl}/models`, {
        headers: { Authorization: `Bearer ${this.apiKey}` },
      });
      return res.ok;
    } catch {
      return false;
    }
  }
}
