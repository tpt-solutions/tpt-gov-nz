import Link from "next/link";
import { fetchDocDataForCitizen } from "./actions";

export const metadata = { title: "Department of Conservation — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function DocStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Department of Conservation — Case File</h1>
        <p>No citizen selected. Enter a DID to view their conservation records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchDocDataForCitizen(did, ["doc:permits", "doc:concessions"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Department of Conservation — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load conservation information for this citizen.</p>}

      {data && (
        <>
          <p>Doc id: ••••{data.docId.slice(-4)}</p>

          <section>
            <h2>Permits</h2>
            {data.permits && data.permits.length > 0 ? (
              <ul>
                {data.permits.map((p) => (
                  <li key={p.permitNumber}>
                    <strong>{p.permitNumber}</strong> — {p.activity} ({p.status}) at {p.location}
                  </li>
                ))}
              </ul>
            ) : (
              <p>No conservation permits on record.</p>
            )}
          </section>

          <section>
            <h2>Concessions</h2>
            {data.concessions && data.concessions.length > 0 ? (
              <ul>
                {data.concessions.map((c) => (
                  <li key={c.concessionId}>
                    <strong>{c.concessionId}</strong> — {c.type} ({c.holder}): {c.startDate} to {c.endDate}
                  </li>
                ))}
              </ul>
            ) : (
              <p>No conservation concessions on record.</p>
            )}
          </section>

          <nav>
            <Link href={`/dept/doc/permits?did=${encodeURIComponent(did)}`}>Permits</Link>
            {" · "}
            <Link href={`/dept/doc/concessions?did=${encodeURIComponent(did)}`}>Concessions</Link>
          </nav>
        </>
      )}
    </main>
  );
}
