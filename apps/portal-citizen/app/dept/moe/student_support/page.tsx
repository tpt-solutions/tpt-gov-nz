import Link from "next/link";
import { fetchMoeData } from "../actions";

export const metadata = { title: "Student support — Ministry of Education — My Gov NZ" };

export default async function MoeStudentSupportPage() {
  const data = await fetchMoeData(["moe:student-support"]);
  if (!data) {
    return (
      <main>
        <h1>Student support</h1>
        <p>Unable to load your Education information.</p>
        <Link href={"/consent?dept=moe"}>Grant Education access</Link>
      </main>
    );
  }

  const rows = data.student_support ?? [];

  return (
    <main>
      <Link href={"/dept/moe"}>← Back to Education</Link>
      <h1>Student support</h1>
      {rows.length === 0 ? (
        <p>No student support on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>service</th>
                <th>status</th>
                <th>nextReview</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.service}</td>
                  <td>{row.status}</td>
                  <td>{row.nextReview}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
