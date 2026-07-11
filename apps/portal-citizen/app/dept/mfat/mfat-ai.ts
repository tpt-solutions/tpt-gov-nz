import type { MfatDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceMfatAiContext(data: MfatDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "mfat",
    content: "Ministry of Foreign Affairs and Trade local ID: " + data.mfatId + ".",
    metadata: { area: "mfat-id" },
  });

  if (data.overseas_missions && data.overseas_missions.length > 0) {
    for (const x of data.overseas_missions) {
      chunks.push({
        deptId: "mfat",
        content: "Overseas missions: " + x.country + ", " + x.status,
        metadata: { area: "overseas_missions" },
      });
    }
  }

  if (data.travel_advisories && data.travel_advisories.length > 0) {
    for (const x of data.travel_advisories) {
      chunks.push({
        deptId: "mfat",
        content: "Travel advisories: " + x.country + ", " + x.level + ", " + x.updated,
        metadata: { area: "travel_advisories" },
      });
    }
  }

  return chunks;
}
