import Link from "next/link";
import { fetchMojDataForCitizen } from "./actions";

export const metadata = { title: "Ministry of Justice — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MojStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Ministry of Justice — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Ministry of Justice records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchMojDataForCitizen(did, [
    "moj:fines",
    "moj:disputes",
    "moj:court-records",
  ]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry of Justice — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Ministry of Justice information for this citizen.</p>}

      {data && (
        <>
          <p>Client number: ••••{data.clientNumber.slice(-4)}</p>

          <section>
            <h2>Fines</h2>
            {data.fines && data.fines.length > 0 ? (
              <ul>
                {data.fines.map((f) => (
                  <li key={f.fineNumber}>
                    <strong>{f.fineNumber}</strong> — {f.fineType} ({f.status}): ${f.amount}
                  </li>
                ))}
              </ul>
            ) : (
              <p>No fines on record.</p>
            )}
          </section>

          <section>
            <h2>Disputes Tribunal</h2>
            {data.disputes && data.disputes.length > 0 ? (
              <ul>
                {data.disputes.map((d) => (
                  <li key={d.disputeNumber}>
                    <strong>{d.disputeNumber}</strong> — {d.claimType} ({d.status}): {d.description}
                  </li>
                ))}
              </ul>
            ) : (
              <p>No Disputes Tribunal claims on record.</p>
            )}
          </section>

          <section>
            <h2>Court records</h2>
            {data.courtRecords && data.courtRecords.length > 0 ? (
              <ul>
                {data.courtRecords.map((c) => (
                  <li key={c.caseNumber}>
                    <strong>{c.caseNumber}</strong> — {c.caseType} ({c.status}): {c.description}
                  </li>
                ))}
              </ul>
            ) : (
              <p>No court records on file.</p>
            )}
          </section>

          <nav>
            <Link href={`/dept/moj/fines?did=${encodeURIComponent(did)}`}>Fines</Link>
            {" · "}
            <Link href={`/dept/moj/disputes?did=${encodeURIComponent(did)}`}>Disputes</Link>
            {" · "}
            <Link href={`/dept/moj/court-records?did=${encodeURIComponent(did)}`}>Court records</Link>
          </nav>
        </>
      )}
    </main>
  );
}
