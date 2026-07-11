import Link from "next/link";
import { fetchWorksafeData } from "../actions";

export const metadata = { title: "Inspections — WorkSafe New Zealand — My Gov NZ" };

export default async function WorksafeInspectionsPage() {
  const data = await fetchWorksafeData(["worksafe:inspections"]);
  if (!data) {
    return (
      <main>
        <h1>Inspections</h1>
        <p>Unable to load your WorkSafe information.</p>
        <Link href={"/consent?dept=worksafe"}>Grant WorkSafe access</Link>
      </main>
    );
  }

  const rows = data.inspections ?? [];

  return (
    <main>
      <Link href={"/dept/worksafe"}>← Back to WorkSafe</Link>
      <h1>Inspections</h1>
      {rows.length === 0 ? (
        <p>No inspections on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>site</th>
                <th>inspectionDate</th>
                <th>outcome</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.site}</td>
                  <td>{row.inspectionDate}</td>
                  <td>{row.outcome}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
