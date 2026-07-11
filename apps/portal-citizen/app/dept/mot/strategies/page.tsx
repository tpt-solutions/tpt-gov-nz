import Link from "next/link";
import { fetchMotData } from "../actions";

export const metadata = { title: "Strategies — Ministry of Transport — My Gov NZ" };

export default async function MotStrategiesPage() {
  const data = await fetchMotData(["mot:strategies"]);
  if (!data) {
    return (
      <main>
        <h1>Strategies</h1>
        <p>Unable to load your Transport information.</p>
        <Link href={"/consent?dept=mot"}>Grant Transport access</Link>
      </main>
    );
  }

  const rows = data.strategies ?? [];

  return (
    <main>
      <Link href={"/dept/mot"}>← Back to Transport</Link>
      <h1>Strategies</h1>
      {rows.length === 0 ? (
        <p>No strategies on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>title</th>
                <th>year</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.title}</td>
                  <td>{row.year}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
