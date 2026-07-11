import Link from "next/link";
import { fetchPacificData } from "../actions";

export const metadata = { title: "Language services — Ministry for Pacific Peoples — My Gov NZ" };

export default async function PacificLanguageServicesPage() {
  const data = await fetchPacificData(["pacific:language-services"]);
  if (!data) {
    return (
      <main>
        <h1>Language services</h1>
        <p>Unable to load your Pacific Peoples information.</p>
        <Link href={"/consent?dept=pacific"}>Grant Pacific Peoples access</Link>
      </main>
    );
  }

  const rows = data.language_services ?? [];

  return (
    <main>
      <Link href={"/dept/pacific"}>← Back to Pacific Peoples</Link>
      <h1>Language services</h1>
      {rows.length === 0 ? (
        <p>No language services on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>service</th>
                <th>region</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.service}</td>
                  <td>{row.region}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
