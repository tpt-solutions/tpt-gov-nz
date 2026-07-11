import Link from "next/link";
import { fetchStatsnzData } from "../actions";

export const metadata = { title: "Stats NZ Census — My Gov NZ" };

export default async function StatsnzCensusPage() {
  const data = await fetchStatsnzData(["statsnz:census"]);
  const census = data?.census ?? [];

  return (
    <main>
      <Link href="/dept/statsnz">← Back to Stats NZ</Link>
      <h1>Stats NZ Census</h1>

      <section>
        <h2>Your census records</h2>
        {census.length === 0 ? (
          <p>No census records on file.</p>
        ) : (
          <ul>
            {census.map((c) => (
              <li key={c.censusYear}>
                <strong>{c.censusYear}</strong> — {c.dwellingType} in {c.region}
                <br />
                <small>Household of {c.householdSize}</small>
              </li>
            ))}
          </ul>
        )}
      </section>

      <section>
        <h2>Request a data export</h2>
        <Link href="/dept/statsnz/census/request-export">
          Request a data export of your census data →
        </Link>
      </section>
    </main>
  );
}
