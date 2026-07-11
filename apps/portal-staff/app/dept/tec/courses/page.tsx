import Link from "next/link";
import { fetchTecDataForCitizen } from "../actions";

export const metadata = { title: "Courses — Tertiary Education Commission — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function TecCoursesStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchTecDataForCitizen(did, ["tec:courses"]);
  const rows = data?.courses ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/tec?did=${encodeURIComponent(did)}`}>← Back to TEC case file</Link>
      <h1>Courses</h1>
      {rows.length === 0 ? (
        <p>No courses on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>courseName</th>
                <th>provider</th>
                <th>status</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.courseName}</td>
                  <td>{row.provider}</td>
                  <td>{row.status}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
