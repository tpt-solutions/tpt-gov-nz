import Link from "next/link";
import { fetchFenzDataForCitizen } from "./actions";

export const metadata = { title: "Fire and Emergency New Zealand — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function FenzStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Fire and Emergency New Zealand — Case File</h1>
        <p>No citizen selected. Enter a DID to view their FENZ records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchFenzDataForCitizen(did, ["fenz:fire-safety", "fenz:incidents"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Fire and Emergency New Zealand — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load FENZ information for this citizen.</p>}

      {data && (
        <>
          <p>FENZ ID: ••••{data.fenzId.slice(-4)}</p>

      <section>
        <h2>Fire safety</h2>
        {data && data.fire_safety ? (
          <Link href={`/dept/fenz/fire_safety?did=${encodeURIComponent(did)}`}>View fire safety</Link>
        ) : (
          <p>No fire safety on file.</p>
        )}
      </section>

      <section>
        <h2>Incidents</h2>
        {data && data.incidents ? (
          <Link href={`/dept/fenz/incidents?did=${encodeURIComponent(did)}`}>View incidents</Link>
        ) : (
          <p>No incidents on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
