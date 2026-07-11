import Link from "next/link";
import { fetchMchDataForCitizen } from "./actions";

export const metadata = { title: "Ministry for Culture and Heritage — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MchStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Ministry for Culture and Heritage — Case File</h1>
        <p>No citizen selected. Enter a DID to view their MCH records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchMchDataForCitizen(did, ["mch:heritage-sites", "mch:grants"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry for Culture and Heritage — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load MCH information for this citizen.</p>}

      {data && (
        <>
          <p>MCH ID: ••••{data.mchId.slice(-4)}</p>

      <section>
        <h2>Heritage sites</h2>
        {data && data.heritage_sites ? (
          <Link href={`/dept/mch/heritage_sites?did=${encodeURIComponent(did)}`}>View heritage sites</Link>
        ) : (
          <p>No heritage sites on file.</p>
        )}
      </section>

      <section>
        <h2>Grants</h2>
        {data && data.grants ? (
          <Link href={`/dept/mch/grants?did=${encodeURIComponent(did)}`}>View grants</Link>
        ) : (
          <p>No grants on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
