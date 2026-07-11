import Link from "next/link";
import { fetchStatsnzDataForCitizen } from "./actions";

export const metadata = { title: "Statistics New Zealand — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function StatsnzStaffOverviewPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Statistics New Zealand — Case File</h1>
        <p>No citizen selected. Enter a DID to view their statistics records.</p>
        <Link href="/">← Back to case worker home</Link>
      </main>
    );
  }

  const data = await fetchStatsnzDataForCitizen(did, ["statsnz:census", "statsnz:profile"]);

  return (
    <main style={{ padding: "1rem" }}>
      <Link href="/">← Back to case worker home</Link>
      <h1>Statistics New Zealand — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>
      <p>
        <em>Read-only view. No actions can be taken from this screen.</em>
      </p>

      {!data && <p>Unable to load statistics information for this citizen.</p>}

      {data && (
        <>
          <p>Stats ID: ••••{data.statsId.slice(-4)}</p>

          <section>
            <h2>Census</h2>
            {data.census && data.census.length > 0 ? (
              <ul>
                {data.census.map((c) => (
                  <li key={c.censusYear}>
                    <strong>{c.censusYear}</strong> — {c.dwellingType} in {c.region}, household of {c.householdSize}
                  </li>
                ))}
              </ul>
            ) : (
              <p>No census records on file.</p>
            )}
          </section>

          <section>
            <h2>Profile</h2>
            {data.profile ? (
              <p>
                {data.profile.dataSummary} {data.profile.recordCount} records, last updated {data.profile.lastUpdated}.
              </p>
            ) : (
              <p>No data profile on file.</p>
            )}
          </section>

          <nav>
            <Link href={`/dept/statsnz/census?did=${encodeURIComponent(did)}`}>Census</Link>
            {" · "}
            <Link href={`/dept/statsnz/profile?did=${encodeURIComponent(did)}`}>Profile</Link>
          </nav>
        </>
      )}
    </main>
  );
}
