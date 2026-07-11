import Link from "next/link";
import { fetchMoeDataForCitizen } from "../actions";

export const metadata = { title: "Student support — Ministry of Education — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MoeStudentSupportStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchMoeDataForCitizen(did, ["moe:student-support"]);
  const rows = data?.student_support ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/moe?did=${encodeURIComponent(did)}`}>← Back to Education case file</Link>
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
