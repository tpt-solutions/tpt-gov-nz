import Link from "next/link";
import { fetchRegulationDataForCitizen } from "./actions";

export const metadata = { title: "Ministry for Regulation — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function RegulationStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Ministry for Regulation — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Regulation records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchRegulationDataForCitizen(did, ["regulation:regulatory-reviews", "regulation:proposals"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry for Regulation — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Regulation information for this citizen.</p>}

      {data && (
        <>
          <p>Regulation ID: ••••{data.regulationId.slice(-4)}</p>

      <section>
        <h2>Regulatory reviews</h2>
        {data && data.regulatory_reviews ? (
          <Link href={`/dept/regulation/regulatory_reviews?did=${encodeURIComponent(did)}`}>View regulatory reviews</Link>
        ) : (
          <p>No regulatory reviews on file.</p>
        )}
      </section>

      <section>
        <h2>Proposals</h2>
        {data && data.proposals ? (
          <Link href={`/dept/regulation/proposals?did=${encodeURIComponent(did)}`}>View proposals</Link>
        ) : (
          <p>No proposals on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
