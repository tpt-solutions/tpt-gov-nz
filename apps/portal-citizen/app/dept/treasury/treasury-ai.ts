import type { TreasuryDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceTreasuryAiContext(data: TreasuryDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "treasury",
    content: "The Treasury local ID: " + data.treasuryId + ".",
    metadata: { area: "treasury-id" },
  });

  if (data.budget && data.budget.length > 0) {
    for (const x of data.budget) {
      chunks.push({
        deptId: "treasury",
        content: "Budget: " + x.fiscalYear + ", " + x.portfolio + ", " + x.appropriation + ", " + x.amount,
        metadata: { area: "budget" },
      });
    }
  }

  if (data.economic_outlook) {
    chunks.push({
      deptId: "treasury",
      content: "Economic outlook: " + data.economic_outlook.forecastYear + ", " + data.economic_outlook.gdpGrowthPct + ", " + data.economic_outlook.inflationPct + ", " + data.economic_outlook.netDebtPct,
      metadata: { area: "economic_outlook" },
    });
  }

  return chunks;
}
