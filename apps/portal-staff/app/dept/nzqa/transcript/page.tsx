import Link from "next/link";
import { fetchNzqaDataForCitizen } from "../actions";

export const metadata = { title: "Transcript — Case File — My Gov NZ" };

type SearchParams = { did?: string };

export default async function NzqaStaffTranscriptPage({
  searchParams,
}: {
  searchParams: Promise<SearchParams>;
}) {
  const { did } = await searchParams;

  if (!did) {
    return (
      <main style={{ padding: "1rem" }}>
        <h1>Transcript — Case File</h1>
        <p>No citizen selected.</p>
        <Link href="/">← Back</Link>
      </main>
    );
  }

  const data = await fetchNzqaDataForCitizen(did, ["nzqa:transcripts"]);
  const transcript = data?.transcript;

  return (
    <main style={{ padding: "1rem" }}>
      <Link href={`/dept/nzqa?did=${encodeURIComponent(did)}`}>← Back to case file</Link>
      <h1>Transcript — Case File</h1>
      <p>
        <strong>Citizen:</strong> {did}
      </p>

      {!data && <p>Unable to load transcript.</p>}
      {data && !transcript && <p>No transcript on record.</p>}

      {transcript && (
        <ul>
          {transcript.recordSummary && (
            <li>
              <strong>Summary:</strong> {transcript.recordSummary}
            </li>
          )}
          {transcript.totalCredits != null && (
            <li>
              <strong>Total credits:</strong> {transcript.totalCredits}
            </li>
          )}
          {transcript.creditSummary && (
            <li>
              <strong>Credit summary:</strong> {transcript.creditSummary}
            </li>
          )}
        </ul>
      )}
    </main>
  );
}
