import Link from "next/link";
import { fetchOrangaData } from "../actions";

export const metadata = { title: "Care placements — Oranga Tamariki — My Gov NZ" };

export default async function OrangaCarePlacementsPage() {
  const data = await fetchOrangaData(["oranga:care-placements"]);
  if (!data) {
    return (
      <main>
        <h1>Care placements</h1>
        <p>Unable to load your Oranga Tamariki information.</p>
        <Link href={"/consent?dept=oranga"}>Grant Oranga Tamariki access</Link>
      </main>
    );
  }

  const rows = data.care_placements ?? [];

  return (
    <main>
      <Link href={"/dept/oranga"}>← Back to Oranga Tamariki</Link>
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
