import Link from "next/link";
import { fetchNzqaDataForCitizen } from "../actions";

export const metadata = { title: "Qualifications — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function NzqaStaffQualificationsPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Qualifications — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchNzqaDataForCitizen(did, ["nzqa:qualifications"]);
  const qualifications = data?.qualifications;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/nzqa?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Qualifications — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load qualifications.</p>}
      {data && (!qualifications || qualifications.length === 0) && <p>No qualifications on record.</p>}

      {qualifications && qualifications.length > 0 && (
        <ul>
          {qualifications.map((q) => (
            <li key={q.qualificationId}>
              <strong>{q.title}</strong> — Level {q.level}
              <br />
              <small>{q.provider} · awarded {q.awardedDate}</small>
            </li>
          ))}
        </ul>
      )}
    </main>
  );
}
