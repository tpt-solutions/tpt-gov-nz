import Link from "next/link";
import { fetchGcsbDataForCitizen } from "./actions";

export const metadata = { title: "Government Communications Security Bureau — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function GcsbStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Government Communications Security Bureau — Case File</h1>
        <p>No citizen selected. Enter a DID to view their GCSB records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchGcsbDataForCitizen(did, ["gcsb:mandates", "gcsb:engagements"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Government Communications Security Bureau — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load GCSB information for this citizen.</p>}

      {data && (
        <>
          <p>GCSB ID: ••••{data.gcsbId.slice(-4)}</p>

      <section>
        <h2>Mandates</h2>
        {data && data.mandates ? (
          <Link href={`/dept/gcsb/mandates?did=${encodeURIComponent(did)}`}>View mandates</Link>
        ) : (
          <p>No mandates on file.</p>
        )}
      </section>

      <section>
        <h2>Engagements</h2>
        {data && data.engagements ? (
          <Link href={`/dept/gcsb/engagements?did=${encodeURIComponent(did)}`}>View engagements</Link>
        ) : (
          <p>No engagements on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
