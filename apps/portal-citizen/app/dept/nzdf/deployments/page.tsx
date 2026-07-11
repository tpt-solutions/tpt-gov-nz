import Link from "next/link";
import { fetchNzdfData } from "../actions";

export const metadata = { title: "Deployments — New Zealand Defence Force — My Gov NZ" };

export default async function NzdfDeploymentsPage() {
  const data = await fetchNzdfData(["nzdf:deployments"]);
  if (!data) {
    return (
      <main>
        <h1>Deployments</h1>
        <p>Unable to load your NZDF information.</p>
        <Link href={"/consent?dept=nzdf"}>Grant NZDF access</Link>
      </main>
    );
  }

  const rows = data.deployments ?? [];

  return (
    <main>
      <Link href={"/dept/nzdf"}>← Back to NZDF</Link>
      <h1>Deployments</h1>
      {rows.length === 0 ? (
        <p>No deployments on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>operation</th>
                <th>country</th>
                <th>year</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.operation}</td>
                  <td>{row.country}</td>
                  <td>{row.year}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
