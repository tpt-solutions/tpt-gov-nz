import Link from "next/link";
import { fetchMojData } from "./actions";
import MojAiPrompt from "./ai-prompt";

export const metadata = { title: "Ministry of Justice — My Gov NZ" };

export default async function MojOverviewPage() {
  const data = await fetchMojData(["moj:fines", "moj:disputes", "moj:court-records"]);

  if (!data) {
    return (
      <main>
        <h1>Ministry of Justice</h1>
        <p>Unable to load your Ministry of Justice information. Please grant access to continue.</p>
        <Link href="/consent?dept=moj">Grant Ministry of Justice access</Link>
      </main>
    );
  }

  const unpaidFines = data.fines?.filter((f) => f.status === "unpaid" || f.status === "overdue") ?? [];
  const openDisputes = data.disputes?.filter((d) => d.status === "filed" || d.status === "scheduled") ?? [];
  const openCases = data.courtRecords?.filter((c) => c.status === "open") ?? [];

  return (
    <main>
      <h1>Ministry of Justice</h1>
      <p>Client number: ••••{data.clientNumber.slice(-4)}</p>

      <section>
        <h2>Fines</h2>
        <p>{unpaidFines.length} unpaid or overdue fine(s).</p>
        <Link href="/dept/moj/fines">View fines →</Link>
      </section>

      <section>
        <h2>Disputes Tribunal</h2>
        <p>{openDisputes.length} open dispute(s).</p>
        <Link href="/dept/moj/disputes">View disputes →</Link>
      </section>

      <section>
        <h2>Court records</h2>
        <p>{openCases.length} open case(s).</p>
        <Link href="/dept/moj/court-records">View court records →</Link>
      </section>

      <section>
        <h2>Name change</h2>
        <Link href="/dept/moj/name-change">Request a name change →</Link>
      </section>

      <MojAiPrompt />
    </main>
  );
}
