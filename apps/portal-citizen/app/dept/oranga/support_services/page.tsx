import Link from "next/link";
import { fetchOrangaData } from "../actions";

export const metadata = { title: "Support services — Oranga Tamariki — My Gov NZ" };

export default async function OrangaSupportServicesPage() {
  const data = await fetchOrangaData(["oranga:support-services"]);
  if (!data) {
    return (
      <main>
        <h1>Support services</h1>
        <p>Unable to load your Oranga Tamariki information.</p>
        <Link href={"/consent?dept=oranga"}>Grant Oranga Tamariki access</Link>
      </main>
    );
  }

  const rows = data.support_services ?? [];

  return (
    <main>
      <Link href={"/dept/oranga"}>← Back to Oranga Tamariki</Link>
      <h1>Support services</h1>
      {rows.length === 0 ? (
        <p>No support services on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>service</th>
                <th>status</th>
                <th>nextReview</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.service}</td>
                  <td>{row.status}</td>
                  <td>{row.nextReview}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
