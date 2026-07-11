import type { DpmcDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceDpmcAiContext(data: DpmcDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "dpmc",
    content: "Department of the Prime Minister and Cabinet local ID: " + data.dpmcId + ".",
    metadata: { area: "dpmc-id" },
  });

  if (data.honours && data.honours.length > 0) {
    for (const x of data.honours) {
      chunks.push({
        deptId: "dpmc",
        content: "Honours: " + x.awardYear + ", " + x.award + ", " + x.status,
        metadata: { area: "honours" },
      });
    }
  }

  if (data.engagements && data.engagements.length > 0) {
    for (const x of data.engagements) {
      chunks.push({
        deptId: "dpmc",
        content: "Engagements: " + x.eventName + ", " + x.eventDate + ", " + x.location,
        metadata: { area: "engagements" },
      });
    }
  }

  return chunks;
}
