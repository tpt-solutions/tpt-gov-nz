import Link from "next/link";
import { fetchTecDataForCitizen } from "../actions";

export const metadata = { title: "Funding — Tertiary Education Commission — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function TecFundingStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchTecDataForCitizen(did, ["tec:funding"]);
  const rows = data?.funding ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/tec?did=${encodeURIComponent(did)}`}>← Back to TEC case file</Link>
      <h1>Funding</h1>
      {rows.length === 0 ? (
        <p>No funding on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>provider</th>
                <th>amount</th>
                <th>year</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.provider}</td>
                  <td>{row.amount}</td>
                  <td>{row.year}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
