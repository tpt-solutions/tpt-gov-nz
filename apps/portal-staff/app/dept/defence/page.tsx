import Link from "next/link";
import { fetchDefenceDataForCitizen } from "./actions";

export const metadata = { title: "Ministry of Defence — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function DefenceStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Ministry of Defence — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Defence records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchDefenceDataForCitizen(did, ["defence:procurements", "defence:bases"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry of Defence — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Defence information for this citizen.</p>}

      {data && (
        <>
          <p>Defence ID: ••••{data.defenceId.slice(-4)}</p>

      <section>
        <h2>Procurements</h2>
        {data && data.procurements ? (
          <Link href={`/dept/defence/procurements?did=${encodeURIComponent(did)}`}>View procurements</Link>
        ) : (
          <p>No procurements on file.</p>
        )}
      </section>

      <section>
        <h2>Bases</h2>
        {data && data.bases ? (
          <Link href={`/dept/defence/bases?did=${encodeURIComponent(did)}`}>View bases</Link>
        ) : (
          <p>No bases on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
