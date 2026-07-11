import Link from "next/link";
import { fetchCorrectionsData } from "./actions";
import CorrectionsAiPrompt from "./ai-prompt";

export const metadata = { title: "Corrections — My Gov NZ" };

export default async function CorrectionsOverviewPage() {
  const data = await fetchCorrectionsData(["corrections:probation", "corrections:case"]);

  if (!data) {
    return (
      <main>
        <h1>Department of Corrections</h1>
        <p>Unable to load your Corrections information. Please grant access to continue.</p>
        <Link href="/consent?dept=corrections">Grant Corrections access</Link>
      </main>
    );
  }

  const hasProbation = !!data.probation;
  const activeCases = data.case ?? [];

  return (
    <main>
      <h1>Department of Corrections</h1>
      <p>Corrections ID: ••••{data.correctionsId.slice(-4)}</p>

      <section>
        <h2>Probation</h2>
        <p>{hasProbation ? "You have an active probation record." : "No probation record on file."}</p>
        <Link href="/dept/corrections/probation">View probation →</Link>
      </section>

      <section>
        <h2>Case</h2>
        <p>{activeCases.length} case(s) on record.</p>
        <Link href="/dept/corrections/case">View cases →</Link>
      </section>

      <CorrectionsAiPrompt />
    </main>
  );
}
