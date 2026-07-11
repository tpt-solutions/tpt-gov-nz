import Link from "next/link";
import { fetchTpkDataForCitizen } from "./actions";

export const metadata = { title: "Te Puni Kōkiri — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function TpkStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Te Puni Kōkiri — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Te Puni Kōkiri records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchTpkDataForCitizen(did, ["tpk:programmes", "tpk:funding"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Te Puni Kōkiri — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Te Puni Kōkiri information for this citizen.</p>}

      {data && (
        <>
          <p>TPK id: ••••{data.tpkId.slice(-4)}</p>

          <section>
            <h2>Programmes</h2>
            {data.programmes && data.programmes.length > 0 ? (
              <ul>
                {data.programmes.map((p) => (
                  <li key={p.programmeName}>
                    <strong>{p.programmeName}</strong> — {p.status} ({p.region})
                  </li>
                ))}
              </ul>
            ) : (
              <p>No Te Puni Kōkiri programmes on record.</p>
            )}
          </section>

          <section>
            <h2>Funding</h2>
            {data.funding && data.funding.length > 0 ? (
              <ul>
                {data.funding.map((f) => (
                  <li key={f.grantId}>
                    <strong>{f.grantId}</strong> — ${f.amount} ({f.status}): {f.purpose}
                  </li>
                ))}
              </ul>
            ) : (
              <p>No Te Puni Kōkiri funding on record.</p>
            )}
          </section>

          <nav>
            <Link href={`/dept/tpk/programmes?did=${encodeURIComponent(did)}`}>Programmes</Link>
            {" · "}
            <Link href={`/dept/tpk/funding?did=${encodeURIComponent(did)}`}>Funding</Link>
          </nav>
        </>
      )}
    </main>
  );
}
