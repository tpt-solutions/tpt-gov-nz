import Link from "next/link";
import { fetchGcsbDataForCitizen } from "../actions";

export const metadata = { title: "Engagements — Government Communications Security Bureau — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function GcsbEngagementsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchGcsbDataForCitizen(did, ["gcsb:engagements"]);
  const rows = data?.engagements ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/gcsb?did=${encodeURIComponent(did)}`}>← Back to GCSB case file</Link>
      <h1>Engagements</h1>
      {rows.length === 0 ? (
        <p>No engagements on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>partner</th>
                <th>engagementType</th>
                <th>engagementDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.partner}</td>
                  <td>{row.engagementType}</td>
                  <td>{row.engagementDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
