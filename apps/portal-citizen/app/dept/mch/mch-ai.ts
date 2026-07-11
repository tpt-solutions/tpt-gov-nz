import type { MchDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceMchAiContext(data: MchDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "mch",
    content: "Ministry for Culture and Heritage local ID: " + data.mchId + ".",
    metadata: { area: "mch-id" },
  });

  if (data.heritage_sites && data.heritage_sites.length > 0) {
    for (const x of data.heritage_sites) {
      chunks.push({
        deptId: "mch",
        content: "Heritage sites: " + x.name + ", " + x.status + ", " + x.region,
        metadata: { area: "heritage_sites" },
      });
    }
  }

  if (data.grants && data.grants.length > 0) {
    for (const x of data.grants) {
      chunks.push({
        deptId: "mch",
        content: "Grants: " + x.grantName + ", " + x.amount + ", " + x.status,
        metadata: { area: "grants" },
      });
    }
  }

  return chunks;
}
