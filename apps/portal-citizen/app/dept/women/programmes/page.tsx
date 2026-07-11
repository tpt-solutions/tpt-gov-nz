import Link from "next/link";
import { fetchWomenData } from "../actions";

export const metadata = { title: "Programmes — Ministry for Women — My Gov NZ" };

export default async function WomenProgrammesPage() {
  const data = await fetchWomenData(["women:programmes"]);
  if (!data) {
    return (
      <main>
        <h1>Programmes</h1>
        <p>Unable to load your Women information.</p>
        <Link href={"/consent?dept=women"}>Grant Women access</Link>
      </main>
    );
  }

  const rows = data.programmes ?? [];

  return (
    <main>
      <Link href={"/dept/women"}>← Back to Women</Link>
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
