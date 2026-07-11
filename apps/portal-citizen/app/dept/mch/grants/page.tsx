import Link from "next/link";
import { fetchMchData } from "../actions";

export const metadata = { title: "Grants — Ministry for Culture and Heritage — My Gov NZ" };

export default async function MchGrantsPage() {
  const data = await fetchMchData(["mch:grants"]);
  if (!data) {
    return (
      <main>
        <h1>Grants</h1>
        <p>Unable to load your MCH information.</p>
        <Link href={"/consent?dept=mch"}>Grant MCH access</Link>
      </main>
    );
  }

  const rows = data.grants ?? [];

  return (
    <main>
      <Link href={"/dept/mch"}>← Back to MCH</Link>
      <h1>Grants</h1>
      {rows.length === 0 ? (
        <p>No grants on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>grantName</th>
                <th>amount</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.grantName}</td>
                  <td>{row.amount}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
