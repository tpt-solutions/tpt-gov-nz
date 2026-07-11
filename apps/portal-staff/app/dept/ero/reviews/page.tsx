import Link from "next/link";
import { fetchEroDataForCitizen } from "../actions";

export const metadata = { title: "Reviews — Education Review Office — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function EroReviewsStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchEroDataForCitizen(did, ["ero:reviews"]);
  const rows = data?.reviews ?? [];

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/ero?did=${encodeURIComponent(did)}`}>← Back to ERO case file</Link>
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
