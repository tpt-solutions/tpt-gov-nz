import Link from "next/link";
import { fetchTearawhitiData } from "../actions";

export const metadata = { title: "Engagements — Te Arawhiti — My Gov NZ" };

export default async function TearawhitiEngagementsPage() {
  const data = await fetchTearawhitiData(["tearawhiti:engagements"]);
  if (!data) {
    return (
      <main>
        <h1>Engagements</h1>
        <p>Unable to load your Te Arawhiti information.</p>
        <Link href={"/consent?dept=tearawhiti"}>Grant Te Arawhiti access</Link>
      </main>
    );
  }

  const rows = data.engagements ?? [];

  return (
    <main>
      <Link href={"/dept/tearawhiti"}>← Back to Te Arawhiti</Link>
      <h1>Engagements</h1>
      {rows.length === 0 ? (
        <p>No engagements on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>topic</th>
                <th>engagementDate</th>
                <th>outcome</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.topic}</td>
                  <td>{row.engagementDate}</td>
                  <td>{row.outcome}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
