import Link from "next/link";
import { fetchEqcData } from "../actions";

export const metadata = { title: "Claims — Earthquake Commission (Toka Tū Ake) — My Gov NZ" };

export default async function EqcClaimsPage() {
  const data = await fetchEqcData(["eqc:claims"]);
  if (!data) {
    return (
      <main>
        <h1>Claims</h1>
        <p>Unable to load your EQC information.</p>
        <Link href={"/consent?dept=eqc"}>Grant EQC access</Link>
      </main>
    );
  }

  const rows = data.claims ?? [];

  return (
    <main>
      <Link href={"/dept/eqc"}>← Back to EQC</Link>
      <h1>Claims</h1>
      {rows.length === 0 ? (
        <p>No claims on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>property</th>
                <th>status</th>
                <th>lodgedDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.property}</td>
                  <td>{row.status}</td>
                  <td>{row.lodgedDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
