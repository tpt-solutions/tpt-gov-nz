import Link from "next/link";
import { fetchRegulationData } from "../actions";

export const metadata = { title: "Proposals — Ministry for Regulation — My Gov NZ" };

export default async function RegulationProposalsPage() {
  const data = await fetchRegulationData(["regulation:proposals"]);
  if (!data) {
    return (
      <main>
        <h1>Proposals</h1>
        <p>Unable to load your Regulation information.</p>
        <Link href={"/consent?dept=regulation"}>Grant Regulation access</Link>
      </main>
    );
  }

  const rows = data.proposals ?? [];

  return (
    <main>
      <Link href={"/dept/regulation"}>← Back to Regulation</Link>
      <h1>Proposals</h1>
      {rows.length === 0 ? (
        <p>No proposals on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>title</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.title}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
