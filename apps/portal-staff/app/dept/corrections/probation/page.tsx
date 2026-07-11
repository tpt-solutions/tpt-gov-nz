import Link from "next/link";
import { fetchCorrectionsDataForCitizen } from "../actions";

export const metadata = { title: "Probation — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function CorrectionsStaffProbationPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Probation — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchCorrectionsDataForCitizen(did, ["corrections:probation"]);
  const probation = data?.probation;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/corrections?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Probation — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load probation.</p>}
      {data && !probation && <p>No probation record on file.</p>}

      {probation && (
        <ul>
          <li>
            <strong>{probation.status}</strong> — Officer {probation.officerName} ({probation.location})
            <br />
            <small>Next report {probation.nextReportDate}</small>
          </li>
        </ul>
      )}
    </main>
  );
}
