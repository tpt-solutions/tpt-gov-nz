import Link from "next/link";
import { fetchMsdDataForCitizen } from "../actions";

export const metadata = { title: "StudyLink — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MsdStaffStudyLinkPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>StudyLink — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchMsdDataForCitizen(did, ["msd:studylink"]);
  const studylink = data?.studylink;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/msd?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>StudyLink — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load StudyLink record.</p>}
      {data && !studylink && <p>No StudyLink record on file.</p>}

      {studylink && (
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
    </main>
  );
}
