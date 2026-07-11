import Link from "next/link";
import { fetchMoeData } from "../actions";

export const metadata = { title: "Enrolment — Ministry of Education — My Gov NZ" };

export default async function MoeEnrolmentPage() {
  const data = await fetchMoeData(["moe:enrolment"]);
  if (!data) {
    return (
      <main>
        <h1>Enrolment</h1>
        <p>Unable to load your Education information.</p>
        <Link href={"/consent?dept=moe"}>Grant Education access</Link>
      </main>
    );
  }

  const item = data.enrolment;

  return (
    <main>
      <Link href={"/dept/moe"}>← Back to Education</Link>
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
