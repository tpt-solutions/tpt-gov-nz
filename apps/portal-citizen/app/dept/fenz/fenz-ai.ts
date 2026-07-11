import type { FenzDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceFenzAiContext(data: FenzDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "fenz",
    content: "Fire and Emergency New Zealand local ID: " + data.fenzId + ".",
    metadata: { area: "fenz-id" },
  });

  if (data.fire_safety) {
    chunks.push({
      deptId: "fenz",
      content: "Fire safety: " + data.fire_safety.property + ", " + data.fire_safety.grade + ", " + data.fire_safety.lastInspection,
      metadata: { area: "fire_safety" },
    });
  }

  if (data.incidents && data.incidents.length > 0) {
    for (const x of data.incidents) {
      chunks.push({
        deptId: "fenz",
        content: "Incidents: " + x.reference + ", " + x.incidentType + ", " + x.incidentDate + ", " + x.status,
        metadata: { area: "incidents" },
      });
    }
  }

  return chunks;
}
