import Link from "next/link";
import { fetchMchData } from "../actions";

export const metadata = { title: "Heritage sites — Ministry for Culture and Heritage — My Gov NZ" };

export default async function MchHeritageSitesPage() {
  const data = await fetchMchData(["mch:heritage-sites"]);
  if (!data) {
    return (
      <main>
        <h1>Heritage sites</h1>
        <p>Unable to load your MCH information.</p>
        <Link href={"/consent?dept=mch"}>Grant MCH access</Link>
      </main>
    );
  }

  const rows = data.heritage_sites ?? [];

  return (
    <main>
      <Link href={"/dept/mch"}>← Back to MCH</Link>
      <h1>Heritage sites</h1>
      {rows.length === 0 ? (
        <p>No heritage sites on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>name</th>
                <th>status</th>
                <th>region</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.name}</td>
                  <td>{row.status}</td>
                  <td>{row.region}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
