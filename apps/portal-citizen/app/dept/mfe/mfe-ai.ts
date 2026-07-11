import type { MfeDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceMfeAiContext(data: MfeDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "mfe",
    content: "Ministry for the Environment local ID: " + data.mfeId + ".",
    metadata: { area: "mfe-id" },
  });

  if (data.emissions && data.emissions.length > 0) {
    for (const x of data.emissions) {
      chunks.push({
        deptId: "mfe",
        content: "Emissions: " + x.reportYear + ", " + x.sector + ", " + x.tonnesCO2e,
        metadata: { area: "emissions" },
      });
    }
  }

  if (data.reports && data.reports.length > 0) {
    for (const x of data.reports) {
      chunks.push({
        deptId: "mfe",
        content: "Reports: " + x.title + ", " + x.published + ", " + x.status,
        metadata: { area: "reports" },
      });
    }
  }

  return chunks;
}
