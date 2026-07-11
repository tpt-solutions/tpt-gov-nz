import type { EthnicDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceEthnicAiContext(data: EthnicDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "ethnic",
    content: "Ministry for Ethnic Communities local ID: " + data.ethnicId + ".",
    metadata: { area: "ethnic-id" },
  });

  if (data.programmes && data.programmes.length > 0) {
    for (const x of data.programmes) {
      chunks.push({
        deptId: "ethnic",
        content: "Programmes: " + x.programmeName + ", " + x.status + ", " + x.year,
        metadata: { area: "programmes" },
      });
    }
  }

  if (data.community_grants && data.community_grants.length > 0) {
    for (const x of data.community_grants) {
      chunks.push({
        deptId: "ethnic",
        content: "Community grants: " + x.grantName + ", " + x.amount + ", " + x.status,
        metadata: { area: "community_grants" },
      });
    }
  }

  return chunks;
}
