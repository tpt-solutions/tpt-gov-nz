import Link from "next/link";
import { fetchCaaData } from "../actions";

export const metadata = { title: "Licences — Civil Aviation Authority — My Gov NZ" };

export default async function CaaLicencesPage() {
  const data = await fetchCaaData(["caa:licences"]);
  if (!data) {
    return (
      <main>
        <h1>Licences</h1>
        <p>Unable to load your CAA information.</p>
        <Link href={"/consent?dept=caa"}>Grant CAA access</Link>
      </main>
    );
  }

  const rows = data.licences ?? [];

  return (
    <main>
      <Link href={"/dept/caa"}>← Back to CAA</Link>
      <h1>Licences</h1>
      {rows.length === 0 ? (
        <p>No licences on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>licenceNo</th>
                <th>category</th>
                <th>status</th>
                <th>expires</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.licenceNo}</td>
                  <td>{row.category}</td>
                  <td>{row.status}</td>
                  <td>{row.expires}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
