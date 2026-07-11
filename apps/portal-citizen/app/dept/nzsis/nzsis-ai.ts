import type { NzsisDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceNzsisAiContext(data: NzsisDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "nzsis",
    content: "New Zealand Security Intelligence Service local ID: " + data.nzsisId + ".",
    metadata: { area: "nzsis-id" },
  });

  if (data.mandates && data.mandates.length > 0) {
    for (const x of data.mandates) {
      chunks.push({
        deptId: "nzsis",
        content: "Mandates: " + x.reference + ", " + x.agency + ", " + x.status + ", " + x.issuedDate,
        metadata: { area: "mandates" },
      });
    }
  }

  if (data.threats && data.threats.length > 0) {
    for (const x of data.threats) {
      chunks.push({
        deptId: "nzsis",
        content: "Threats: " + x.reference + ", " + x.category + ", " + x.status + ", " + x.assessedDate,
        metadata: { area: "threats" },
      });
    }
  }

  return chunks;
}
