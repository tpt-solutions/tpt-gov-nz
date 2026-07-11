import Link from "next/link";
import { fetchCorrectionsDataForCitizen } from "./actions";

export const metadata = { title: "Department of Corrections — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function CorrectionsStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Department of Corrections — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Corrections records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchCorrectionsDataForCitizen(did, ["corrections:probation", "corrections:case"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Department of Corrections — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Corrections information for this citizen.</p>}

      {data && (
        <>
          <p>Corrections ID: ••••{data.correctionsId.slice(-4)}</p>

          <section>
            <h2>Probation</h2>
            {data.probation ? (
              <ul>
                <li>
                  <strong>{data.probation.status}</strong> — Officer {data.probation.officerName} ({data.probation.location}), next report {data.probation.nextReportDate}
                </li>
              </ul>
            ) : (
              <p>No probation record on file.</p>
            )}
          </section>

          <section>
            <h2>Case</h2>
            {data.case && data.case.length > 0 ? (
              <ul>
                {data.case.map((c) => (
                  <li key={c.caseNumber}>
                    <strong>{c.caseNumber}</strong> — {c.sentenceType}
                    <br />
                    <small>{c.startDate}{c.endDate ? ` – ${c.endDate}` : " – ongoing"} · {c.summary}</small>
                  </li>
                ))}
              </ul>
            ) : (
              <p>No cases on record.</p>
            )}
          </section>

          <nav>
            <Link href={`/dept/corrections/probation?did=${encodeURIComponent(did)}`}>Probation</Link>
            {" · "}
            <Link href={`/dept/corrections/case?did=${encodeURIComponent(did)}`}>Case</Link>
          </nav>
        </>
      )}
    </main>
  );
}
