import Link from "next/link";
import { fetchOrangaDataForCitizen } from "../actions";

export const metadata = { title: "Care placements — Oranga Tamariki — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function OrangaCarePlacementsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchOrangaDataForCitizen(did, ["oranga:care-placements"]);
  const rows = data?.care_placements ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/oranga?did=${encodeURIComponent(did)}`}>← Back to Oranga Tamariki case file</Link>
      <h1>Care placements</h1>
      {rows.length === 0 ? (
        <p>No care placements on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>placementType</th>
                <th>startDate</th>
                <th>region</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.placementType}</td>
                  <td>{row.startDate}</td>
                  <td>{row.region}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
