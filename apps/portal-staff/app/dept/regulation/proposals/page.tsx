import Link from "next/link";
import { fetchRegulationDataForCitizen } from "../actions";

export const metadata = { title: "Proposals — Ministry for Regulation — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function RegulationProposalsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchRegulationDataForCitizen(did, ["regulation:proposals"]);
  const rows = data?.proposals ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/regulation?did=${encodeURIComponent(did)}`}>← Back to Regulation case file</Link>
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
