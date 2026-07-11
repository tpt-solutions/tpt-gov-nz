import Link from "next/link";
import { fetchMotDataForCitizen } from "./actions";

export const metadata = { title: "Ministry of Transport — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MotStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Ministry of Transport — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Transport records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchMotDataForCitizen(did, ["mot:strategies", "mot:programmes"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry of Transport — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Transport information for this citizen.</p>}

      {data && (
        <>
          <p>Transport ID: ••••{data.motId.slice(-4)}</p>

      <section>
        <h2>Strategies</h2>
        {data && data.strategies ? (
          <Link href={`/dept/mot/strategies?did=${encodeURIComponent(did)}`}>View strategies</Link>
        ) : (
          <p>No strategies on file.</p>
        )}
      </section>

      <section>
        <h2>Programmes</h2>
        {data && data.programmes ? (
          <Link href={`/dept/mot/programmes?did=${encodeURIComponent(did)}`}>View programmes</Link>
        ) : (
          <p>No programmes on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
