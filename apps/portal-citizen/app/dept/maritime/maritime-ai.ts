import type { MaritimeDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceMaritimeAiContext(data: MaritimeDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "maritime",
    content: "Maritime New Zealand local ID: " + data.maritimeId + ".",
    metadata: { area: "maritime-id" },
  });

  if (data.vessels && data.vessels.length > 0) {
    for (const x of data.vessels) {
      chunks.push({
        deptId: "maritime",
        content: "Vessels: " + x.vesselName + ", " + x.flag + ", " + x.status,
        metadata: { area: "vessels" },
      });
    }
  }

  if (data.incidents && data.incidents.length > 0) {
    for (const x of data.incidents) {
      chunks.push({
        deptId: "maritime",
        content: "Incidents: " + x.reference + ", " + x.incidentType + ", " + x.incidentDate + ", " + x.status,
        metadata: { area: "incidents" },
      });
    }
  }

  return chunks;
}
