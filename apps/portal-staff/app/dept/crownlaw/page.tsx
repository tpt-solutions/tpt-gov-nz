import Link from "next/link";
import { fetchCrownlawDataForCitizen } from "./actions";

export const metadata = { title: "Crown Law Office — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function CrownlawStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Crown Law Office — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Crown Law records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchCrownlawDataForCitizen(did, ["crownlaw:legal-opinions", "crownlaw:litigation"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Crown Law Office — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Crown Law information for this citizen.</p>}

      {data && (
        <>
          <p>Crown Law ID: ••••{data.crownlawId.slice(-4)}</p>

      <section>
        <h2>Legal opinions</h2>
        {data && data.legal_opinions ? (
          <Link href={`/dept/crownlaw/legal_opinions?did=${encodeURIComponent(did)}`}>View legal opinions</Link>
        ) : (
          <p>No legal opinions on file.</p>
        )}
      </section>

      <section>
        <h2>Litigation</h2>
        {data && data.litigation ? (
          <Link href={`/dept/crownlaw/litigation?did=${encodeURIComponent(did)}`}>View litigation</Link>
        ) : (
          <p>No litigation on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
