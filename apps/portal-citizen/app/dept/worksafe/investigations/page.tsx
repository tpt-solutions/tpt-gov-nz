import Link from "next/link";
import { fetchWorksafeData } from "../actions";

export const metadata = { title: "Investigations — WorkSafe New Zealand — My Gov NZ" };

export default async function WorksafeInvestigationsPage() {
  const data = await fetchWorksafeData(["worksafe:investigations"]);
  if (!data) {
    return (
      <main>
        <h1>Investigations</h1>
        <p>Unable to load your WorkSafe information.</p>
        <Link href={"/consent?dept=worksafe"}>Grant WorkSafe access</Link>
      </main>
    );
  }

  const rows = data.investigations ?? [];

  return (
    <main>
      <Link href={"/dept/worksafe"}>← Back to WorkSafe</Link>
      <h1>Investigations</h1>
      {rows.length === 0 ? (
        <p>No investigations on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>reference</th>
                <th>matter</th>
                <th>status</th>
                <th>openedDate</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.reference}</td>
                  <td>{row.matter}</td>
                  <td>{row.status}</td>
                  <td>{row.openedDate}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
