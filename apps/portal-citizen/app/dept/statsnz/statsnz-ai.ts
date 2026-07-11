import type { StatsNZDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceStatsnzAiContext(data: StatsNZDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "statsnz",
    content: `Statistics NZ local ID: ${data.statsId}.`,
    metadata: { area: "stats-id" },
  });

  if (data.census && data.census.length > 0) {
    for (const c of data.census) {
      chunks.push({
        deptId: "statsnz",
        content: `Census ${c.censusYear}: ${c.dwellingType} dwelling in ${c.region}, household of ${c.householdSize}.`,
        metadata: { area: "census", censusYear: c.censusYear.toString() },
      });
    }
  }

  if (data.profile) {
    chunks.push({
      deptId: "statsnz",
      content: `Data profile: ${data.profile.dataSummary} ${data.profile.recordCount} records, last updated ${data.profile.lastUpdated}.`,
      metadata: { area: "profile" },
    });
  }

  return chunks;
}
