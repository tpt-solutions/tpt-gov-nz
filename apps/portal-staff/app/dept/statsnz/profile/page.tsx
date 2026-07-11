import Link from "next/link";
import { fetchStatsnzDataForCitizen } from "../actions";

export const metadata = { title: "Profile — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function StatsnzStaffProfilePage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Profile — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchStatsnzDataForCitizen(did, ["statsnz:profile"]);
  const profile = data?.profile;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/statsnz?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Profile — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load data profile.</p>}
      {data && !profile && <p>No data profile on file.</p>}

      {profile && (
        <section>
          <p>{profile.dataSummary}</p>
          <ul>
            <li>Records: {profile.recordCount}</li>
            <li>Last updated: {profile.lastUpdated}</li>
          </ul>
        </section>
      )}
    </main>
  );
}
