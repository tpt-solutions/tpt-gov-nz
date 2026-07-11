import type { MotDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceMotAiContext(data: MotDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "mot",
    content: "Ministry of Transport local ID: " + data.motId + ".",
    metadata: { area: "mot-id" },
  });

  if (data.strategies && data.strategies.length > 0) {
    for (const x of data.strategies) {
      chunks.push({
        deptId: "mot",
        content: "Strategies: " + x.title + ", " + x.year + ", " + x.status,
        metadata: { area: "strategies" },
      });
    }
  }

  if (data.programmes && data.programmes.length > 0) {
    for (const x of data.programmes) {
      chunks.push({
        deptId: "mot",
        content: "Programmes: " + x.name + ", " + x.budget + ", " + x.status,
        metadata: { area: "programmes" },
      });
    }
  }

  return chunks;
}
