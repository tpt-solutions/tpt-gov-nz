import Link from "next/link";
import { fetchPacificData } from "../actions";

export const metadata = { title: "Programmes — Ministry for Pacific Peoples — My Gov NZ" };

export default async function PacificProgrammesPage() {
  const data = await fetchPacificData(["pacific:programmes"]);
  if (!data) {
    return (
      <main>
        <h1>Programmes</h1>
        <p>Unable to load your Pacific Peoples information.</p>
        <Link href={"/consent?dept=pacific"}>Grant Pacific Peoples access</Link>
      </main>
    );
  }

  const rows = data.programmes ?? [];

  return (
    <main>
      <Link href={"/dept/pacific"}>← Back to Pacific Peoples</Link>
      <h1>Programmes</h1>
      {rows.length === 0 ? (
        <p>No programmes on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>programmeName</th>
                <th>status</th>
                <th>year</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.programmeName}</td>
                  <td>{row.status}</td>
                  <td>{row.year}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
