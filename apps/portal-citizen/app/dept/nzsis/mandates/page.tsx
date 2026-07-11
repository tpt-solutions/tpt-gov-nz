import Link from "next/link";
import { fetchNzsisData } from "../actions";

export const metadata = { title: "Mandates — New Zealand Security Intelligence Service — My Gov NZ" };

export default async function NzsisMandatesPage() {
  const data = await fetchNzsisData(["nzsis:mandates"]);
  if (!data) {
    return (
      <main>
        <h1>Mandates</h1>
        <p>Unable to load your NZSIS information.</p>
        <Link href={"/consent?dept=nzsis"}>Grant NZSIS access</Link>
      </main>
    );
  }

  const rows = data.mandates ?? [];

  return (
    <main>
      <Link href={"/dept/nzsis"}>← Back to NZSIS</Link>
      <h1>Mandates</h1>
      {rows.length === 0 ? (
        <p>No mandates on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>agency</th>
                <th>status</th>
                <th>issuedDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.agency}</td>
                  <td>{row.status}</td>
                  <td>{row.issuedDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
