import type { MOHDataBundle, AiContextChunk } from "@tpt/gov-schema";

/** Build the AI context chunks from a MOH data bundle. */
export function produceMohAiContext(data: MOHDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "moh",
    content: `National Health Index number ends in ${data.nhiNumber.slice(-4)}.`,
    metadata: { area: "nhi" },
  });

  if (data.enrolledGP) {
    chunks.push({
      deptId: "moh",
      content: `Enrolled with ${data.enrolledGP.practiceName} (${data.enrolledGP.address}).`,
      metadata: { area: "gp" },
    });
  }

  for (const p of data.activePrescriptions ?? []) {
    chunks.push({
      deptId: "moh",
      content: `Prescription ${p.medication} ${p.dose}, ${p.repeatsRemaining} repeat(s) remaining.`,
      metadata: { area: "prescription" },
    });
  }

  for (const v of data.vaccinations ?? []) {
    const booster = v.dueForBooster ? " — booster due" : "";
    chunks.push({
      deptId: "moh",
      content: `Vaccination ${v.vaccine} on ${v.date}${booster}.`,
      metadata: { area: "vaccination" },
    });
  }

  return chunks;
}
