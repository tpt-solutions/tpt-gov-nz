import Link from "next/link";
import { fetchMaritimeDataForCitizen } from "./actions";

export const metadata = { title: "Maritime New Zealand — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MaritimeStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Maritime New Zealand — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Maritime records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchMaritimeDataForCitizen(did, ["maritime:vessels", "maritime:incidents"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Maritime New Zealand — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Maritime information for this citizen.</p>}

      {data && (
        <>
          <p>Maritime ID: ••••{data.maritimeId.slice(-4)}</p>

      <section>
        <h2>Vessels</h2>
        {data && data.vessels ? (
          <Link href={`/dept/maritime/vessels?did=${encodeURIComponent(did)}`}>View vessels</Link>
        ) : (
          <p>No vessels on file.</p>
        )}
      </section>

      <section>
        <h2>Incidents</h2>
        {data && data.incidents ? (
          <Link href={`/dept/maritime/incidents?did=${encodeURIComponent(did)}`}>View incidents</Link>
        ) : (
          <p>No incidents on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
