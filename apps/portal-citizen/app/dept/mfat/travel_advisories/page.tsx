import Link from "next/link";
import { fetchMfatData } from "../actions";

export const metadata = { title: "Travel advisories — Ministry of Foreign Affairs and Trade — My Gov NZ" };

export default async function MfatTravelAdvisoriesPage() {
  const data = await fetchMfatData(["mfat:travel-advisories"]);
  if (!data) {
    return (
      <main>
        <h1>Travel advisories</h1>
        <p>Unable to load your MFAT information.</p>
        <Link href={"/consent?dept=mfat"}>Grant MFAT access</Link>
      </main>
    );
  }

  const rows = data.travel_advisories ?? [];

  return (
    <main>
      <Link href={"/dept/mfat"}>← Back to MFAT</Link>
      <h1>Travel advisories</h1>
      {rows.length === 0 ? (
        <p>No travel advisories on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>country</th>
                <th>level</th>
                <th>updated</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.country}</td>
                  <td>{row.level}</td>
                  <td>{row.updated}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
