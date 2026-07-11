import Link from "next/link";
import { fetchSfoData } from "../actions";

export const metadata = { title: "Investigations — Serious Fraud Office — My Gov NZ" };

export default async function SfoInvestigationsPage() {
  const data = await fetchSfoData(["sfo:investigations"]);
  if (!data) {
    return (
      <main>
        <h1>Investigations</h1>
        <p>Unable to load your SFO information.</p>
        <Link href={"/consent?dept=sfo"}>Grant SFO access</Link>
      </main>
    );
  }

  const rows = data.investigations ?? [];

  return (
    <main>
      <Link href={"/dept/sfo"}>← Back to SFO</Link>
      <h1>Investigations</h1>
      {rows.length === 0 ? (
        <p>No investigations on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>matter</th>
                <th>status</th>
                <th>openedDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.matter}</td>
                  <td>{row.status}</td>
                  <td>{row.openedDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
