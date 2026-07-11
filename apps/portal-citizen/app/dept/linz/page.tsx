import Link from "next/link";
import { fetchLinzData } from "./actions";
import LinzAiPrompt from "./ai-prompt";

export const metadata = { title: "LINZ — My Gov NZ" };

export default async function LinzOverviewPage() {
  const data = await fetchLinzData(["linz:titles", "linz:ownership"]);

  if (!data) {
    return (
      <main>
        <h1>Toitū Te Whenua Land Information New Zealand</h1>
        <p>Unable to load your land information. Please grant access to continue.</p>
        <Link href="/consent?dept=linz">Grant LINZ access</Link>
      </main>
    );
  }

  const titles = data.titles ?? [];
  const ownership = data.ownership ?? [];

  return (
    <main>
      <h1>Toitū Te Whenua Land Information New Zealand</h1>
      <p>Customer ID: ••••{data.customerId.slice(-4)}</p>

      <section>
        <h2>Titles</h2>
        <p>{titles.length} title(s) in your name.</p>
        <Link href="/dept/linz/titles">View titles →</Link>
      </section>

      <section>
        <h2>Ownership</h2>
        <p>{ownership.length} ownership record(s).</p>
        <Link href="/dept/linz/ownership">View ownership →</Link>
      </section>

      <LinzAiPrompt />
    </main>
  );
}
