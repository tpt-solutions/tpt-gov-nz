import type { MSDDataBundle, AiContextChunk } from "@tpt/gov-schema";

export function produceMsdAiContext(data: MSDDataBundle): AiContextChunk[] {
  const chunks: AiContextChunk[] = [];

  chunks.push({
    deptId: "msd",
    content: `Client number: ${data.clientNumber}.`,
    metadata: { area: "client-number" },
  });

  if (data.studylink) {
    const s = data.studylink;
    const parts: string[] = [];
    if (s.hasStudentLoan) {
      parts.push(
        `StudyLink student loan: active${s.loanBalance != null ? `, balance $${s.loanBalance}` : ""}${s.repaymentPlan ? `, repayment plan ${s.repaymentPlan}` : ""}.`
      );
    } else {
      parts.push("StudyLink student loan: none.");
    }
    if (s.hasAllowance) {
      parts.push(
        `Student allowance: ${s.allowanceType ?? "yes"}${s.weeklyAmount != null ? `, $${s.weeklyAmount}/week` : ""}${s.nextPaymentDate ? `, next payment ${s.nextPaymentDate}` : ""}.`
      );
    } else {
      parts.push("Student allowance: none.");
    }
    chunks.push({
      deptId: "msd",
      content: parts.join(" "),
      metadata: { area: "studylink" },
    });
  }

  if (data.caseHistory && data.caseHistory.length > 0) {
    for (const e of data.caseHistory) {
      chunks.push({
        deptId: "msd",
        content: `Case event ${e.eventDate} (${e.serviceLine}): ${e.summary}.`,
        metadata: { area: "case-history", eventDate: e.eventDate, serviceLine: e.serviceLine },
      });
    }
  }

  return chunks;
}
