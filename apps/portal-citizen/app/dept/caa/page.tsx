import Link from "next/link";
import { fetchCaaData } from "./actions";
import CaaAiPrompt from "./ai-prompt";

export const metadata = { title: "Civil Aviation Authority — My Gov NZ" };

export default async function CaaOverviewPage() {
  const data = await fetchCaaData(["caa:licences", "caa:aircraft"]);

  if (!data) {
    return (
      <main>
        <h1>Civil Aviation Authority</h1>
        <p>Unable to load your CAA information. Please grant access to continue.</p>
        <Link href={"/consent?dept=caa"}>Grant CAA access</Link>
      </main>
    );
  }

  return (
    <main>
      <h1>Civil Aviation Authority</h1>
      <p>CAA ID: ••••{data.caaId.slice(-4)}</p>

      <section>
        <h2>Licences</h2>
        {data.licences ? (
          <Link href={"/dept/caa/licences"}>View licences →</Link>
        ) : (
          <p>No licences on file.</p>
        )}
      </section>

      <section>
        <h2>Aircraft</h2>
        {data.aircraft ? (
          <Link href={"/dept/caa/aircraft"}>View aircraft →</Link>
        ) : (
          <p>No aircraft on file.</p>
        )}
      </section>

      <CaaAiPrompt />
    </main>
  );
}
