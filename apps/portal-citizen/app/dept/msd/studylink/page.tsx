import Link from "next/link";
import StudyLinkForm from "./apply/page";
import { fetchMsdData } from "../actions";

export const metadata = { title: "StudyLink — My Gov NZ" };

export default async function MsdStudyLinkPage() {
  const data = await fetchMsdData(["msd:studylink"]);
  const studylink = data?.studylink;

  return (
    <main>
      <Link href="/dept/msd">← Back to MSD</Link>
      <h1>StudyLink</h1>

      <section>
        <h2>Your StudyLink record</h2>
        {!studylink ? (
          <p>No StudyLink record on file.</p>
        ) : (
          <ul>
            <li>
              <strong>Student loan:</strong> {studylink.hasStudentLoan ? "Yes" : "No"}
              {studylink.loanBalance != null ? ` — balance $${studylink.loanBalance}` : ""}
              {studylink.repaymentPlan ? ` (${studylink.repaymentPlan})` : ""}
            </li>
            <li>
              <strong>Student allowance:</strong> {studylink.hasAllowance ? "Yes" : "No"}
              {studylink.allowanceType ? ` — ${studylink.allowanceType}` : ""}
              {studylink.weeklyAmount != null ? ` ($${studylink.weeklyAmount}/week)` : ""}
              {studylink.nextPaymentDate ? ` · next payment ${studylink.nextPaymentDate}` : ""}
            </li>
          </ul>
        )}
      </section>

      <section>
        <h2>Apply for a student allowance</h2>
        <StudyLinkForm />
      </section>
    </main>
  );
}
