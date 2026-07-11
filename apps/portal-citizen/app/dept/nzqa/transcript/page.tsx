import Link from "next/link";
import RequestTranscriptForm from "./request/form";
import { fetchNzqaData } from "../actions";

export const metadata = { title: "NZQA Transcript — My Gov NZ" };

export default async function NzqaTranscriptPage() {
  const data = await fetchNzqaData(["nzqa:transcripts"]);
  const transcript = data?.transcript;

  return (
    <main>
      <Link href="/dept/nzqa">← Back to NZQA</Link>
      <h1>NZQA Transcript</h1>

      <section>
        <h2>Your Record of Achievement</h2>
        {!transcript ? (
          <p>No transcript on record yet.</p>
        ) : (
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
      </section>

      <section>
        <h2>Request an official transcript</h2>
        <RequestTranscriptForm />
      </section>
    </main>
  );
}
