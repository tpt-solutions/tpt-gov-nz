import Link from "next/link";
import { fetchPoliceDataForCitizen } from "./actions";

export const metadata = { title: "Police — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function PoliceStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>New Zealand Police — Case File</h1>
        <p>No citizen selected. Enter a DID to view their Police records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchPoliceDataForCitizen(did, ["police:infringements", "police:reports"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>New Zealand Police — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load Police information for this citizen.</p>}

      {data && (
        <>
          <p>Client number: ••••{data.clientNumber.slice(-4)}</p>

          <section>
            <h2>Infringements</h2>
            {data.infringements && data.infringements.length > 0 ? (
              <ul>
                {data.infringements.map((i) => (
                  <li key={i.ticketNumber}>
                    <strong>{i.ticketNumber}</strong> — {i.offenseType} ({i.status}): ${i.amount}
                  </li>
                ))}
              </ul>
            ) : (
              <p>No infringements on record.</p>
            )}
          </section>

          <section>
            <h2>Reports</h2>
            {data.reports && data.reports.length > 0 ? (
              <ul>
                {data.reports.map((r) => (
                  <li key={r.reportNumber}>
                    <strong>{r.reportNumber}</strong> — {r.reportType} ({r.status}): {r.description}
                  </li>
                ))}
              </ul>
            ) : (
              <p>No reports on record.</p>
            )}
          </section>

          <nav>
            <Link href={`/dept/police/infringements?did=${encodeURIComponent(did)}`}>Infringements</Link>
            {" · "}
            <Link href={`/dept/police/reports?did=${encodeURIComponent(did)}`}>Reports</Link>
          </nav>
        </>
      )}
    </main>
  );
}
