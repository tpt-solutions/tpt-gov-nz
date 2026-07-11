import type { ACCDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceAccAiContext(data: ACCDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "acc",
    content: `ACC client number: ${data.clientNumber}.`,
    metadata: { area: "client-number" },
  });

  if (data.claims && data.claims.length > 0) {
    for (const c of data.claims) {
      chunks.push({
        deptId: "acc",
        content: `Claim ${c.claimNumber} (${c.claimType}): ${c.status}. ${c.description}. Injury date ${c.injuryDate}.${c.weeklyCompensation != null ? ` Weekly compensation $${c.weeklyCompensation}.` : ""}`,
        metadata: { area: "claim", claimNumber: c.claimNumber },
      });
    }
  }

  if (data.entitlements) {
    const e = data.entitlements;
    chunks.push({
      deptId: "acc",
      content: e.hasEntitlement
        ? `Entitlement: ${e.type ?? "active"}${e.weeklyAmount != null ? `, $${e.weeklyAmount}/week` : ""}${e.remainingWeeks != null ? `, ${e.remainingWeeks} weeks remaining` : ""}.`
        : "No current ACC entitlement.",
      metadata: { area: "entitlement" },
    });
  }

  if (data.rehabilitation && data.rehabilitation.length > 0) {
    for (const r of data.rehabilitation) {
      chunks.push({
        deptId: "acc",
        content: `Rehabilitation plan ${r.planId}: ${r.status}. ${r.description}.${r.provider ? ` Provider: ${r.provider}.` : ""}${r.nextReview ? ` Next review ${r.nextReview}.` : ""}`,
        metadata: { area: "rehabilitation", planId: r.planId },
      });
    }
  }

  return chunks;
}
