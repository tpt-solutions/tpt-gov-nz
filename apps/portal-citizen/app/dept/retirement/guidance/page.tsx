import Link from "next/link";
import { fetchRetirementData } from "../actions";

export const metadata = { title: "Guidance — Retirement Commission (Te Ara Ahunga Ora) — My Gov NZ" };

export default async function RetirementGuidancePage() {
  const data = await fetchRetirementData(["retirement:guidance"]);
  if (!data) {
    return (
      <main>
        <h1>Guidance</h1>
        <p>Unable to load your Retirement information.</p>
        <Link href={"/consent?dept=retirement"}>Grant Retirement access</Link>
      </main>
    );
  }

  const rows = data.guidance ?? [];

  return (
    <main>
      <Link href={"/dept/retirement"}>← Back to Retirement</Link>
      <h1>Guidance</h1>
      {rows.length === 0 ? (
        <p>No guidance on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>topic</th>
                <th>summary</th>
                <th>published</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.topic}</td>
                  <td>{row.summary}</td>
                  <td>{row.published}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
