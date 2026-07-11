import Link from "next/link";
import { fetchOrangaDataForCitizen } from "./actions";

export const metadata = { title: "Oranga Tamariki — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function OrangaStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Oranga Tamariki — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Oranga Tamariki records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchOrangaDataForCitizen(did, ["oranga:care-placements", "oranga:support-services"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Oranga Tamariki — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Oranga Tamariki information for this citizen.</p>}

      {data && (
        <>
          <p>Oranga Tamariki ID: ••••{data.orangaId.slice(-4)}</p>

      <section>
        <h2>Care placements</h2>
        {data && data.care_placements ? (
          <Link href={`/dept/oranga/care_placements?did=${encodeURIComponent(did)}`}>View care placements</Link>
        ) : (
          <p>No care placements on file.</p>
        )}
      </section>

      <section>
        <h2>Support services</h2>
        {data && data.support_services ? (
          <Link href={`/dept/oranga/support_services?did=${encodeURIComponent(did)}`}>View support services</Link>
        ) : (
          <p>No support services on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
