import type { PacificDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function producePacificAiContext(data: PacificDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "pacific",
    content: "Ministry for Pacific Peoples local ID: " + data.pacificId + ".",
    metadata: { area: "pacific-id" },
  });

  if (data.programmes && data.programmes.length > 0) {
    for (const x of data.programmes) {
      chunks.push({
        deptId: "pacific",
        content: "Programmes: " + x.programmeName + ", " + x.status + ", " + x.year,
        metadata: { area: "programmes" },
      });
    }
  }

  if (data.language_services && data.language_services.length > 0) {
    for (const x of data.language_services) {
      chunks.push({
        deptId: "pacific",
        content: "Language services: " + x.service + ", " + x.region + ", " + x.status,
        metadata: { area: "language_services" },
      });
    }
  }

  return chunks;
}
