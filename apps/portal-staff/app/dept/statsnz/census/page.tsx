import Link from "next/link";
import { fetchStatsnzDataForCitizen } from "../actions";

export const metadata = { title: "Census — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function StatsnzStaffCensusPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Census — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchStatsnzDataForCitizen(did, ["statsnz:census"]);
  const census = data?.census;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/statsnz?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Census — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load census records.</p>}
      {data && (!census || census.length === 0) && <p>No census records on file.</p>}

      {census && census.length > 0 && (
        <ul>
          {census.map((c) => (
            <li key={c.censusYear}>
              <strong>{c.censusYear}</strong> — {c.dwellingType} in {c.region}, household of {c.householdSize}
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
