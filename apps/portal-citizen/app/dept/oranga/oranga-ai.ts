import type { OrangaDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceOrangaAiContext(data: OrangaDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "oranga",
    content: "Oranga Tamariki local ID: " + data.orangaId + ".",
    metadata: { area: "oranga-id" },
  });

  if (data.care_placements && data.care_placements.length > 0) {
    for (const x of data.care_placements) {
      chunks.push({
        deptId: "oranga",
        content: "Care placements: " + x.placementType + ", " + x.startDate + ", " + x.region,
        metadata: { area: "care_placements" },
      });
    }
  }

  if (data.support_services && data.support_services.length > 0) {
    for (const x of data.support_services) {
      chunks.push({
        deptId: "oranga",
        content: "Support services: " + x.service + ", " + x.status + ", " + x.nextReview,
        metadata: { area: "support_services" },
      });
    }
  }

  return chunks;
}
