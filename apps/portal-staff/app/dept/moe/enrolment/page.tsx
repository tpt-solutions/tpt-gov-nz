import Link from "next/link";
import { fetchMoeDataForCitizen } from "../actions";

export const metadata = { title: "Enrolment — Ministry of Education — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function MoeEnrolmentStaffPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;
  if (!did) return <main><p>No citizen selected.</p></main>;

  const data = await fetchMoeDataForCitizen(did, ["moe:enrolment"]);
  const item = data?.enrolment;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/moe?did=${encodeURIComponent(did)}`}>← Back to Education case file</Link>
      <h1>Enrolment</h1>
      {item ? (
        <div>
        <p><strong>school:</strong> {item.school}</p>
        <p><strong>yearLevel:</strong> {item.yearLevel}</p>
        <p><strong>status:</strong> {item.status}</p>
        </div>
      ) : (
        <p>No enrolment on file.</p>
      )}
    </main>
  );
}
