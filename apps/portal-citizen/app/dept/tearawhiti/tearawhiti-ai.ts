import type { TearawhitiDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceTearawhitiAiContext(data: TearawhitiDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "tearawhiti",
    content: "Te Arawhiti local ID: " + data.tearawhitiId + ".",
    metadata: { area: "tearawhiti-id" },
  });

  if (data.treaty_settlements && data.treaty_settlements.length > 0) {
    for (const x of data.treaty_settlements) {
      chunks.push({
        deptId: "tearawhiti",
        content: "Treaty settlements: " + x.iwi + ", " + x.status + ", " + x.settledDate,
        metadata: { area: "treaty_settlements" },
      });
    }
  }

  if (data.engagements && data.engagements.length > 0) {
    for (const x of data.engagements) {
      chunks.push({
        deptId: "tearawhiti",
        content: "Engagements: " + x.topic + ", " + x.engagementDate + ", " + x.outcome,
        metadata: { area: "engagements" },
      });
    }
  }

  return chunks;
}
