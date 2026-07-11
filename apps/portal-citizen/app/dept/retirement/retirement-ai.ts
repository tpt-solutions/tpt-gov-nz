import type { RetirementDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceRetirementAiContext(data: RetirementDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "retirement",
    content: "Retirement Commission (Te Ara Ahunga Ora) local ID: " + data.retirementId + ".",
    metadata: { area: "retirement-id" },
  });

  if (data.retirement_plan) {
    chunks.push({
      deptId: "retirement",
      content: "Retirement plan: " + data.retirement_plan.hasPlan + ", " + data.retirement_plan.retirementAge + ", " + data.retirement_plan.lastReview,
      metadata: { area: "retirement_plan" },
    });
  }

  if (data.guidance && data.guidance.length > 0) {
    for (const x of data.guidance) {
      chunks.push({
        deptId: "retirement",
        content: "Guidance: " + x.topic + ", " + x.summary + ", " + x.published,
        metadata: { area: "guidance" },
      });
    }
  }

  return chunks;
}
