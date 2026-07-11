import type { RegulationDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceRegulationAiContext(data: RegulationDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "regulation",
    content: "Ministry for Regulation local ID: " + data.regulationId + ".",
    metadata: { area: "regulation-id" },
  });

  if (data.regulatory_reviews && data.regulatory_reviews.length > 0) {
    for (const x of data.regulatory_reviews) {
      chunks.push({
        deptId: "regulation",
        content: "Regulatory reviews: " + x.topic + ", " + x.agency + ", " + x.status + ", " + x.reviewYear,
        metadata: { area: "regulatory_reviews" },
      });
    }
  }

  if (data.proposals && data.proposals.length > 0) {
    for (const x of data.proposals) {
      chunks.push({
        deptId: "regulation",
        content: "Proposals: " + x.title + ", " + x.status,
        metadata: { area: "proposals" },
      });
    }
  }

  return chunks;
}
