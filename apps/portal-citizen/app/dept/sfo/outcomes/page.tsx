import Link from "next/link";
import { fetchSfoData } from "../actions";

export const metadata = { title: "Outcomes — Serious Fraud Office — My Gov NZ" };

export default async function SfoOutcomesPage() {
  const data = await fetchSfoData(["sfo:outcomes"]);
  if (!data) {
    return (
      <main>
        <h1>Outcomes</h1>
        <p>Unable to load your SFO information.</p>
        <Link href={"/consent?dept=sfo"}>Grant SFO access</Link>
      </main>
    );
  }

  const rows = data.outcomes ?? [];

  return (
    <main>
      <Link href={"/dept/sfo"}>← Back to SFO</Link>
      <h1>Outcomes</h1>
      {rows.length === 0 ? (
        <p>No outcomes on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>result</th>
                <th>resultDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.result}</td>
                  <td>{row.resultDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
