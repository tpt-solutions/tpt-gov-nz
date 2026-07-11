import type { PublicserviceDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function producePublicserviceAiContext(data: PublicserviceDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "publicservice",
    content: "Te Kawa Mataaho Public Service Commission local ID: " + data.publicserviceId + ".",
    metadata: { area: "publicservice-id" },
  });

  if (data.workforce && data.workforce.length > 0) {
    for (const x of data.workforce) {
      chunks.push({
        deptId: "publicservice",
        content: "Workforce: " + x.reportYear + ", " + x.agency + ", " + x.headcount,
        metadata: { area: "workforce" },
      });
    }
  }

  if (data.agency_ratings && data.agency_ratings.length > 0) {
    for (const x of data.agency_ratings) {
      chunks.push({
        deptId: "publicservice",
        content: "Agency ratings: " + x.agency + ", " + x.rating + ", " + x.ratingYear,
        metadata: { area: "agency_ratings" },
      });
    }
  }

  return chunks;
}
