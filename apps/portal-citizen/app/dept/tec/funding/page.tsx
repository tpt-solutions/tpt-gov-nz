import Link from "next/link";
import { fetchTecData } from "../actions";

export const metadata = { title: "Funding — Tertiary Education Commission — My Gov NZ" };

export default async function TecFundingPage() {
  const data = await fetchTecData(["tec:funding"]);
  if (!data) {
    return (
      <main>
        <h1>Funding</h1>
        <p>Unable to load your TEC information.</p>
        <Link href={"/consent?dept=tec"}>Grant TEC access</Link>
      </main>
    );
  }

  const rows = data.funding ?? [];

  return (
    <main>
      <Link href={"/dept/tec"}>← Back to TEC</Link>
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
