import Link from "next/link";
import { fetchMfeData } from "../actions";

export const metadata = { title: "Emissions — Ministry for the Environment — My Gov NZ" };

export default async function MfeEmissionsPage() {
  const data = await fetchMfeData(["mfe:emissions"]);
  if (!data) {
    return (
      <main>
        <h1>Emissions</h1>
        <p>Unable to load your MfE information.</p>
        <Link href={"/consent?dept=mfe"}>Grant MfE access</Link>
      </main>
    );
  }

  const rows = data.emissions ?? [];

  return (
    <main>
      <Link href={"/dept/mfe"}>← Back to MfE</Link>
      <h1>Emissions</h1>
      {rows.length === 0 ? (
        <p>No emissions on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reportYear</th>
                <th>sector</th>
                <th>tonnesCO2e</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reportYear}</td>
                  <td>{row.sector}</td>
                  <td>{row.tonnesCO2e}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
