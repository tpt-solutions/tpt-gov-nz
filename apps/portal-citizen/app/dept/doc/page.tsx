import Link from "next/link";
import { fetchDocData } from "./actions";
import DocAiPrompt from "./ai-prompt";

export const metadata = { title: "DOC — My Gov NZ" };

export default async function DocOverviewPage() {
  const data = await fetchDocData(["doc:permits", "doc:concessions"]);

  if (!data) {
    return (
      <main>
        <h1>Department of Conservation</h1>
        <p>Unable to load your conservation information. Please grant access to continue.</p>
        <Link href="/consent?dept=doc">Grant conservation access</Link>
      </main>
    );
  }

  const activePermits =
    data.permits?.filter((p) => p.status === "active" || p.status === "pending") ?? [];
  const activeConcessions =
    data.concessions?.filter(
      (c) => new Date(c.endDate) >= new Date(new Date().toDateString())
    ) ?? [];

  return (
    <main>
      <h1>Department of Conservation</h1>
      <p>Doc id: ••••{data.docId.slice(-4)}</p>

      <section>
        <h2>Permits</h2>
        <p>{activePermits.length} active permit(s).</p>
        <Link href="/dept/doc/permits">View permits →</Link>
      </section>

      <section>
        <h2>Concessions</h2>
        <p>{activeConcessions.length} active concession(s).</p>
        <Link href="/dept/doc/concessions">View concessions →</Link>
      </section>

      <DocAiPrompt />
    </main>
  );
}
