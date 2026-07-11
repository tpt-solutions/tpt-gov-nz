import Link from "next/link";
import { fetchMsdData } from "./actions";
import MsdAiPrompt from "./ai-prompt";

export const metadata = { title: "MSD — My Gov NZ" };

export default async function MsdOverviewPage() {
  const data = await fetchMsdData(["msd:studylink", "msd:case-history"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry of Social Development</h1>
        <p>Unable to load your information. Please grant access to continue.</p>
        <Link href="/consent?dept=msd">Grant MSD access</Link>
      </main>
    );
  }

  const studylink = data.studylink;
  const caseEvents = data.caseHistory ?? [];

  return (
    <main>
      <h1>Ministry of Social Development</h1>
      <p>Client number: ••••{data.clientNumber.slice(-4)}</p>

      <section>
        <h2>StudyLink</h2>
        <p>
          {studylink
            ? `${studylink.hasStudentLoan ? "Student loan" : "No student loan"}${
                studylink.hasAllowance ? " · student allowance" : ""
              }.`
            : "No StudyLink record."}
        </p>
        <Link href="/dept/msd/studylink">View StudyLink →</Link>
      </section>

      <section>
        <h2>Case history</h2>
        <p>{caseEvents.length} case event(s) on record.</p>
        <Link href="/dept/msd/case-history">View case history →</Link>
      </section>

      <MsdAiPrompt />
    </main>
  );
}
