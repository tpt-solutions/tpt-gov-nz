import type { GcsbDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceGcsbAiContext(data: GcsbDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "gcsb",
    content: "Government Communications Security Bureau local ID: " + data.gcsbId + ".",
    metadata: { area: "gcsb-id" },
  });

  if (data.mandates && data.mandates.length > 0) {
    for (const x of data.mandates) {
      chunks.push({
        deptId: "gcsb",
        content: "Mandates: " + x.reference + ", " + x.agency + ", " + x.status + ", " + x.issuedDate,
        metadata: { area: "mandates" },
      });
    }
  }

  if (data.engagements && data.engagements.length > 0) {
    for (const x of data.engagements) {
      chunks.push({
        deptId: "gcsb",
        content: "Engagements: " + x.partner + ", " + x.engagementType + ", " + x.engagementDate,
        metadata: { area: "engagements" },
      });
    }
  }

  return chunks;
}
