"use server";

import { GovAiClient } from "@tpt/gov-ai-client";
import type { AiConfig } from "@tpt/gov-schema";
import { getScenario } from "@/app/lib/policies";

export interface SimulationResult {
  ok: boolean;
  report?: string;
  error?: string;
}

function buildAiConfig(): AiConfig {
  const provider = (process.env.AI_PROVIDER ?? "ollama") as AiConfig["provider"];
  const config: AiConfig = {
    level: (process.env.AI_LEVEL as AiConfig["level"]) ?? "advisory",
    provider,
    model: process.env.AI_MODEL,
    baseUrl:
      provider === "ollama"
        ? process.env.OLLAMA_BASE_URL ?? "http://localhost:11434"
        : process.env.AI_BASE_URL,
    apiKey: process.env.AI_API_KEY,
  };
  return config;
}

const SYSTEM_PROMPT = `You are a senior public-policy analyst working for a whole-of-government simulation lab.
Given a proposed policy change and the departments it touches, produce a structured impact simulation.

Always include these sections:
1. Summary — one paragraph on the change.
2. Affected departments — what each must do differently.
3. Citizen impact — who is better/worse off and by how much (estimates only).
4. Fiscal effect — revenue/spend direction and magnitude (clearly labelled as illustrative).
5. Risks & trade-offs — unintended consequences.
6. Monitoring — what metrics to watch if trialled.

Write in plain, neutral language suitable for policy makers. Never invent specific citizen identifiers.
This is a simulation for planning purposes, not legal or fiscal advice.`;

export async function simulatePolicy(
  _prev: SimulationResult,
  formData: FormData,
): Promise<SimulationResult> {
  const id = String(formData.get("scenario") ?? "");
  const scenario = getScenario(id);
  if (!scenario) {
    return { ok: false, error: "Unknown policy scenario." };
  }

  const config = buildAiConfig();
  const client = new GovAiClient(config);

  if (!client.isEnabled) {
    return {
      ok: false,
      error:
        "AI simulation is disabled (AI_LEVEL=none or provider not configured). Set AI_LEVEL=advisory and configure a provider to run simulations.",
    };
  }

  const userMessage = `Proposed change: ${scenario.change}

Affected departments: ${scenario.affectedDepartments.join(", ")}.
Key parameter: ${scenario.parameter}.

Run the cross-department impact simulation.`;

  try {
    const response = await client.chat(SYSTEM_PROMPT, userMessage);
    return { ok: true, report: response.content };
  } catch (err) {
    return {
      ok: false,
      error: err instanceof Error ? err.message : "Simulation failed.",
    };
  }
}
