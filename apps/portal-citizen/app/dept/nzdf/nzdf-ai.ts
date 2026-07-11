import type { NzdfDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceNzdfAiContext(data: NzdfDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "nzdf",
    content: "New Zealand Defence Force local ID: " + data.nzdfId + ".",
    metadata: { area: "nzdf-id" },
  });

  if (data.service_records && data.service_records.length > 0) {
    for (const x of data.service_records) {
      chunks.push({
        deptId: "nzdf",
        content: "Service records: " + x.serviceNo + ", " + x.branch + ", " + x.status,
        metadata: { area: "service_records" },
      });
    }
  }

  if (data.deployments && data.deployments.length > 0) {
    for (const x of data.deployments) {
      chunks.push({
        deptId: "nzdf",
        content: "Deployments: " + x.operation + ", " + x.country + ", " + x.year,
        metadata: { area: "deployments" },
      });
    }
  }

  return chunks;
}
