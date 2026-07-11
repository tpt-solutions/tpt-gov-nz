import type { WorksafeDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceWorksafeAiContext(data: WorksafeDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "worksafe",
    content: "WorkSafe New Zealand local ID: " + data.worksafeId + ".",
    metadata: { area: "worksafe-id" },
  });

  if (data.inspections && data.inspections.length > 0) {
    for (const x of data.inspections) {
      chunks.push({
        deptId: "worksafe",
        content: "Inspections: " + x.reference + ", " + x.site + ", " + x.inspectionDate + ", " + x.outcome,
        metadata: { area: "inspections" },
      });
    }
  }

  if (data.investigations && data.investigations.length > 0) {
    for (const x of data.investigations) {
      chunks.push({
        deptId: "worksafe",
        content: "Investigations: " + x.reference + ", " + x.matter + ", " + x.status + ", " + x.openedDate,
        metadata: { area: "investigations" },
      });
    }
  }

  return chunks;
}
