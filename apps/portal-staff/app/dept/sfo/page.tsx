import Link from "next/link";
import { fetchSfoDataForCitizen } from "./actions";

export const metadata = { title: "Serious Fraud Office — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function SfoStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Serious Fraud Office — Case File</h1>
        <p>No citizen selected. Enter a DID to view their SFO records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchSfoDataForCitizen(did, ["sfo:investigations", "sfo:outcomes"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Serious Fraud Office — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load SFO information for this citizen.</p>}

      {data && (
        <>
          <p>SFO ID: ••••{data.sfoId.slice(-4)}</p>

      <section>
        <h2>Investigations</h2>
        {data && data.investigations ? (
          <Link href={`/dept/sfo/investigations?did=${encodeURIComponent(did)}`}>View investigations</Link>
        ) : (
          <p>No investigations on file.</p>
        )}
      </section>

      <section>
        <h2>Outcomes</h2>
        {data && data.outcomes ? (
          <Link href={`/dept/sfo/outcomes?did=${encodeURIComponent(did)}`}>View outcomes</Link>
        ) : (
          <p>No outcomes on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
