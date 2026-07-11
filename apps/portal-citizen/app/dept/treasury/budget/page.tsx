import Link from "next/link";
import { fetchTreasuryData } from "../actions";

export const metadata = { title: "Budget — The Treasury — My Gov NZ" };

export default async function TreasuryBudgetPage() {
  const data = await fetchTreasuryData(["treasury:budget"]);
  if (!data) {
    return (
      <main>
        <h1>Budget</h1>
        <p>Unable to load your Treasury information.</p>
        <Link href={"/consent?dept=treasury"}>Grant Treasury access</Link>
      </main>
    );
  }

  const rows = data.budget ?? [];

  return (
    <main>
      <Link href={"/dept/treasury"}>← Back to Treasury</Link>
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
