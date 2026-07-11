import Link from "next/link";
import { fetchTecData } from "../actions";

export const metadata = { title: "Courses — Tertiary Education Commission — My Gov NZ" };

export default async function TecCoursesPage() {
  const data = await fetchTecData(["tec:courses"]);
  if (!data) {
    return (
      <main>
        <h1>Courses</h1>
        <p>Unable to load your TEC information.</p>
        <Link href={"/consent?dept=tec"}>Grant TEC access</Link>
      </main>
    );
  }

  const rows = data.courses ?? [];

  return (
    <main>
      <Link href={"/dept/tec"}>← Back to TEC</Link>
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
