import type { DefenceDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceDefenceAiContext(data: DefenceDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "defence",
    content: "Ministry of Defence local ID: " + data.defenceId + ".",
    metadata: { area: "defence-id" },
  });

  if (data.procurements && data.procurements.length > 0) {
    for (const x of data.procurements) {
      chunks.push({
        deptId: "defence",
        content: "Procurements: " + x.programme + ", " + x.value + ", " + x.status,
        metadata: { area: "procurements" },
      });
    }
  }

  if (data.bases && data.bases.length > 0) {
    for (const x of data.bases) {
      chunks.push({
        deptId: "defence",
        content: "Bases: " + x.name + ", " + x.location + ", " + x.status,
        metadata: { area: "bases" },
      });
    }
  }

  return chunks;
}
