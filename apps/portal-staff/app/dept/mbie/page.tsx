import Link from "next/link";
import { fetchMbieDataForCitizen } from "./actions";

export const metadata = { title: "Business — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MbieStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Business — Case File</h1>
        <p>No citizen selected. Enter a DID to view their business records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchMbieDataForCitizen(did, ["mbie:business", "mbie:directorships"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Ministry of Business, Innovation and Employment — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load business information for this citizen.</p>}

      {data && (
        <>
          <p>Person id: ••••{data.personId.slice(-4)}</p>

          <section>
            <h2>Business registrations</h2>
            {data.businessRegistrations && data.businessRegistrations.length > 0 ? (
              <ul>
                {data.businessRegistrations.map((b) => (
                  <li key={b.nzbn}>
                    <strong>{b.entityName}</strong> — {b.entityType} ({b.status})
                  </li>
                ))}
              </ul>
            ) : (
              <p>No business registrations on record.</p>
            )}
          </section>

          <section>
            <h2>Directorships</h2>
            {data.directorships && data.directorships.length > 0 ? (
              <ul>
                {data.directorships.map((d) => (
                  <li key={`${d.nzbn}-${d.appointedDate}`}>
                    <strong>{d.entityName}</strong> — {d.role}
                  </li>
                ))}
              </ul>
            ) : (
              <p>No directorships on record.</p>
            )}
          </section>

          <nav>
            <Link href={`/dept/mbie/business?did=${encodeURIComponent(did)}`}>Business registrations</Link>
            {" · "}
            <Link href={`/dept/mbie/directorships?did=${encodeURIComponent(did)}`}>Directorships</Link>
          </nav>
        </>
      )}
    </main>
  );
}
