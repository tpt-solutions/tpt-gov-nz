import type { SfoDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceSfoAiContext(data: SfoDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "sfo",
    content: "Serious Fraud Office local ID: " + data.sfoId + ".",
    metadata: { area: "sfo-id" },
  });

  if (data.investigations && data.investigations.length > 0) {
    for (const x of data.investigations) {
      chunks.push({
        deptId: "sfo",
        content: "Investigations: " + x.reference + ", " + x.matter + ", " + x.status + ", " + x.openedDate,
        metadata: { area: "investigations" },
      });
    }
  }

  if (data.outcomes && data.outcomes.length > 0) {
    for (const x of data.outcomes) {
      chunks.push({
        deptId: "sfo",
        content: "Outcomes: " + x.reference + ", " + x.result + ", " + x.resultDate,
        metadata: { area: "outcomes" },
      });
    }
  }

  return chunks;
}
