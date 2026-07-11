import Link from "next/link";
import { fetchNzqaData } from "../actions";

export const metadata = { title: "NZQA Qualifications — My Gov NZ" };

export default async function NzqaQualificationsPage() {
  const data = await fetchNzqaData(["nzqa:qualifications"]);
  const qualifications = data?.qualifications ?? [];

  return (
    <main>
      <Link href="/dept/nzqa">← Back to NZQA</Link>
      <h1>NZQA Qualifications</h1>

      {qualifications.length === 0 ? (
        <p>No qualifications on record.</p>
      ) : (
        <ul>
          {qualifications.map((q) => (
            <li key={q.qualificationId}>
              <strong>{q.title}</strong> — Level {q.level}
              <br />
              <small>
                {q.provider} · awarded {q.awardedDate}
              </small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
