import Link from "next/link";
import { fetchEroData } from "../actions";

export const metadata = { title: "Reviews — Education Review Office — My Gov NZ" };

export default async function EroReviewsPage() {
  const data = await fetchEroData(["ero:reviews"]);
  if (!data) {
    return (
      <main>
        <h1>Reviews</h1>
        <p>Unable to load your ERO information.</p>
        <Link href={"/consent?dept=ero"}>Grant ERO access</Link>
      </main>
    );
  }

  const rows = data.reviews ?? [];

  return (
    <main>
      <Link href={"/dept/ero"}>← Back to ERO</Link>
      <h1>Reviews</h1>
      {rows.length === 0 ? (
        <p>No reviews on file.</p>
      ) : (
        <table>
          <thead>
            <tr>
                <th>school</th>
                <th>rating</th>
                <th>reviewDate</th>
                <th>nextReview</th>
            </tr>
          </thead>
          <tbody>
            {rows.map((row, i) => (
              <tr key={i}>
                  <td>{row.school}</td>
                  <td>{row.rating}</td>
                  <td>{row.reviewDate}</td>
                  <td>{row.nextReview}</td>
              </tr>
            ))}
          </tbody>
        </table>
      )}
    </main>
  );
}
