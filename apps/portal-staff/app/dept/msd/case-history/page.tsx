import Link from "next/link";
import { fetchMsdDataForCitizen } from "../actions";

export const metadata = { title: "Case History — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MsdStaffCaseHistoryPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Case History — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchMsdDataForCitizen(did, ["msd:case-history"]);
  const events = data?.caseHistory;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/msd?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Case History — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load case history.</p>}
      {data && (!events || events.length === 0) && <p>No case history on record.</p>}

      {events && events.length > 0 && (
        <ul>
          {events.map((e, i) => (
            <li key={`${e.eventDate}-${i}`}>
              <strong>{e.eventDate}</strong> — {e.serviceLine}: {e.summary}
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
