import Link from "next/link";
import { fetchPoliceData } from "./actions";
import PoliceAiPrompt from "./ai-prompt";

export const metadata = { title: "New Zealand Police — My Gov NZ" };

export default async function PoliceOverviewPage() {
  const data = await fetchPoliceData(["police:infringements", "police:reports"]);

  if (!data) {
    return (
      <main>
        <h1>New Zealand Police</h1>
        <p>Unable to load your Police information. Please grant access to continue.</p>
        <Link href="/consent?dept=police">Grant Police access</Link>
      </main>
    );
  }

  const unpaidInfringements = data.infringements?.filter((i) => i.status === "unpaid") ?? [];
  const openReports = data.reports?.filter((r) => r.status !== "closed") ?? [];

  return (
    <main>
      <h1>New Zealand Police</h1>
      <p>Client number: ••••{data.clientNumber.slice(-4)}</p>

      <section>
        <h2>Infringements</h2>
        <p>{unpaidInfringements.length} unpaid infringement(s).</p>
        <Link href="/dept/police/infringements">View infringements →</Link>
      </section>

      <section>
        <h2>Reports</h2>
        <p>{openReports.length} open report(s).</p>
        <Link href="/dept/police/reports">View reports →</Link>
      </section>

      <PoliceAiPrompt />
    </main>
  );
}
