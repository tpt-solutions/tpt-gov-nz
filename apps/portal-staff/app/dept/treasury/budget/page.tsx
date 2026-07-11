import Link from "next/link";
import { fetchTreasuryDataForCitizen } from "../actions";

export const metadata = { title: "Budget — The Treasury — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function TreasuryBudgetStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchTreasuryDataForCitizen(did, ["treasury:budget"]);
  const rows = data?.budget ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/treasury?did=${encodeURIComponent(did)}`}>← Back to Treasury case file</Link>
      <h1>Budget</h1>
      {rows.length === 0 ? (
        <p>No budget on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>fiscalYear</th>
                <th>portfolio</th>
                <th>appropriation</th>
                <th>amount</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.fiscalYear}</td>
                  <td>{row.portfolio}</td>
                  <td>{row.appropriation}</td>
                  <td>{row.amount}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
