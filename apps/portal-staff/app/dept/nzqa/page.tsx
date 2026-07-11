import Link from "next/link";
import { fetchNzqaDataForCitizen } from "./actions";

export const metadata = { title: "Ministry of Education / NZQA — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function NzqaStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Ministry of Education / NZQA — Case File</h1>
        <p>No citizen selected. Enter a DID to view their NZQA records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchNzqaDataForCitizen(did, ["nzqa:qualifications", "nzqa:transcripts"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry of Education / NZQA — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load NZQA information for this citizen.</p>}

      {data && (
        <>
          <p>NSN: ••••{data.nsn.slice(-4)}</p>

          <section>
            <h2>Qualifications</h2>
            {data.qualifications && data.qualifications.length > 0 ? (
              <ul>
                {data.qualifications.map((q) => (
                  <li key={q.qualificationId}>
                    <strong>{q.title}</strong> — Level {q.level} ({q.provider})
                  </li>
                ))}
              </ul>
            ) : (
              <p>No qualifications on record.</p>
            )}
          </section>

          <section>
            <h2>Transcript</h2>
            {data.transcript ? (
              <ul>
                {data.transcript.recordSummary && (
                  <li>
                    <strong>Summary:</strong> {data.transcript.recordSummary}
                  </li>
                )}
                {data.transcript.totalCredits != null && (
                  <li>
                    <strong>Total credits:</strong> {data.transcript.totalCredits}
                  </li>
                )}
                {data.transcript.creditSummary && (
                  <li>
                    <strong>Credit summary:</strong> {data.transcript.creditSummary}
                  </li>
                )}
              </ul>
            ) : (
              <p>No transcript on record.</p>
            )}
          </section>

          <nav>
            <Link href={`/dept/nzqa/qualifications?did=${encodeURIComponent(did)}`}>Qualifications</Link>
            {" · "}
            <Link href={`/dept/nzqa/transcript?did=${encodeURIComponent(did)}`}>Transcript</Link>
          </nav>
        </>
      )}
    </main>
  );
}
