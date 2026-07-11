import type { CaaDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceCaaAiContext(data: CaaDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "caa",
    content: "Civil Aviation Authority local ID: " + data.caaId + ".",
    metadata: { area: "caa-id" },
  });

  if (data.licences && data.licences.length > 0) {
    for (const x of data.licences) {
      chunks.push({
        deptId: "caa",
        content: "Licences: " + x.licenceNo + ", " + x.category + ", " + x.status + ", " + x.expires,
        metadata: { area: "licences" },
      });
    }
  }

  if (data.aircraft && data.aircraft.length > 0) {
    for (const x of data.aircraft) {
      chunks.push({
        deptId: "caa",
        content: "Aircraft: " + x.registration + ", " + x.aircraftType + ", " + x.status,
        metadata: { area: "aircraft" },
      });
    }
  }

  return chunks;
}
