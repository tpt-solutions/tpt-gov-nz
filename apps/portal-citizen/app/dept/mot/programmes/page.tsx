import Link from "next/link";
import { fetchMotData } from "../actions";

export const metadata = { title: "Programmes — Ministry of Transport — My Gov NZ" };

export default async function MotProgrammesPage() {
  const data = await fetchMotData(["mot:programmes"]);
  if (!data) {
    return (
      <main>
        <h1>Programmes</h1>
        <p>Unable to load your Transport information.</p>
        <Link href={"/consent?dept=mot"}>Grant Transport access</Link>
      </main>
    );
  }

  const rows = data.programmes ?? [];

  return (
    <main>
      <Link href={"/dept/mot"}>← Back to Transport</Link>
      <h1>Programmes</h1>
      {rows.length === 0 ? (
        <p>No programmes on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>name</th>
                <th>budget</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.name}</td>
                  <td>{row.budget}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
