import Link from "next/link";
import { fetchEqcDataForCitizen } from "../actions";

export const metadata = { title: "Claims — Earthquake Commission (Toka Tū Ake) — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function EqcClaimsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchEqcDataForCitizen(did, ["eqc:claims"]);
  const rows = data?.claims ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/eqc?did=${encodeURIComponent(did)}`}>← Back to EQC case file</Link>
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
