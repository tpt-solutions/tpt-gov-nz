import Link from "next/link";
import { fetchCaaDataForCitizen } from "./actions";

export const metadata = { title: "Civil Aviation Authority — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function CaaStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Civil Aviation Authority — Case File</h1>
        <p>No citizen selected. Enter a DID to view their CAA records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchCaaDataForCitizen(did, ["caa:licences", "caa:aircraft"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Civil Aviation Authority — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load CAA information for this citizen.</p>}

      {data && (
        <>
          <p>CAA ID: ••••{data.caaId.slice(-4)}</p>

      <section>
        <h2>Licences</h2>
        {data && data.licences ? (
          <Link href={`/dept/caa/licences?did=${encodeURIComponent(did)}`}>View licences</Link>
        ) : (
          <p>No licences on file.</p>
        )}
      </section>

      <section>
        <h2>Aircraft</h2>
        {data && data.aircraft ? (
          <Link href={`/dept/caa/aircraft?did=${encodeURIComponent(did)}`}>View aircraft</Link>
        ) : (
          <p>No aircraft on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
