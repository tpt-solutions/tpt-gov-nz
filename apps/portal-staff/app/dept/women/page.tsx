import Link from "next/link";
import { fetchWomenDataForCitizen } from "./actions";

export const metadata = { title: "Ministry for Women — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function WomenStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Ministry for Women — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Women records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchWomenDataForCitizen(did, ["women:programmes", "women:insights"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry for Women — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Women information for this citizen.</p>}

      {data && (
        <>
          <p>Women ID: ••••{data.womenId.slice(-4)}</p>

      <section>
        <h2>Programmes</h2>
        {data && data.programmes ? (
          <Link href={`/dept/women/programmes?did=${encodeURIComponent(did)}`}>View programmes</Link>
        ) : (
          <p>No programmes on file.</p>
        )}
      </section>

      <section>
        <h2>Insights</h2>
        {data && data.insights ? (
          <Link href={`/dept/women/insights?did=${encodeURIComponent(did)}`}>View insights</Link>
        ) : (
          <p>No insights on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
