import Link from "next/link";
import { fetchPoliceDataForCitizen } from "../actions";

export const metadata = { title: "Reports — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function PoliceStaffReportsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Reports — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchPoliceDataForCitizen(did, ["police:reports"]);
  const reports = data?.reports;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/police?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Reports — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load reports.</p>}
      {data && (!reports || reports.length === 0) && <p>No reports on record.</p>}

      {reports && reports.length > 0 && (
        <ul>
          {reports.map((r) => (
            <li key={r.reportNumber}>
              <strong>{r.reportNumber}</strong> — {r.reportType} ({r.status}): {r.description}
              <br />
              <small>Filed {r.filedDate}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
