import Link from "next/link";
import { fetchMsdDataForCitizen } from "./actions";

export const metadata = { title: "Ministry of Social Development — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MsdStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Ministry of Social Development — Case File</h1>
        <p>No citizen selected. Enter a DID to view their MSD records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchMsdDataForCitizen(did, ["msd:studylink", "msd:case-history"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry of Social Development — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load MSD information for this citizen.</p>}

      {data && (
        <>
          <p>Client number: ••••{data.clientNumber.slice(-4)}</p>

          <section>
            <h2>StudyLink</h2>
            {data.studylink ? (
              <ul>
                <li>
                  <strong>Student loan:</strong> {data.studylink.hasStudentLoan ? "Yes" : "No"}
                  {data.studylink.loanBalance != null ? ` — balance $${data.studylink.loanBalance}` : ""}
                  {data.studylink.repaymentPlan ? ` (${data.studylink.repaymentPlan})` : ""}
                </li>
                <li>
                  <strong>Student allowance:</strong> {data.studylink.hasAllowance ? "Yes" : "No"}
                  {data.studylink.allowanceType ? ` — ${data.studylink.allowanceType}` : ""}
                  {data.studylink.weeklyAmount != null ? ` ($${data.studylink.weeklyAmount}/week)` : ""}
                </li>
              </ul>
            ) : (
              <p>No StudyLink record on file.</p>
            )}
          </section>

          <section>
            <h2>Case History</h2>
            {data.caseHistory && data.caseHistory.length > 0 ? (
              <ul>
                {data.caseHistory.map((e, i) => (
                  <li key={`${e.eventDate}-${i}`}>
                    <strong>{e.eventDate}</strong> — {e.serviceLine}: {e.summary}
                  </li>
                ))}
              </ul>
            ) : (
              <p>No case history on record.</p>
            )}
          </section>

          <nav>
            <Link href={`/dept/msd/studylink?did=${encodeURIComponent(did)}`}>StudyLink</Link>
            {" · "}
            <Link href={`/dept/msd/case-history?did=${encodeURIComponent(did)}`}>Case History</Link>
          </nav>
        </>
      )}
    </main>
  );
}
