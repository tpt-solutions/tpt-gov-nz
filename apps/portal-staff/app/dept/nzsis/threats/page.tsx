import Link from "next/link";
import { fetchNzsisDataForCitizen } from "../actions";

export const metadata = { title: "Threats — New Zealand Security Intelligence Service — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function NzsisThreatsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchNzsisDataForCitizen(did, ["nzsis:threats"]);
  const rows = data?.threats ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/nzsis?did=${encodeURIComponent(did)}`}>← Back to NZSIS case file</Link>
      <h1>Threats</h1>
      {rows.length === 0 ? (
        <p>No threats on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>category</th>
                <th>status</th>
                <th>assessedDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.category}</td>
                  <td>{row.status}</td>
                  <td>{row.assessedDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
