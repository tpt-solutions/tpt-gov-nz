import type { AiContextChunk, IRDDataBundle, WINZDataBundle, MOHDataBundle, DIADataBundle } from "@tpt/gov-schema";

function money(n: unknown): string {
  const v = Number(n);
  return Number.isFinite(v) ? `$${v.toLocaleString("en-NZ")}` : "n/a";
}

function irdContext(b: IRDDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];
  const tax = b.currentTaxYear;
  if (tax) {
    chunks.push({
      deptId: "ird",
      content: `Tax year ${tax.assessmentYear} (${tax.assessmentStatus}). Total income ${money(tax.totalIncome)}, tax paid ${money(tax.taxPaid)}, refund due ${money(tax.taxRefundDue)}, owing ${money(tax.taxOwing)}.`,
      metadata: { area: "tax" },
    });
  }
  if (b.kiwiSaver) {
    const k = b.kiwiSaver;
    chunks.push({
      deptId: "ird",
      content: `KiwiSaver: ${k.membershipStatus}, contribution rate ${k.contributionRate}%, balance ${k.totalBalance != null ? money(k.totalBalance) : "unknown"}.`,
      metadata: { area: "kiwisaver" },
    });
  }
  if (b.workingForFamilies) {
    const w = b.workingForFamilies;
    chunks.push({
      deptId: "ird",
      content: `Working for Families: ${w.eligible ? "eligible" : "not eligible"}, ${w.numberOfDependantChildren} dependent children, income threshold ${money(w.incomeThreshold)}.${w.currentEntitlement ? ` Weekly entitlement ${money(w.currentEntitlement.totalWeeklyEntitlement)}.` : ""}`,
      metadata: { area: "working-for-families", eligible: w.eligible },
    });
  }
  return chunks;
}

function winzContext(b: WINZDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];
  chunks.push({
    deptId: "winz",
    content: `Work and Income client ${b.clientId}. Total weekly payment ${money(b.totalWeeklyPayment)} across ${b.activeBenefits.length} active benefit(s).`,
    metadata: { area: "summary" },
  });
  for (const ben of b.activeBenefits) {
    chunks.push({
      deptId: "winz",
      content: `Benefit ${ben.type}: ${money(ben.weeklyAmount)}/week, status ${ben.status}${ben.reviewDate ? `, review ${ben.reviewDate}` : ""}.`,
      metadata: { area: "benefit" },
    });
  }
  return chunks;
}

function mohContext(b: MOHDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];
  if (b.enrolledGP) {
    chunks.push({
      deptId: "moh",
      content: `Enrolled with ${b.enrolledGP.practiceName} (${b.enrolledGP.address}).`,
      metadata: { area: "gp" },
    });
  }
  for (const p of b.activePrescriptions ?? []) {
    chunks.push({
      deptId: "moh",
      content: `Prescription ${p.medication} ${p.dose}, ${p.repeatsRemaining} repeat(s) remaining.`,
      metadata: { area: "prescription" },
    });
  }
  for (const v of b.vaccinations ?? []) {
    chunks.push({
      deptId: "moh",
      content: `Vaccination ${v.vaccine} on ${v.date}${v.dueForBooster ? " — booster due" : ""}.`,
      metadata: { area: "vaccination" },
    });
  }
  return chunks;
}

function diaContext(b: DIADataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];
  if (b.passport) {
    const expiringSoon = new Date(b.passport.expiryDate).getTime() - Date.now() < 1000 * 60 * 60 * 24 * 365;
    chunks.push({
      deptId: "dia",
      content: `Passport expires ${b.passport.expiryDate}.${b.passport.renewable ? " Renewable online." : ""}${expiringSoon ? " Expiring within a year." : ""}`,
      metadata: { area: "passport" },
    });
  }
  if (b.citizenship) {
    chunks.push({
      deptId: "dia",
      content: `Citizenship status: ${b.citizenship.status}.`,
      metadata: { area: "citizenship" },
    });
  }
  return chunks;
}

export interface Bundles {
  ird?: IRDDataBundle;
  winz?: WINZDataBundle;
  moh?: MOHDataBundle;
  dia?: DIADataBundle;
}

export function buildAiContext(bundles: Bundles): AiContextChunk[] {
  return [
    ...(bundles.ird ? irdContext(bundles.ird) : []),
    ...(bundles.winz ? winzContext(bundles.winz) : []),
    ...(bundles.moh ? mohContext(bundles.moh) : []),
    ...(bundles.dia ? diaContext(bundles.dia) : []),
  ];
}
