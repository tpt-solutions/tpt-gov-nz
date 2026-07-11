import Link from "next/link";
import { fetchMojDataForCitizen } from "../actions";

export const metadata = { title: "Disputes — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MojStaffDisputesPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Disputes Tribunal — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchMojDataForCitizen(did, ["moj:disputes"]);
  const disputes = data?.disputes;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/moj?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Disputes Tribunal — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load disputes.</p>}
      {data && (!disputes || disputes.length === 0) && <p>No Disputes Tribunal claims on record.</p>}

      {disputes && disputes.length > 0 && (
        <ul>
          {disputes.map((d) => (
            <li key={d.disputeNumber}>
              <strong>{d.disputeNumber}</strong> — {d.claimType} ({d.status}): {d.description}
              {d.hearingDate ? <><br /><small>Hearing date: {d.hearingDate}</small></> : null}
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
