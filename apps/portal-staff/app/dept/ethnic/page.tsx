import Link from "next/link";
import { fetchEthnicDataForCitizen } from "./actions";

export const metadata = { title: "Ministry for Ethnic Communities — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function EthnicStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Ministry for Ethnic Communities — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Ethnic Communities records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchEthnicDataForCitizen(did, ["ethnic:programmes", "ethnic:community-grants"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry for Ethnic Communities — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Ethnic Communities information for this citizen.</p>}

      {data && (
        <>
          <p>Ethnic Communities ID: ••••{data.ethnicId.slice(-4)}</p>

      <section>
        <h2>Programmes</h2>
        {data && data.programmes ? (
          <Link href={`/dept/ethnic/programmes?did=${encodeURIComponent(did)}`}>View programmes</Link>
        ) : (
          <p>No programmes on file.</p>
        )}
      </section>

      <section>
        <h2>Community grants</h2>
        {data && data.community_grants ? (
          <Link href={`/dept/ethnic/community_grants?did=${encodeURIComponent(did)}`}>View community grants</Link>
        ) : (
          <p>No community grants on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
