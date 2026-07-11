import Link from "next/link";
import { fetchNzsisDataForCitizen } from "./actions";

export const metadata = { title: "New Zealand Security Intelligence Service — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function NzsisStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>New Zealand Security Intelligence Service — Case File</h1>
        <p>No citizen selected. Enter a DID to view their NZSIS records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchNzsisDataForCitizen(did, ["nzsis:mandates", "nzsis:threats"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>New Zealand Security Intelligence Service — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load NZSIS information for this citizen.</p>}

      {data && (
        <>
          <p>NZSIS ID: ••••{data.nzsisId.slice(-4)}</p>

      <section>
        <h2>Mandates</h2>
        {data && data.mandates ? (
          <Link href={`/dept/nzsis/mandates?did=${encodeURIComponent(did)}`}>View mandates</Link>
        ) : (
          <p>No mandates on file.</p>
        )}
      </section>

      <section>
        <h2>Threats</h2>
        {data && data.threats ? (
          <Link href={`/dept/nzsis/threats?did=${encodeURIComponent(did)}`}>View threats</Link>
        ) : (
          <p>No threats on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
