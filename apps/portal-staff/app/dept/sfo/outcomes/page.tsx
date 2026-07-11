import Link from "next/link";
import { fetchSfoDataForCitizen } from "../actions";

export const metadata = { title: "Outcomes — Serious Fraud Office — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function SfoOutcomesStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchSfoDataForCitizen(did, ["sfo:outcomes"]);
  const rows = data?.outcomes ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/sfo?did=${encodeURIComponent(did)}`}>← Back to SFO case file</Link>
      <h1>Outcomes</h1>
      {rows.length === 0 ? (
        <p>No outcomes on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>result</th>
                <th>resultDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.result}</td>
                  <td>{row.resultDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
