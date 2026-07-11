import Link from "next/link";
import { fetchNzsisDataForCitizen } from "../actions";

export const metadata = { title: "Mandates — New Zealand Security Intelligence Service — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function NzsisMandatesStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchNzsisDataForCitizen(did, ["nzsis:mandates"]);
  const rows = data?.mandates ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/nzsis?did=${encodeURIComponent(did)}`}>← Back to NZSIS case file</Link>
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
