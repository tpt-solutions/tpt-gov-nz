import type { WomenDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceWomenAiContext(data: WomenDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "women",
    content: "Ministry for Women local ID: " + data.womenId + ".",
    metadata: { area: "women-id" },
  });

  if (data.programmes && data.programmes.length > 0) {
    for (const x of data.programmes) {
      chunks.push({
        deptId: "women",
        content: "Programmes: " + x.programmeName + ", " + x.status + ", " + x.year,
        metadata: { area: "programmes" },
      });
    }
  }

  if (data.insights && data.insights.length > 0) {
    for (const x of data.insights) {
      chunks.push({
        deptId: "women",
        content: "Insights: " + x.topic + ", " + x.summary + ", " + x.published,
        metadata: { area: "insights" },
      });
    }
  }

  return chunks;
}
