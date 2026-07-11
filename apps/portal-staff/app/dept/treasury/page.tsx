import Link from "next/link";
import { fetchTreasuryDataForCitizen } from "./actions";

export const metadata = { title: "The Treasury — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function TreasuryStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>The Treasury — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Treasury records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchTreasuryDataForCitizen(did, ["treasury:budget", "treasury:economic-outlook"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>The Treasury — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Treasury information for this citizen.</p>}

      {data && (
        <>
          <p>Treasury ID: ••••{data.treasuryId.slice(-4)}</p>

      <section>
        <h2>Budget</h2>
        {data && data.budget ? (
          <Link href={`/dept/treasury/budget?did=${encodeURIComponent(did)}`}>View budget</Link>
        ) : (
          <p>No budget on file.</p>
        )}
      </section>

      <section>
        <h2>Economic outlook</h2>
        {data && data.economic_outlook ? (
          <Link href={`/dept/treasury/economic_outlook?did=${encodeURIComponent(did)}`}>View economic outlook</Link>
        ) : (
          <p>No economic outlook on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
