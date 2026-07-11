import Link from "next/link";
import { fetchNzqaData } from "./actions";
import NzqaAiPrompt from "./ai-prompt";

export const metadata = { title: "NZQA — My Gov NZ" };

export default async function NzqaOverviewPage() {
  const data = await fetchNzqaData(["nzqa:qualifications", "nzqa:transcripts"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry of Education / NZQA</h1>
        <p>Unable to load your NZQA information. Please grant access to continue.</p>
        <Link href="/consent?dept=nzqa">Grant NZQA access</Link>
      </main>
    );
  }

  const qualificationCount = data.qualifications?.length ?? 0;
  const hasTranscript = !!data.transcript;

  return (
    <main>
      <h1>Ministry of Education / NZQA</h1>
      <p>NSN: ••••{data.nsn.slice(-4)}</p>

      <section>
        <h2>Qualifications</h2>
        <p>{qualificationCount} qualification(s) on record.</p>
        <Link href="/dept/nzqa/qualifications">View qualifications →</Link>
      </section>

      <section>
        <h2>Transcript</h2>
        <p>{hasTranscript ? "Your Record of Achievement is available." : "No transcript on record yet."}</p>
        <Link href="/dept/nzqa/transcript">View transcript →</Link>
      </section>

      <NzqaAiPrompt />
    </main>
  );
}
