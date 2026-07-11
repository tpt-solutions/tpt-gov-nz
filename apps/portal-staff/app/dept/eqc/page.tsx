import Link from "next/link";
import { fetchEqcDataForCitizen } from "./actions";

export const metadata = { title: "Earthquake Commission (Toka Tū Ake) — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function EqcStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Earthquake Commission (Toka Tū Ake) — Case File</h1>
        <p>No citizen selected. Enter a DID to view their EQC records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchEqcDataForCitizen(did, ["eqc:claims", "eqc:cover"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Earthquake Commission (Toka Tū Ake) — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load EQC information for this citizen.</p>}

      {data && (
        <>
          <p>EQC ID: ••••{data.eqcId.slice(-4)}</p>

      <section>
        <h2>Claims</h2>
        {data && data.claims ? (
          <Link href={`/dept/eqc/claims?did=${encodeURIComponent(did)}`}>View claims</Link>
        ) : (
          <p>No claims on file.</p>
        )}
      </section>

      <section>
        <h2>Cover</h2>
        {data && data.cover ? (
          <Link href={`/dept/eqc/cover?did=${encodeURIComponent(did)}`}>View cover</Link>
        ) : (
          <p>No cover on file.</p>
        )}
      </section>
        </>
      )}
    </main>
  );
}
