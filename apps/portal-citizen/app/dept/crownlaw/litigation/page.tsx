import Link from "next/link";
import { fetchCrownlawData } from "../actions";

export const metadata = { title: "Litigation — Crown Law Office — My Gov NZ" };

export default async function CrownlawLitigationPage() {
  const data = await fetchCrownlawData(["crownlaw:litigation"]);
  if (!data) {
    return (
      <main>
        <h1>Litigation</h1>
        <p>Unable to load your Crown Law information.</p>
        <Link href={"/consent?dept=crownlaw"}>Grant Crown Law access</Link>
      </main>
    );
  }

  const rows = data.litigation ?? [];

  return (
    <main>
      <Link href={"/dept/crownlaw"}>← Back to Crown Law</Link>
      <h1>Litigation</h1>
      {rows.length === 0 ? (
        <p>No litigation on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>caseName</th>
                <th>crownRole</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.caseName}</td>
                  <td>{row.crownRole}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
